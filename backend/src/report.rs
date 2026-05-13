use crate::Input;
use std::collections::HashMap;

pub struct ReportMaker {
    input: Input,
    results: HashMap<String, String>,

    buffer: String,
}

impl ReportMaker {
    pub fn new(input: Input, results: HashMap<String, String>) -> Self {
        Self {
            input,
            results,
            buffer: String::new(),
        }
    }

    /// Renders the calculations into an HTML string with MathJax LaTeX formatting,
    /// completely replicating the flow of the original C# Render() method.
    pub fn generate_html_report(&mut self) -> String {
        let (a, b) = (self.input.a, self.input.b);

        // Base HTML setup with MathJax configuration
        self.buffer.push_str(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Calculations Report</title>
    <script>
        MathJax = {
            chtml: { displayAlign: 'left', displayIndent: '0em' },
            svg: { displayAlign: 'left', displayIndent: '0em' }
        };
    </script>
    <script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 20px auto; font-size: 18px; }
        .math-block { margin: 15px 0; overflow-x: auto; text-align: left; }
        h2 { margin-top: 30px; }
    </style>
</head>
<body>
"#);

        // Input render
        self.add_latex(&format!(r"\text{{Input: }} A = {}, B = {} \\", a, b));

        // Line equations formula
        self.add_latex(r"\text{Equation of a straight line: } \frac{x-x_0}{x_1-x_0}=\frac{y-y_0}{y_1-y_0}");

        // Line 2 title and result
        let query2 = format!("Line equation, points ({},0), (0,a)", a);
        self.add_query_image_fallback(&query2);
        self.add_latex(&self.get_res("Line2"));

        // Line 3 title and result
        let query3 = format!("Line equation, points (0, a), ({}, 0)", b);
        self.add_query_image_fallback(&query3);
        self.add_latex(&self.get_res("Line3"));

        // f(x) system (lines)
        self.add_latex(&format!(
            r"f(x) = \left\{{\matrix{{{}, & x \leq  {} \\ {}, & {} < x \leq  0 \\ {}, & 0 < x \leq  {} \\ {}, & x \geq  {}}}\right.",
            self.get_res("Line1"), a,
            self.get_res("Line2"), a,
            self.get_res("Line3"), b,
            self.get_res("Line4"), b
        ));

        // A TITLE
        self.add_title("Find a:");

        // Formula A title
        self.add_latex(r"\text{Formula:}");

        // Formula A
        self.add_latex(r"\int_{-\infty}^{\infty}f(x)dx = 1 \Rightarrow");

        // Formula A extended
        self.add_latex(&format!(
            r"\star \int_{{-\infty}}^{{{}}}{}dx + \int_{{{}}}^{{0}}({})dx + \int_{{0}}^{{{}}}({})dx + \int_{{{}}}^{{\infty}}{}dx = ... = 1? ",
            a, self.get_res("Line1"), a, self.get_res("Line2"), b, self.get_res("Line3"), b, self.get_res("Line4")
        ));

        // Integrals Title
        self.add_latex(r"\text{Each integral separately:}");

        // First integral
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{{}}}{}dx = {}",
            a,
            self.get_res("Line1"),
            self.get_res("A1Integral")
        ));

        // Second integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{0}}({})dx = {}",
            a,
            self.get_res("Line2"),
            self.get_res("A2Integral")
        ));

        // Third integral
        self.add_latex(&format!(
            r"\bullet \int_{{0}}^{{{}}}({})dx = {}",
            b,
            self.get_res("Line3"),
            self.get_res("A3Integral")
        ));

        // Fourth integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{\infty}}{}dx = {}",
            b,
            self.get_res("Line4"),
            self.get_res("A4Integral")
        ));

        // Sum Title
        self.add_latex(r"\star \text{Sum:}");

        // Sum
        self.add_latex(&format!(
            r"{} + {} + {} + {} = {} \Rightarrow {} = 1",
            self.get_res("A1Integral"),
            self.get_res("A2Integral"),
            self.get_res("A3Integral"),
            self.get_res("A4Integral"),
            self.get_res("ASum"),
            self.get_res("ASum")
        ));

        // A
        self.add_latex(&format!(r"a = {}", self.get_res("A")));

        // F(X) TITLE
        self.add_title("Find F(x):");

        // Formula F(x) title
        self.add_latex(r"\text{Formula:}");

        // Formula F(x)
        self.add_latex(r"F(x) = \int_{-\infty}^{x}f(x)dx");

        // Number Line Title
        self.add_latex(r"\text{Number line on 4 intervals:}");

        // f(x) system (lines)
        self.add_latex(&format!(
            r"f(x) = \left\{{\matrix{{{}, & x \leq  {} \\ {}, & {} < x \leq  0 \\ {}, & 0 < x \leq  {} \\ {}, & x >  {}}}\right.",
            self.get_res("Line1"), a,
            self.get_res("Line2"), a,
            self.get_res("Line3"), b,
            self.get_res("Line4"), b
        ));

        // Number Line
        let query_line = format!("Line {}, 0, {}", a, b);
        self.add_query_image_fallback(&query_line);

        // First Interval
        self.add_latex(&format!(r"1) -\infty; {}", a));
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{x}}{}dx = {}",
            self.get_res("Line1"),
            self.get_res("Fx1Integral")
        ));

        // Second Interval
        self.add_latex(&format!(r"2) {}; 0", a));
        self.add_latex(&format!(r"\star \int_{{-\infty}}^{{{}}}{}dx + \int_{{{}}}^{{x}}({})dx = ... \Rightarrow", a, self.get_res("Line1"), a, self.get_res("Line2")));

        // Second 1 integral
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{{}}}{}dx = {}",
            a,
            self.get_res("Line1"),
            self.get_res("A1Integral")
        ));

        // Second 2 integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{x}}({})dx = {}",
            a,
            self.get_res("Line2"),
            self.get_res("Fx2Integral")
        ));

        // Second sum
        self.add_latex(&format!(
            r"\star {} + {} = {} \Rightarrow a = {} \Rightarrow {}",
            self.get_res("A1Integral"),
            self.get_res("Fx2Integral"),
            self.get_res("Fx2RawSum"),
            self.get_res("A"),
            self.get_res("Fx2Sum")
        ));

        // Third Interval
        self.add_latex(&format!(r"3) 0; {}", b));
        self.add_latex(&format!(r"\star \int_{{-\infty}}^{{{}}}{}dx + \int_{{{}}}^{{0}}({})dx + \int_{{0}}^{{x}}({})dx = ... \Rightarrow", a, self.get_res("Line1"), a, self.get_res("Line2"), self.get_res("Line3")));

        // Third 1 integral
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{{}}}{}dx = {}",
            a,
            self.get_res("Line1"),
            self.get_res("A1Integral")
        ));

        // Third 2 integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{0}}({})dx = {}",
            a,
            self.get_res("Line2"),
            self.get_res("A2Integral")
        ));

        // Third 3 integral
        self.add_latex(&format!(
            r"\bullet \int_{{0}}^{{x}}({})dx = {}",
            self.get_res("Line3"),
            self.get_res("Fx3Integral")
        ));

        // Third sum
        self.add_latex(&format!(
            r"\star {} + {} + {} = {} \Rightarrow a = {} \Rightarrow {}",
            self.get_res("A1Integral"),
            self.get_res("A2Integral"),
            self.get_res("Fx3Integral"),
            self.get_res("Fx3RawSum"),
            self.get_res("A"),
            self.get_res("Fx3Sum")
        ));

        // Fourth Interval
        self.add_latex(&format!(r"4) {}; \infty", b));
        self.add_latex(&format!(
            r"\star \int_{{-\infty}}^{{{}}}{}dx + \int_{{{}}}^{{0}}({})dx + \int_{{0}}^{{{}}}({})dx + \int_{{{}}}^{{x}}{}dx = ... \Rightarrow",
            a, self.get_res("Line1"), a, self.get_res("Line2"), b, self.get_res("Line3"), b, self.get_res("Line4")
        ));

        // Fourth 1 integral
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{{}}}{}dx = {}",
            a,
            self.get_res("Line1"),
            self.get_res("A1Integral")
        ));

        // Fourth 2 integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{0}}({})dx = {}",
            a,
            self.get_res("Line2"),
            self.get_res("A2Integral")
        ));

        // Fourth 3 integral
        self.add_latex(&format!(
            r"\bullet \int_{{0}}^{{{}}}({})dx = {}",
            b,
            self.get_res("Line3"),
            self.get_res("A3Integral")
        ));

        // Fourth 4 integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{x}}{}dx = {}",
            b,
            self.get_res("Line4"),
            self.get_res("Fx4Integral")
        ));

        // Fourth sum
        self.add_latex(&format!(
            r"\star {} + {} + {} + {} = {} \Rightarrow a = {} \Rightarrow {}",
            self.get_res("A1Integral"),
            self.get_res("A2Integral"),
            self.get_res("A3Integral"),
            self.get_res("Fx4Integral"),
            self.get_res("Fx4RawSum"),
            self.get_res("A"),
            self.get_res("Fx4Sum")
        ));

        // F(x) system (integrals)
        self.add_latex(&format!(
            r"F(x) = \left\{{\matrix{{{}, & x \leq  {} \\ {}, & {} < x \leq  0 \\ {}, & 0 < x \leq  {} \\ {}, & x >  {}}}\right.",
            self.get_res("Fx1Sum"), a,
            self.get_res("Fx2Sum"), a,
            self.get_res("Fx3Sum"), b,
            self.get_res("Fx4Sum"), b
        ));

        // Intersection with the y-axis Title
        self.add_latex(r"\text{Intersection with the y-axis: }");

        // Intersection with the y-axis, second integral
        let query_y2 = format!("{}, where x = 0", self.get_res("Fx2Sum"));
        self.add_query_image_fallback(&query_y2);

        // Intersection with the y-axis third integral
        let query_y3 = format!("{}, where x = 0", self.get_res("Fx3Sum"));
        self.add_query_image_fallback(&query_y3);

        // Graphic :(
        self.add_latex(r"\text{Plot. Unfortunately, you have to draw it yourself :(}");

        // M(X) TITLE
        self.add_title("Find M(x):");

        // Formula M(x) title
        self.add_latex(r"\text{Formula:}");

        // Formula M(x)
        self.add_latex(r"M(x) = \int_{-\infty}^{\infty}x f(x)dx = ... \Rightarrow");

        // Formula M(x) extended
        self.add_latex(&format!(
            r"\star \int_{{-\infty}}^{{{}}}{}xdx + \int_{{{}}}^{{0}}({})xdx + \int_{{0}}^{{{}}}({})xdx + \int_{{{}}}^{{\infty}}{}xdx = ... ",
            a, self.get_res("Line1"), a, self.get_res("Line2"), b,self. get_res("Line3"), b, self.get_res("Line4")
        ));

        // Integrals Title
        self.add_latex(r"\text{Each integral separately:}");

        // First integral
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{{}}}{}xdx = {}",
            a,
            self.get_res("Line1"),
            self.get_res("Mx1Integral")
        ));

        // Second integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{0}}({})xdx = {}",
            a,
            self.get_res("Line2"),
            self.get_res("Mx2Integral")
        ));

        // Third integral
        self.add_latex(&format!(
            r"\bullet \int_{{0}}^{{{}}}({})xdx = {}",
            b,
            self.get_res("Line3"),
            self.get_res("Mx3Integral")
        ));

        // Fourth integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{\infty}}{}xdx = {}",
            b,
            self.get_res("Line4"),
            self.get_res("Mx4Integral")
        ));

        // Sum Title
        self.add_latex(r"\star \text{Sum:}");

        // Sum
        self.add_latex(&format!(
            r"{} + {} + {} + {} = {} \Rightarrow a = {} \Rightarrow {}",
            self.get_res("Mx1Integral"),
            self.get_res("Mx2Integral"),
            self.get_res("Mx3Integral"),
            self.get_res("Mx4Integral"),
            self.get_res("MxRawSum"),
            self.get_res("A"),
            self.get_res("MxSum")
        ));

        // M(x)
        self.add_latex(&format!(
            r"M(x) = m = {} = {}",
            self.get_res("MxSum"),
            self.get_res("MxFloat")
        ));

        // D(X) TITLE
        self.add_title("Find D(x):");

        // Formula D(x) title
        self.add_latex(r"\text{Formula:}");

        // Formula D(x)
        self.add_latex(r"D(x) = M(x^2)-[M(x)]^2 = d - m^2 = ... ");

        // M(x^2) title
        self.add_latex(r"M(x^2) = d = ... \Rightarrow");

        // Formula M(x^2) extended
        self.add_latex(&format!(
            r"\star \int_{{-\infty}}^{{{}}}{}x^2dx + \int_{{{}}}^{{0}}x^2({})dx + \int_{{0}}^{{{}}}x^2({})dx + \int_{{{}}}^{{\infty}}{}x^2dx = ... ",
            a, self.get_res("Line1"), a, self.get_res("Line2"), b, self.get_res("Line3"), b, self.get_res("Line4")
        ));

        // Integrals Title
        self.add_latex(r"\text{Each integral separately:}");

        // First integral
        self.add_latex(&format!(
            r"\bullet \int_{{-\infty}}^{{{}}}{}x^2dx = {}",
            a,
            self.get_res("Line1"),
            self.get_res("Mx2Integral1")
        ));

        // Second integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{0}}({})x^2dx = {}",
            a,
            self.get_res("Line2"),
            self.get_res("Mx2Integral2")
        ));

        // Third integral
        self.add_latex(&format!(
            r"\bullet \int_{{0}}^{{{}}}({})x^2dx = {}",
            b,
            self.get_res("Line3"),
            self.get_res("Mx2Integral3")
        ));

        // Fourth integral
        self.add_latex(&format!(
            r"\bullet \int_{{{}}}^{{\infty}}{}x^2dx = {}",
            b,
            self.get_res("Line4"),
            self.get_res("Mx2Integral4")
        ));

        // Sum Title
        self.add_latex(r"\star \text{Sum:}");

        // Sum
        self.add_latex(&format!(
            r"{} + {} + {} + {} = {} \Rightarrow a = {} \Rightarrow {}",
            self.get_res("Mx2Integral1"),
            self.get_res("Mx2Integral2"),
            self.get_res("Mx2Integral3"),
            self.get_res("Mx2Integral4"),
            self.get_res("Mx2RawSum"),
            self.get_res("A"),
            self.get_res("Mx2Sum")
        ));

        // M(x^2)
        self.add_latex(&format!(r"M(x^2) = d = {}", self.get_res("Mx2Sum")));

        // M(x)^2
        self.add_latex(&format!(
            r"M(x)^2 = m^2 = ({})^2 = {}",
            self.get_res("MxSum"),
            self.get_res("m2Sum")
        ));

        // Formula D(x)
        self.add_latex(&format!(
            r"D(x) = M(x^2)-[M(x)]^2 = d - m^2 = {} - {} = {}",
            self.get_res("Mx2Sum"),
            self.get_res("m2Sum"),
            self.get_res("Dx")
        ));

        // D(x)
        self.add_latex(&format!(
            r"D(x) = {} = {}",
            self.get_res("Dx"),
            self.get_res("DxFloat")
        ));

        // G(X) TITLE
        self.add_title("Find G(x):");

        // Formula G(x) title
        self.add_latex(r"\text{Formula:}");

        // Formula G(x)
        self.add_latex(&format!(
            r"\sigma(x) = \sqrt{{D(x)}} = \sqrt{{{}}} = {}",
            self.get_res("Dx"),
            self.get_res("G")
        ));

        // G(x)
        self.add_latex(&format!(r"\sigma(x) = {}", self.get_res("G")));

        self.buffer.push_str("</body>\n</html>");

        self.buffer.to_string()
    }

    /// Replace of the C# images.Add(...) logic
    fn add_latex(&mut self, latex: &str) {
        self.buffer.push_str(&format!(
            "<div class=\"math-block\">\\[ {} \\]</div>\n",
            latex
        ));
    }

    /// Corresponds to BitmapService.ByText(text, 24, FontStyle.Underline)
    fn add_title(&mut self, text: &str) {
        self.buffer.push_str(&format!("<h2><u>{}</u></h2>\n", text));
    }

    /// Corresponds to BitmapService.ByGenboxImage(_solver.Image(query))
    /// Since we are generating HTML directly, we output the query text as math text
    fn add_query_image_fallback(&mut self, query: &str) {
        self.buffer.push_str(&format!(
            "<div class=\"math-block\">\\[ \\text{{{}}} \\]</div>\n",
            query
        ));
    }

    /// Helper to safely get results from the map
    fn get_res(&self, key: &str) -> String {
        self.results
            .get(key)
            .cloned()
            .unwrap_or_else(|| "?".to_string())
    }
}
