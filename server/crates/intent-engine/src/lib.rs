use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub struct NameOptions {
    pub form: NameForm,
    pub et_al: Option<EtAlConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum NameForm {
    Long,
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub struct EtAlConfig {
    pub min: u8,
    pub use_first: u8,
}

/// Represents the user's intent for the citation style they are building.
/// This struct captures the state of the "Decision Wizard" and is used
/// to generate the next set of questions or the final CSL style.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
#[serde(rename_all = "snake_case")]
pub struct StyleIntent {
    /// The starting point or template (e.g., "apa", "chicago").
    pub base_archetype: Option<String>,
    /// The general class of citation (in-text, note, etc.).
    pub class: Option<CitationClass>,
    /// Detailed name and et-al options.
    pub author_format: Option<NameOptions>,
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
                        choice_value: serde_json::json!({ "class": "in_text" }),
                    },
                    Preview {
                        label: "Numeric (Vancouver)".to_string(),
                        html: "<div class='preview'>[1]</div>".to_string(),
                        choice_value: serde_json::json!({ "class": "numeric" }),
                    },
                    Preview {
                        label: "Notes (Chicago)".to_string(),
                        html: "<div class='preview'><sup>1</sup></div>".to_string(),
                        choice_value: serde_json::json!({ "class": "note" }),
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
                        choice_value: serde_json::json!({ 
                            "author_format": { "form": "long", "et_al": null } 
                        }),
                    },
                    Preview {
                        label: "Abbreviated (Et Al. after 3)".to_string(),
                        html: "<div class='preview'>(Doe et al., 2023)</div>".to_string(),
                        choice_value: serde_json::json!({ 
                            "author_format": { "form": "long", "et_al": { "min": 3, "use_first": 1 } } 
                        }),
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
            Some(CitationClass::InText) => {
                let has_et_al = self.author_format.as_ref()
                    .and_then(|f| f.et_al.as_ref())
                    .is_some();
                if has_et_al {
                    "(Doe et al., 2023)"
                } else {
                    "(Doe, Smith, & Jones, 2023)"
                }
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

    /// Converts the current intent into a `csln_core::Style` struct.
    pub fn to_style(&self) -> csln_core::Style {
         // Construct the basic metadata for the new style
        let mut style = csln_core::Style {
            info: csln_core::StyleInfo {
                id: Some("custom-style".to_string()),
                title: Some("Custom Style".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let preset = match self.class {
             Some(CitationClass::Numeric) => Some(csln_core::TemplatePreset::Vancouver),
             Some(CitationClass::Note) => Some(csln_core::TemplatePreset::ChicagoAuthorDate), // Fallback/Placeholder
             Some(CitationClass::InText) => Some(csln_core::TemplatePreset::Apa),
             None => None,
        };

        if let Some(p) = preset {
             let wrap = match self.class {
                 Some(CitationClass::InText) => Some(csln_core::template::WrapPunctuation::Parentheses),
                 _ => None,
             };

             let options = self.author_format.as_ref().and_then(|f| {
                 f.et_al.as_ref().map(|et_al| csln_core::options::Config {
                     contributors: Some(csln_core::options::ContributorConfig {
                         shorten: Some(csln_core::options::ShortenListOptions {
                             min: et_al.min,
                             use_first: et_al.use_first,
                             ..Default::default()
                         }),
                         ..Default::default()
                     }),
                     ..Default::default()
                 })
             });

             style.citation = Some(csln_core::CitationSpec {
                 use_preset: Some(p.clone()),
                 wrap,
                 options,
                 ..Default::default()
             });
             
             // If bibliography is requested, add it too (using same preset usually works for matching styles)
             if self.has_bibliography.unwrap_or(false) {
                 style.bibliography = Some(csln_core::BibliographySpec {
                     use_preset: Some(p),
                     ..Default::default()
                 });
             }
        }

        style
    }

    /// Generates a complete CSLN YAML string based on the current intent.
    pub fn generate_csln(&self) -> String {
        let style = self.to_style();
        serde_yaml::to_string(&style).unwrap_or_else(|_| "# Error generating CSLN".to_string())
    }

}

#[cfg(test)]
mod intent_tests {
    use super::*;

    #[test]
    fn test_render_preview_initial() {
        let intent = StyleIntent::default();
        let html = intent.render_preview();
        assert!(html.contains("[Select Citation Class]"));
    }

    #[test]
    fn test_to_style_numeric() {
        let mut intent = StyleIntent::default();
        intent.class = Some(CitationClass::Numeric);
        let style = intent.to_style();
        assert!(style.citation.is_some());
        let spec = style.citation.unwrap();
        assert_eq!(spec.use_preset, Some(csln_core::TemplatePreset::Vancouver));
        assert_eq!(spec.wrap, None);
    }

    #[test]
    fn test_to_style_etal() {
        let mut intent = StyleIntent::default();
        intent.class = Some(CitationClass::InText);
        intent.author_format = Some(NameOptions {
            form: NameForm::Long,
            et_al: Some(EtAlConfig { min: 3, use_first: 1 }),
        });
        let style = intent.to_style();
        
        let spec = style.citation.unwrap();
        assert_eq!(spec.wrap, Some(csln_core::template::WrapPunctuation::Parentheses));
        
        let opts = spec.options.unwrap();
        let contribs = opts.contributors.unwrap();
        let shorten = contribs.shorten.unwrap();
        assert_eq!(shorten.min, 3);
        assert_eq!(shorten.use_first, 1);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum CitationClass {
    InText,
    Note,
    Numeric,
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
        out.push_str(&ts::export::<NameOptions>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<NameForm>(&config).unwrap());
        out.push_str(";\n\n");
        out.push_str(&ts::export::<EtAlConfig>(&config).unwrap());
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
