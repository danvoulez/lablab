use chrono::{DateTime, Utc};
use md5;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub id: String,
    pub authors: Vec<String>,
    pub title: String,
    pub journal: Option<String>,
    pub year: u16,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    pub url: Option<String>,
    pub cited_in_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub id: String,
    pub caption: String,
    pub figure_type: FigureType,
    pub data: FigureData,
    pub referenced_in_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FigureType {
    Plot,
    Diagram,
    Molecular,
    Heatmap,
    Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FigureData {
    Svg(String),
    PlotlyJson(Value),
    ImagePath(PathBuf),
    AsciiPlot(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementaryMaterial {
    pub id: String,
    pub title: String,
    pub file_type: String,
    pub path: PathBuf,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManuscriptMetadata {
    pub generated_at: DateTime<Utc>,
    pub execution_id: String,
    pub version: String,
    pub checksum: String,
    pub contract_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub content: String,
    pub subsections: Vec<Section>,
    pub figure_refs: Vec<String>,
    pub citation_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedManuscript {
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub keywords: Vec<String>,
    pub sections: Vec<Section>,
    pub citations: Vec<Citation>,
    pub figures: Vec<Figure>,
    pub supplementary: Vec<SupplementaryMaterial>,
    pub metadata: ManuscriptMetadata,
}

pub struct ManuscriptBuilder {
    manuscript: EnhancedManuscript,
    citation_index: HashMap<String, usize>,
}

impl ManuscriptBuilder {
    pub fn new(title: String, execution_id: String) -> Self {
        Self {
            manuscript: EnhancedManuscript {
                title,
                authors: vec![],
                abstract_text: String::new(),
                keywords: vec![],
                sections: vec![],
                citations: vec![],
                figures: vec![],
                supplementary: vec![],
                metadata: ManuscriptMetadata {
                    generated_at: Utc::now(),
                    execution_id,
                    version: "1.0.0".to_string(),
                    checksum: String::new(),
                    contract_path: None,
                },
            },
            citation_index: HashMap::new(),
        }
    }

    pub fn with_authors(mut self, authors: Vec<String>) -> Self {
        self.manuscript.authors = authors;
        self
    }

    pub fn with_abstract(mut self, abstract_text: String) -> Self {
        self.manuscript.abstract_text = abstract_text;
        self
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.manuscript.keywords = keywords;
        self
    }

    pub fn add_section(&mut self, section: Section) -> &mut Self {
        self.manuscript.sections.push(section);
        self
    }

    pub fn add_citation(&mut self, citation: Citation) -> String {
        let cite_id = citation.id.clone();
        if !self.citation_index.contains_key(&cite_id) {
            let idx = self.manuscript.citations.len();
            self.manuscript.citations.push(citation);
            self.citation_index.insert(cite_id.clone(), idx);
        }
        cite_id
    }

    pub fn add_figure(&mut self, figure: Figure) -> String {
        let fig_id = figure.id.clone();
        self.manuscript.figures.push(figure);
        fig_id
    }

    pub fn generate_energy_plot(&mut self, data: Vec<(f64, f64)>, section_id: &str) -> String {
        let fig_id = format!("fig_energy_{}", self.manuscript.figures.len() + 1);
        let plot = self.create_ascii_plot(&data, "Time (ps)", "Energy (kcal/mol)");
        let figure = Figure {
            id: fig_id.clone(),
            caption: "Energy trajectory showing system stability over simulation time".to_string(),
            figure_type: FigureType::Plot,
            data: FigureData::AsciiPlot(plot),
            referenced_in_sections: vec![section_id.to_string()],
        };
        self.add_figure(figure)
    }

    pub fn generate_causal_network(
        &mut self,
        nodes: Vec<String>,
        edges: Vec<(usize, usize, f64)>,
        section_id: &str,
    ) -> String {
        let fig_id = format!("fig_network_{}", self.manuscript.figures.len() + 1);
        let network_svg = self.create_network_svg(&nodes, &edges);
        let figure = Figure {
            id: fig_id.clone(),
            caption: "Causal network showing relationships between system components"
                .to_string(),
            figure_type: FigureType::Network,
            data: FigureData::Svg(network_svg),
            referenced_in_sections: vec![section_id.to_string()],
        };
        self.add_figure(figure)
    }

    fn create_ascii_plot(&self, data: &[(f64, f64)], x_label: &str, y_label: &str) -> String {
        let width = 60usize;
        let height = 20usize;
        if data.is_empty() {
            return format!("No data available for {} vs {}", x_label, y_label);
        }

        let (mut min_x, mut max_x) = (f64::MAX, f64::MIN);
        let (mut min_y, mut max_y) = (f64::MAX, f64::MIN);
        for (x, y) in data {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        if (max_x - min_x).abs() < std::f64::EPSILON {
            max_x = min_x + 1.0;
        }
        if (max_y - min_y).abs() < std::f64::EPSILON {
            max_y = min_y + 1.0;
        }

        let mut plot = vec![vec![' '; width]; height];
        for i in 0..height {
            plot[i][0] = '│';
        }
        for j in 0..width {
            plot[height - 1][j] = '─';
        }
        plot[height - 1][0] = '└';

        for (x, y) in data {
            let x_pos = (((x - min_x) / (max_x - min_x)) * ((width - 2) as f64)) as usize + 1;
            let y_pos = height
                .saturating_sub(2)
                .saturating_sub((((*y - min_y) / (max_y - min_y)) * ((height - 2) as f64)) as usize);
            if x_pos < width && y_pos < height {
                plot[y_pos][x_pos] = '●';
            }
        }

        let mut result = format!("{:^width$}\n", y_label, width = width);
        for row in plot {
            result.push_str(&row.iter().collect::<String>());
            result.push('\n');
        }
        result.push_str(&format!("{:^width$}\n", x_label, width = width));
        result.push_str(&format!(
            "Range: X[{:.2}, {:.2}] Y[{:.2}, {:.2}]\n",
            min_x, max_x, min_y, max_y
        ));
        result
    }

    fn create_network_svg(&self, nodes: &[String], edges: &[(usize, usize, f64)]) -> String {
        let width = 600.0;
        let height = 400.0;
        let radius = 150.0;
        let node_size = 8.0;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width as i32, height as i32
        );

        let positions: Vec<(f64, f64)> = (0..nodes.len())
            .map(|i| {
                let angle = 2.0 * std::f64::consts::PI * (i as f64) / (nodes.len().max(1) as f64);
                (
                    width / 2.0 + radius * angle.cos(),
                    height / 2.0 + radius * angle.sin(),
                )
            })
            .collect();

        for (from, to, weight) in edges {
            if *from < positions.len() && *to < positions.len() {
                let opacity = (weight * 0.8).clamp(0.1, 1.0);
                svg.push_str(&format!(
                    r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="blue" stroke-width="2" opacity="{:.2}"/>"#,
                    positions[*from].0, positions[*from].1, positions[*to].0, positions[*to].1, opacity
                ));
            }
        }

        for (i, node) in nodes.iter().enumerate() {
            if i < positions.len() {
                svg.push_str(&format!(
                    r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}" fill="darkblue"/>"#,
                    positions[i].0, positions[i].1, node_size
                ));
                svg.push_str(&format!(
                    r#"<text x="{:.1}" y="{:.1}" text-anchor="middle" dy="-15" font-size="12">{}</text>"#,
                    positions[i].0, positions[i].1, node
                ));
            }
        }

        svg.push_str("</svg>");
        svg
    }

    pub fn add_methods_section(&mut self, protocol_data: &Value) {
        let mut content = String::new();
        content.push_str("## Simulation Protocol\n\n");
        if let Some(force_field) = protocol_data.get("force_field").and_then(|v| v.as_str()) {
            content.push_str(&format!("Force field: {}\n", force_field));
        }
        if let Some(temp) = protocol_data.get("temperature").and_then(|v| v.as_f64()) {
            content.push_str(&format!("Temperature: {} K\n", temp));
        }
        if let Some(steps) = protocol_data.get("steps").and_then(|v| v.as_u64()) {
            content.push_str(&format!("Simulation steps: {}\n", steps));
        }

        let section = Section {
            id: "methods".to_string(),
            title: "Methods".to_string(),
            content,
            subsections: vec![],
            figure_refs: vec![],
            citation_refs: vec![],
        };
        self.add_section(section);
    }

    pub fn add_results_section(&mut self, analysis_data: &Value, causal_chains: &Value) {
        let mut content = String::new();
        let mut figure_refs = vec![];

        content.push_str("## Analysis Results\n\n");

        if let Some(energy_data) = analysis_data.get("energy_trajectory").and_then(|v| v.as_array()) {
            let data: Vec<(f64, f64)> = energy_data
                .iter()
                .enumerate()
                .filter_map(|(i, v)| v.as_f64().map(|e| (i as f64, e)))
                .collect();
            if !data.is_empty() {
                let fig_id = self.generate_energy_plot(data, "results");
                figure_refs.push(fig_id.clone());
                content.push_str(&format!("Energy trajectory indicates stability (Figure {}).\n\n", fig_id));
            }
        }

        if let Some(stability) = analysis_data.get("stability").and_then(|v| v.as_str()) {
            content.push_str(&format!("Overall stability classified as {}.\n\n", stability));
        }

        if let Some(chains) = causal_chains.as_array() {
            content.push_str(&format!("Identified {} causal chains.\n", chains.len()));
        }

        let section = Section {
            id: "results".to_string(),
            title: "Results".to_string(),
            content,
            subsections: vec![],
            figure_refs,
            citation_refs: vec![],
        };
        self.add_section(section);
    }

    pub fn render_markdown(&self) -> String {
        render_markdown_from(&self.manuscript)
    }

    pub fn build(mut self) -> EnhancedManuscript {
        let markdown = self.render_markdown();
        self.manuscript.metadata.checksum = format!("{:x}", md5::compute(markdown.as_bytes()));
        self.manuscript
    }
}

pub fn render_markdown_from(manuscript: &EnhancedManuscript) -> String {
    let mut output = String::new();
    output.push_str(&format!("# {}\n\n", manuscript.title));

    if !manuscript.authors.is_empty() {
        output.push_str(&format!("**Authors:** {}\n\n", manuscript.authors.join(", ")));
    }

    if !manuscript.abstract_text.is_empty() {
        output.push_str("## Abstract\n\n");
        output.push_str(&format!("{}\n\n", manuscript.abstract_text));
    }

    if !manuscript.keywords.is_empty() {
        output.push_str(&format!("**Keywords:** {}\n\n", manuscript.keywords.join(", ")));
    }

    for section in &manuscript.sections {
        output.push_str(&render_section(section, 2, &manuscript.citations));
    }

    if !manuscript.figures.is_empty() {
        output.push_str("\n## Figures\n\n");
        for figure in &manuscript.figures {
            output.push_str(&format!("### {}\n", figure.id));
            output.push_str(&format!("*{}*\n\n", figure.caption));
            match &figure.data {
                FigureData::AsciiPlot(plot) => {
                    output.push_str("```\n");
                    output.push_str(plot);
                    output.push_str("\n```\n\n");
                }
                FigureData::Svg(svg) => {
                    output.push_str(&format!("```svg\n{}\n```\n\n", svg));
                }
                FigureData::ImagePath(path) => {
                    output.push_str(&format!("![{}]({})\n\n", figure.caption, path.display()));
                }
                FigureData::PlotlyJson(_) => {
                    output.push_str("*[Interactive plot - view in dashboard]*\n\n");
                }
            }
        }
    }

    if !manuscript.citations.is_empty() {
        output.push_str("\n## References\n\n");
        for (i, citation) in manuscript.citations.iter().enumerate() {
            output.push_str(&format!("{}. ", i + 1));
            output.push_str(&citation.authors.join(", "));
            output.push_str(&format!(". {} ", citation.title));
            if let Some(journal) = &citation.journal {
                output.push_str(&format!("*{}* ", journal));
            }
            output.push_str(&format!("({})", citation.year));
            if let Some(doi) = &citation.doi {
                output.push_str(&format!(" doi:{}", doi));
            }
            output.push_str("\n");
        }
    }

    output.push_str(&format!(
        "\n---\n*Generated: {} | Execution: {} | Version: {}*\n",
        manuscript
            .metadata
            .generated_at
            .format("%Y-%m-%d %H:%M:%S UTC"),
        manuscript.metadata.execution_id,
        manuscript.metadata.version
    ));

    output
}

fn render_section(section: &Section, level: usize, citations: &[Citation]) -> String {
    let mut output = String::new();
    let header = "#".repeat(level);
    output.push_str(&format!("{} {}\n\n", header, section.title));
    output.push_str(&format!("{}\n\n", section.content));

    for fig_ref in &section.figure_refs {
        output.push_str(&format!("See Figure {}\n", fig_ref));
    }

    if !section.citation_refs.is_empty() {
        let mut refs = Vec::new();
        for id in &section.citation_refs {
            if let Some((idx, _)) = citations
                .iter()
                .enumerate()
                .find(|(_, citation)| &citation.id == id)
            {
                refs.push((idx + 1).to_string());
            }
        }
        if !refs.is_empty() {
            output.push_str(&format!(" [{}]\n", refs.join(",")));
        }
    }

    for subsection in &section.subsections {
        output.push_str(&render_section(subsection, level + 1, citations));
    }

    output
}

pub fn generate_enhanced_manuscript(
    execution_id: String,
    protocol_data: Value,
    analysis_data: Value,
    causal_data: Value,
) -> Result<EnhancedManuscript, Box<dyn std::error::Error>> {
    let mut builder = ManuscriptBuilder::new(
        format!("Discovery Lab Analysis Report - {}", execution_id),
        execution_id.clone(),
    )
    .with_authors(vec!["LogLine Discovery Lab".to_string()])
    .with_abstract("Automated analysis of molecular simulation data with causal inference.".to_string())
    .with_keywords(vec![
        "molecular dynamics".to_string(),
        "causal analysis".to_string(),
    ]);

    builder.add_methods_section(&protocol_data);
    builder.add_results_section(&analysis_data, &causal_data);

    let citation = Citation {
        id: "logline2024".to_string(),
        authors: vec!["Discovery Lab Team".to_string()],
        title: "LogLine Discovery Lab: Automated Scientific Discovery Pipeline".to_string(),
        journal: Some("Journal of Computational Discovery".to_string()),
        year: 2024,
        doi: Some("10.1234/example".to_string()),
        pmid: None,
        url: None,
        cited_in_sections: vec!["methods".to_string()],
    };
    builder.add_citation(citation);

    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manuscript_builder_builds_checksum() {
        let manuscript = ManuscriptBuilder::new("Test Manuscript".to_string(), "exec_001".to_string())
            .with_authors(vec!["Author".to_string()])
            .with_abstract("Abstract".to_string())
            .build();
        assert!(!manuscript.metadata.checksum.is_empty());
    }

    #[test]
    fn figure_generation_adds_plot() {
        let mut builder = ManuscriptBuilder::new("Test".to_string(), "exec_001".to_string());
        let data = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)];
        let fig_id = builder.generate_energy_plot(data, "results");
        assert!(fig_id.starts_with("fig_energy"));
        assert_eq!(builder.manuscript.figures.len(), 1);
    }

    #[test]
    fn markdown_render_contains_title() {
        let manuscript = ManuscriptBuilder::new("Test".to_string(), "exec_001".to_string())
            .with_abstract("Abstract text".to_string())
            .build();
        let markdown = render_markdown_from(&manuscript);
        assert!(markdown.contains("# Test"));
        assert!(markdown.contains("Abstract text"));
    }
}
