use serde::{Deserialize, Serialize};
use specta::Type;

/// Represents the user's intent for the citation style they are building.
/// This struct captures the state of the "Decision Wizard" and is used
/// to generate the next set of questions or the final CSL style.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
pub struct StyleIntent {
    /// The starting point or template (e.g., "apa", "chicago").
    pub base_archetype: Option<String>,
    /// The general class of citation (in-text, note, etc.).
    pub class: Option<CitationClass>,
    /// How author names should be formatted (long, short, et al.).
    pub author_format: Option<NameFormat>,
    /// Whether the style requires a bibliography.
    pub has_bibliography: Option<bool>,
}

impl StyleIntent {
    /// Analyzes the current intent and returns the next decision to be made.
    pub fn decide(&self) -> DecisionPackage {
        let mut missing_fields = Vec::new();
        if self.base_archetype.is_none() { missing_fields.push("base_archetype".to_string()); }
        if self.class.is_none() { missing_fields.push("class".to_string()); }
        if self.author_format.is_none() { missing_fields.push("author_format".to_string()); }
        if self.has_bibliography.is_none() { missing_fields.push("has_bibliography".to_string()); }

        // Determine the next question
        let (question, previews) = if self.class.is_none() {
            (
                Some(Question {
                    id: "class".to_string(),
                    text: "How should citations appear in your text?".to_string(),
                    description: Some("This determines the overall structure of your citations.".to_string()),
                }),
                vec![
                    Preview {
                        label: "Parenthetical (Author-Date)".to_string(),
                        html: "<div class='preview'>(Doe 2023)</div>".to_string(),
                        choice_value: serde_json::json!({ "class": "InText" }),
                    },
                    Preview {
                        label: "Numeric (Vancouver)".to_string(),
                        html: "<div class='preview'>[1]</div>".to_string(),
                        choice_value: serde_json::json!({ "class": "Numeric" }),
                    },
                    Preview {
                        label: "Notes (Chicago)".to_string(),
                        html: "<div class='preview'><sup>1</sup></div>".to_string(),
                        choice_value: serde_json::json!({ "class": "Note" }),
                    },
                ]
            )
        } else if self.author_format.is_none() {
            (
                Some(Question {
                    id: "author_format".to_string(),
                    text: "How should multiple authors be displayed?".to_string(),
                    description: Some("Choose how you want to handle author lists in citations.".to_string()),
                }),
                vec![
                    Preview {
                        label: "Full List".to_string(),
                        html: "<div class='preview'>(Doe, Smith, & Jones, 2023)</div>".to_string(),
                        choice_value: serde_json::json!({ "author_format": "Long" }),
                    },
                    Preview {
                        label: "Abbreviated (Et Al.)".to_string(),
                        html: "<div class='preview'>(Doe et al., 2023)</div>".to_string(),
                        choice_value: serde_json::json!({ "author_format": "EtAl" }),
                    },
                ]
            )
        } else if self.has_bibliography.is_none() {
            (
                Some(Question {
                    id: "has_bibliography".to_string(),
                    text: "Do you need a bibliography at the end?".to_string(),
                    description: Some("Most academic styles require a list of references at the end of the document.".to_string()),
                }),
                vec![
                    Preview {
                        label: "Yes, include bibliography".to_string(),
                        html: "<div class='preview'><b>Bibliography</b><br/>Doe, J. (2023). Title...</div>".to_string(),
                        choice_value: serde_json::json!({ "has_bibliography": true }),
                    },
                    Preview {
                        label: "No bibliography".to_string(),
                        html: "<div class='preview'><i>Just the citations.</i></div>".to_string(),
                        choice_value: serde_json::json!({ "has_bibliography": false }),
                    },
                ]
            )
        } else {
            (None, vec![])
        };

        DecisionPackage {
            missing_fields,
            question,
            previews,
            preview_html: self.render_preview(),
        }
    }

    /// Renders a live preview based on current intent fields.
    pub fn render_preview(&self) -> String {
        let mut html = String::new();
        html.push_str("<div class='live-preview-content'>");
        
        let citation = match self.class {
            Some(CitationClass::Numeric) => "[1]",
            Some(CitationClass::Note) => "Doe, \"Title,\" 1.",
            Some(CitationClass::InText) => match self.author_format {
                Some(NameFormat::EtAl) => "(Doe et al., 2023)",
                _ => "(Doe, Smith, & Jones, 2023)",
            },
            None => "[Select Citation Class]",
        };

        html.push_str(&format!("<div class='preview-citation'>{}</div>", citation));

        if self.has_bibliography.unwrap_or(false) {
            html.push_str("<div class='preview-bibliography'>");
            html.push_str("<h4>Example Bibliography</h4>");
            let bib_entry = match self.class {
                Some(CitationClass::Numeric) => "[1] Doe, J. (2023). <i>Title of Work</i>. Publisher.",
                _ => "Doe, J., Smith, R., & Jones, A. (2023). <i>Title of Work</i>. Publisher.",
            };
            html.push_str(&format!("<div class='bib-entry'>{}</div>", bib_entry));
            html.push_str("</div>");
        }

        html.push_str("</div>");
        html
    }

    /// Generates a complete CSLN YAML string based on the current intent.
    pub fn generate_csln(&self) -> String {
        // Construct the basic metadata for the new style
        let mut style = csln_core::Style {
            id: Some("custom-style".to_string()),
            title: Some("Custom Style".to_string()),
            version: Some("1.0".to_string()),
            last_updated: Some("2024-02-03T00:00:00+00:00".to_string()),
            ..Default::default()
        };

        // TODO: Map intent fields to csln_core::Style structure
        // This is a placeholder that outputs a valid but minimal CSLN YAML.
        // As csln_core evolves, we will map specific fields like 'class' and 'author_format'
        // to the actual Context/Driver configuration in CSLN.
        
        // Example (conceptual mapping):
        // if let Some(CitationClass::Numeric) = self.class {
        //     style.citation.driver = Some(Driver::Numeric);
        // }

        serde_yaml::to_string(&style).unwrap_or_else(|_| "# Error generating CSLN".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]

pub enum CitationClass {
    InText,
    Note,
    Numeric,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]

pub enum NameFormat {
    Long,
    Short,
    EtAl,
}

/// A package returned by the backend containing everything the frontend
/// needs to render the next step in the wizard.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct DecisionPackage {
    /// Fields that are currently None in the StyleIntent, helping the UI track progress.
    pub missing_fields: Vec<String>,
    
    /// The next specific question to ask the user.
    /// If None, the style is considered complete.
    pub question: Option<Question>,
    
    /// A/B testing previews to help the user decide.
    pub previews: Vec<Preview>,

    /// Raw HTML rendering of a sample citation/bibliography using current intent.
    pub preview_html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Preview {
    pub label: String,
    pub html: String,
    /// The JSON value (serialized) that will be applied to the intent if this option is chosen
    pub choice_value: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use specta::ts::{self, ExportConfiguration};
    use std::path::PathBuf;

    #[test]
    fn export_bindings() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // Navigate up from crates/intent-engine to project root, then down to client
        path.pop(); // crates
        path.pop(); // server
        path.pop(); // root
        path.push("client/src/lib/types/bindings.ts");
        
        let config = ExportConfiguration::default();

        let mut out = String::new();
        out.push_str("/* eslint-disable */\n// This file was generated by [specta](https://github.com/oscartbeaumont/specta). Do not edit this file manually.\n\n");
        out.push_str(&ts::export::<StyleIntent>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<CitationClass>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<NameFormat>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<DecisionPackage>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<Question>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<Preview>(&config).unwrap());
        out.push_str(";\n");

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, out).unwrap();
    }
}
