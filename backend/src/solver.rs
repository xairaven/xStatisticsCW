use crate::journal::Journaler;
use crate::report::ReportMaker;
use crate::{BackendError, Input};
use api::WolframClient;
use api::models::PodId;
use crossbeam::channel::Sender;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Debug)]
pub struct Solver {
    client: Arc<WolframClient>,
    journaler: Journaler,
    semaphore: Arc<Semaphore>,
}

impl Solver {
    pub fn new(app_id: String, log_tx: Sender<String>, max_concurrent: usize) -> Self {
        let client = WolframClient::new(app_id);

        Self {
            client: Arc::new(client),
            journaler: Journaler::new(log_tx),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn run(&self, input: Input) -> Result<String, BackendError> {
        let map = self.solve(input.clone()).await?;
        let source_code = ReportMaker::new(input, map).generate_html_report();
        log::info!("Generated report source code.");
        Ok(source_code)
    }

    pub async fn solve(
        &self, input: Input,
    ) -> Result<HashMap<String, String>, BackendError> {
        let mut results = HashMap::new();

        results.insert("Line1".to_string(), "0".to_string());
        results.insert("Line4".to_string(), "0".to_string());

        // FINDING LINES
        let line2 = self
            .query_solver(
                format!("Line equation, points ({},0), (0,a)", input.a),
                PodId::Result,
            )
            .await?;
        let line3 = self
            .query_solver(
                format!("Line equation, points (0, a), ({}, 0)", input.b),
                PodId::Result,
            )
            .await?;
        results.insert("Line2".to_string(), line2);
        results.insert("Line3".to_string(), line3);

        // FINDING A INTEGRALS
        let a1 = self
            .query_solver(
                format!(
                    "Integrate({}) from x=-inf to x={}",
                    results["Line1"], input.a
                ),
                PodId::Result,
            )
            .await?;
        let a2 = self
            .query_solver(
                format!("Integrate({}) from x={} to x=0", results["Line2"], input.a),
                PodId::Result,
            )
            .await?;
        let a3 = self
            .query_solver(
                format!("Integrate({}) from x=0 to x={}", results["Line3"], input.b),
                PodId::Result,
            )
            .await?;
        let a4 = self
            .query_solver(
                format!(
                    "Integrate({}) from x={} to x=inf",
                    results["Line4"], input.b
                ),
                PodId::Result,
            )
            .await?;

        results.insert("A1Integral".to_string(), a1);
        results.insert("A2Integral".to_string(), a2);
        results.insert("A3Integral".to_string(), a3);
        results.insert("A4Integral".to_string(), a4);

        // ASum and A
        let a_sum = format!(
            "{} + {} + {} + {}",
            results["A1Integral"],
            results["A2Integral"],
            results["A3Integral"],
            results["A4Integral"]
        );
        results.insert(
            "ASum".to_string(),
            self.query_solver(a_sum, PodId::Result).await?,
        );

        let a_val = self
            .query_solver(format!("Solve {} = 1", results["ASum"]), PodId::Result)
            .await?;
        results.insert("A".to_string(), a_val);

        // F(X) INTERVALS
        let fx1 = self
            .query_solver(
                format!("Integrate({}) from x=-inf to x=x", results["Line1"]),
                PodId::Result,
            )
            .await?;
        let fx2 = self
            .query_solver(
                format!("Integrate({}) from x={} to x=x", results["Line2"], input.a),
                PodId::Result,
            )
            .await?;
        let fx3 = self
            .query_solver(
                format!("Integrate({}) from x=0 to x=x", results["Line3"]),
                PodId::Result,
            )
            .await?;
        let fx4 = self
            .query_solver(
                format!("Integrate({}) from x={} to x=x", results["Line4"], input.b),
                PodId::Result,
            )
            .await?;

        results.insert("Fx1Integral".to_string(), fx1.clone());
        results.insert("Fx1Sum".to_string(), fx1);

        // Expanded forms and sums (Using robust fallbacks for unneeded substitutions)
        let fx2_exp = self
            .query_solver(fx2.clone(), PodId::ExpandedForm)
            .await
            .unwrap_or(fx2);
        results.insert("Fx2Integral".to_string(), fx2_exp);
        let fx2_raw = self
            .query_solver(
                format!("{} + {}", results["A1Integral"], results["Fx2Integral"]),
                PodId::Result,
            )
            .await?;
        results.insert("Fx2RawSum".to_string(), fx2_raw);
        let fx2_sum = self
            .query_solver(
                format!("{} where a = {}", results["Fx2RawSum"], results["A"]),
                PodId::Result,
            )
            .await
            .unwrap_or_else(|_| results["Fx2RawSum"].clone());
        results.insert(
            "Fx2Sum".to_string(),
            self.query_solver(fx2_sum.clone(), PodId::ExpandedForm)
                .await
                .unwrap_or(fx2_sum),
        );

        let fx3_exp = self
            .query_solver(fx3.clone(), PodId::ExpandedForm)
            .await
            .unwrap_or(fx3);
        results.insert("Fx3Integral".to_string(), fx3_exp);
        let fx3_raw = self
            .query_solver(
                format!(
                    "{} + {} + {}",
                    results["A1Integral"], results["A2Integral"], results["Fx3Integral"]
                ),
                PodId::Result,
            )
            .await?;
        results.insert("Fx3RawSum".to_string(), fx3_raw);
        let fx3_sum = self
            .query_solver(
                format!("{} where a = {}", results["Fx3RawSum"], results["A"]),
                PodId::Result,
            )
            .await
            .unwrap_or_else(|_| results["Fx3RawSum"].clone());
        results.insert(
            "Fx3Sum".to_string(),
            self.query_solver(fx3_sum.clone(), PodId::ExpandedForm)
                .await
                .unwrap_or(fx3_sum),
        );

        let fx4_exp = self
            .query_solver(fx4.clone(), PodId::ExpandedForm)
            .await
            .unwrap_or(fx4);
        results.insert("Fx4Integral".to_string(), fx4_exp);
        let fx4_raw = self
            .query_solver(
                format!(
                    "{} + {} + {} + {}",
                    results["A1Integral"],
                    results["A2Integral"],
                    results["A3Integral"],
                    results["Fx4Integral"]
                ),
                PodId::Result,
            )
            .await?;
        results.insert("Fx4RawSum".to_string(), fx4_raw);
        let fx4_sum = self
            .query_solver(
                format!("{} where a = {}", results["Fx4RawSum"], results["A"]),
                PodId::Result,
            )
            .await
            .unwrap_or_else(|_| results["Fx4RawSum"].clone());
        results.insert(
            "Fx4Sum".to_string(),
            self.query_solver(fx4_sum.clone(), PodId::ExpandedForm)
                .await
                .unwrap_or(fx4_sum),
        );

        // M(x) AND D(x) INTEGRALS
        let mx1 = self
            .query_solver(
                format!(
                    "Integrate({} x) from x=-inf to x={}",
                    results["Line1"], input.a
                ),
                PodId::Input,
            )
            .await?;
        let mx2 = self
            .query_solver(
                format!(
                    "Integrate(({}) x) from x={} to x=0",
                    results["Line2"], input.a
                ),
                PodId::Input,
            )
            .await?;
        let mx3 = self
            .query_solver(
                format!(
                    "Integrate(({}) x) from x=0 to x={}",
                    results["Line3"], input.b
                ),
                PodId::Input,
            )
            .await?;
        let mx4 = self
            .query_solver(
                format!(
                    "Integrate({} x) from x={} to x=inf",
                    results["Line4"], input.b
                ),
                PodId::Input,
            )
            .await?;

        let dx1 = self
            .query_solver(
                format!(
                    "Integrate({} x^2) from x=-inf to x={}",
                    results["Line1"], input.a
                ),
                PodId::Input,
            )
            .await?;
        let dx2 = self
            .query_solver(
                format!(
                    "Integrate(({}) x^2) from x={} to x=0",
                    results["Line2"], input.a
                ),
                PodId::Input,
            )
            .await?;
        let dx3 = self
            .query_solver(
                format!(
                    "Integrate(({}) x^2) from x=0 to x={}",
                    results["Line3"], input.b
                ),
                PodId::Input,
            )
            .await?;
        let dx4 = self
            .query_solver(
                format!(
                    "Integrate({} x^2) from x={} to x=inf",
                    results["Line4"], input.b
                ),
                PodId::Input,
            )
            .await?;

        results.insert("Mx1Integral".to_string(), mx1);
        results.insert("Mx2Integral".to_string(), mx2);
        results.insert("Mx3Integral".to_string(), mx3);
        results.insert("Mx4Integral".to_string(), mx4);

        results.insert("Mx2Integral1".to_string(), dx1);
        results.insert("Mx2Integral2".to_string(), dx2);
        results.insert("Mx2Integral3".to_string(), dx3);
        results.insert("Mx2Integral4".to_string(), dx4);

        // M(X) RESULTS (Using robust fallbacks for unneeded substitutions)
        let mx_raw_sum = format!(
            "{} + {} + {} + {}",
            results["Mx1Integral"],
            results["Mx2Integral"],
            results["Mx3Integral"],
            results["Mx4Integral"]
        );
        results.insert(
            "MxRawSum".to_string(),
            self.query_solver(mx_raw_sum, PodId::Result).await?,
        );
        let mx_sum = self
            .query_solver(
                format!("{} where a = {}", results["MxRawSum"], results["A"]),
                PodId::Result,
            )
            .await
            .unwrap_or_else(|_| results["MxRawSum"].clone());
        results.insert("MxSum".to_string(), mx_sum);
        results.insert(
            "MxFloat".to_string(),
            self.query_solver(format!("N[{}, 10]", results["MxSum"]), PodId::Result)
                .await?,
        );

        // D(X) RESULTS (Using robust fallbacks for unneeded substitutions)
        let mx2_raw_sum = format!(
            "{} + {} + {} + {}",
            results["Mx2Integral1"],
            results["Mx2Integral2"],
            results["Mx2Integral3"],
            results["Mx2Integral4"]
        );
        results.insert(
            "Mx2RawSum".to_string(),
            self.query_solver(mx2_raw_sum, PodId::Result).await?,
        );
        let mx2_sum = self
            .query_solver(
                format!("{} where a = {}", results["Mx2RawSum"], results["A"]),
                PodId::Result,
            )
            .await
            .unwrap_or_else(|_| results["Mx2RawSum"].clone());
        results.insert("Mx2Sum".to_string(), mx2_sum);
        results.insert(
            "m2Sum".to_string(),
            self.query_solver(format!("({})^2", results["MxSum"]), PodId::Result)
                .await?,
        );

        let dx_val = self
            .query_solver(
                format!("{} - {}", results["Mx2Sum"], results["m2Sum"]),
                PodId::Result,
            )
            .await?;
        results.insert("Dx".to_string(), dx_val.clone());
        results.insert(
            "DxFloat".to_string(),
            self.query_solver(format!("N[{}, 10]", dx_val), PodId::Result)
                .await?,
        );

        // G(X)
        results.insert(
            "G".to_string(),
            self.query_solver(format!("sqrt({}) 10 digits", dx_val), PodId::Result)
                .await?,
        );

        Ok(results)
    }

    /// Helper method that controls query (semaphore), journals query
    /// and calls client to get text
    async fn query_solver(
        &self, query: String, pod_id: PodId,
    ) -> Result<String, BackendError> {
        // Waiting for semaphore permit to avoid spamming the API
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(BackendError::Semaphore)?;

        self.journaler.log(format!("Requesting: {}", query));
        log::info!("Requesting: {}", query);

        let raw_text = self.client.plain_text(&query, pod_id).await?;
        let clean_text = self.client.operand_from_result(&raw_text);

        Ok(clean_text)
    }
}
