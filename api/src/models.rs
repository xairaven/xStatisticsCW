use serde::Deserialize;
use strum_macros::{Display, EnumString};

// Root response object
#[derive(Debug, Deserialize)]
pub struct Container {
    #[serde(rename = "queryresult")]
    pub result: QueryResult,
}

#[derive(Debug, Deserialize)]
pub struct QueryResult {
    pub success: bool,
    pub error: bool,
    #[serde(rename = "numpods")]
    pub pods_amount: u32,
    pub pods: Vec<Pod>,
    #[serde(rename = "timing")]
    pub timing_seconds: f64,
}

#[derive(Debug, Deserialize)]
pub struct Pod {
    pub title: String,
    pub id: String,
    pub scanner: String,
    pub error: bool,
    #[serde(rename = "numsubpods")]
    pub sub_pods_amount: u32,
    #[serde(rename = "subpods")]
    pub sub_pods: Vec<SubPod>,
}

// Sub-block containing the actual payload (text and images)
#[derive(Debug, Deserialize)]
pub struct SubPod {
    pub title: String,
    #[serde(rename = "plaintext")]
    pub plain_text: Option<String>,
    pub img: Option<Image>,
}

// Image metadata and source URL
#[derive(Debug, Deserialize)]
pub struct Image {
    pub src: String,
    pub alt: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "type")]
    pub image_type: String,
    #[serde(rename = "contenttype")]
    pub content_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
pub enum PodId {
    Input,
    Result,
    Substitution,
    Integral,
    DefiniteIntegral,
    IndefiniteIntegral,
    VisualRepresentationOfTheIntegral,
    RiemannSums,
    AlternateForm,
    ExpandedForm,
    PolynomialForm,
    Solution,
    Value,
    Root,
    Plot,
    RootPlot,
    NumberLine,
    ContourPlot,
}
