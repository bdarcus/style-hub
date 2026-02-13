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
    /// The academic field (e.g., "History", "Physics").
    pub field: Option<String>,
    /// The general class of citation (author-date, footnote, endnote, numeric).
    pub class: Option<CitationClass>,
    /// Detailed name and et-al options.
    pub author_format: Option<NameOptions>,
    /// Whether the style requires a bibliography.
    pub has_bibliography: Option<bool>,
    /// Visual preset choice for citations (e.g. "colon-locator", "comma-sep")
    pub citation_preset: Option<String>,
    /// Visual preset choice for bibliography (e.g. "year-wrapped", "flat")
    pub bibliography_preset: Option<String>,
    /// Whether to show advanced "detailed" configuration options
    pub detailed_config: Option<bool>,
}

impl StyleIntent {
    /// Analyzes the current intent and returns the next decision to be made.
    pub fn decide(&self) -> DecisionPackage {
        let mut missing_fields = Vec::new();
        if self.field.is_none() { missing_fields.push("field".to_string()); }
        if self.class.is_none() { missing_fields.push("class".to_string()); }
        
        // Dynamic missing fields based on class
        if let Some(class) = &self.class {
            match class {
                CitationClass::Footnote => {
                    if self.has_bibliography.is_none() { missing_fields.push("has_bibliography".to_string()); }
                    if self.author_format.is_none() { missing_fields.push("author_format".to_string()); }
                },
                CitationClass::Numeric | CitationClass::AuthorDate | CitationClass::Endnote => {
                    if self.author_format.is_none() { missing_fields.push("author_format".to_string()); }
                    if class == &CitationClass::AuthorDate && self.has_bibliography.is_none() {
                         missing_fields.push("has_bibliography".to_string());
                    }
                }
            }
        }

        // 1. Field Selection
        let (question, previews) = if self.field.is_none() {
            (
                Some(Question {
                    id: "field".to_string(),
                    text: "What is your academic field?".to_string(),
                    description: Some("Select one or more fields to find appropriate styles.".to_string()),
                }),
                vec![
                    Preview { label: "Humanities".to_string(), html: String::new(), choice_value: serde_json::json!({ "field": "humanities" }) },
                    Preview { label: "Social Science".to_string(), html: String::new(), choice_value: serde_json::json!({ "field": "social_science" }) },
                    Preview { label: "Sciences".to_string(), html: String::new(), choice_value: serde_json::json!({ "field": "sciences" }) },
                ]
            )
        // 2. Style Type Selection
        } else if self.class.is_none() {
            let field = self.field.as_ref().unwrap();
            let mut options = Vec::new();
            
            match field.as_str() {
                "humanities" => {
                    options.push(Preview { label: "Footnote".to_string(), html: String::new(), choice_value: serde_json::json!({ "class": "footnote" }) });
                    options.push(Preview { label: "Endnote".to_string(), html: String::new(), choice_value: serde_json::json!({ "class": "endnote" }) });
                    options.push(Preview { label: "Author-Date".to_string(), html: String::new(), choice_value: serde_json::json!({ "class": "author_date" }) });
                },
                "social_science" => {
                    options.push(Preview { label: "Author-Date".to_string(), html: String::new(), choice_value: serde_json::json!({ "class": "author_date" }) });
                },
                _ => { // Sciences
                    options.push(Preview { label: "Author-Date".to_string(), html: String::new(), choice_value: serde_json::json!({ "class": "author_date" }) });
                    options.push(Preview { label: "Numeric".to_string(), html: String::new(), choice_value: serde_json::json!({ "class": "numeric" }) });
                }
            }

            (
                Some(Question {
                    id: "class".to_string(),
                    text: "Select a style type".to_string(),
                    description: None,
                }),
                options
            )
        // 3. Configuration Step
        } else if let Some(class) = &self.class {
            match class {
                // 3a. Author-Date: Citation Pattern
                CitationClass::AuthorDate if self.citation_preset.is_none() => {
                    (
                        Some(Question {
                            id: "citation_preset".to_string(),
                            text: "How should citations appear in your text?".to_string(),
                            description: Some("Choose the pattern that matches your target publication.".to_string()),
                        }),
                        vec![
                            Preview { 
                                label: "(Smith and Jones, 2023: 34)".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "citation_preset": "colon-locator" }) 
                            },
                            Preview { 
                                label: "(Smith and Jones, 2023, p.34)".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "citation_preset": "comma-sep" }) 
                            },
                            Preview { 
                                label: "(Smith and Jones 2023, 34)".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "citation_preset": "minimal" }) 
                            },
                        ]
                    )
                },
                // 3b. Author-Date: Bibliography Pattern
                CitationClass::AuthorDate if self.bibliography_preset.is_none() => {
                    (
                        Some(Question {
                            id: "bibliography_preset".to_string(),
                            text: "How should entries look in the bibliography?".to_string(),
                            description: None,
                        }),
                        vec![
                            Preview { 
                                label: "Smith, J. (2023). Title...".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "bibliography_preset": "year-wrapped", "has_bibliography": true }) 
                            },
                            Preview { 
                                label: "Smith, J. 2023. Title...".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "bibliography_preset": "flat", "has_bibliography": true }) 
                            },
                        ]
                    )
                },
                // 3c. Author-Date: Detailed Config Toggle
                CitationClass::AuthorDate if self.detailed_config.is_none() => {
                    (
                        Some(Question {
                            id: "detailed_config".to_string(),
                            text: "Refine further?".to_string(),
                            description: Some("The presets cover 90% of cases. Do you need to tweak granular details like author initials or et al. rules?".to_string()),
                        }),
                        vec![
                            Preview { label: "No, presets are fine".to_string(), html: String::new(), choice_value: serde_json::json!({ "detailed_config": false }) },
                            Preview { label: "Yes, show detailed config".to_string(), html: String::new(), choice_value: serde_json::json!({ "detailed_config": true }) },
                        ]
                    )
                },
                // 3d. Author-Date: Granular Config (only if detailed_config is true)
                CitationClass::AuthorDate if self.detailed_config == Some(true) && self.author_format.is_none() => {
                    (
                        Some(Question {
                            id: "author_format".to_string(),
                            text: "Advanced Formatting".to_string(),
                            description: Some("Fine-tune how authors and names are handled.".to_string()),
                        }),
                        vec![
                            Preview { 
                                label: "Standard (APA-style et al.)".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "author_format": { "form": "long", "et_al": { "min": 3, "use_first": 1 } } }) 
                            },
                            Preview { 
                                label: "Always show all authors".to_string(), 
                                html: String::new(), 
                                choice_value: serde_json::json!({ "author_format": { "form": "long", "et_al": null } }) 
                            },
                        ]
                    )
                },
                
                // --- Other Classes ---

                // Footnote asks Bibliography first
                CitationClass::Footnote if self.has_bibliography.is_none() => {
                    (
                        Some(Question {
                            id: "has_bibliography".to_string(),
                            text: "Does this style include a bibliography?".to_string(),
                            description: Some("Note formatting typically changes if a bibliography is present.".to_string()),
                        }),
                        vec![
                            Preview { label: "Yes, include bibliography".to_string(), html: String::new(), choice_value: serde_json::json!({ "has_bibliography": true }) },
                            Preview { label: "No, notes only".to_string(), html: String::new(), choice_value: serde_json::json!({ "has_bibliography": false }) },
                        ]
                    )
                },
                // Numeric asks about wrapping (brackets, etc)
                CitationClass::Numeric if self.author_format.is_none() => {
                    (
                        Some(Question {
                            id: "author_format".to_string(),
                            text: "How should citation numbers be wrapped?".to_string(),
                            description: None,
                        }),
                        vec![
                            Preview { label: "Square Brackets [1]".to_string(), html: String::new(), choice_value: serde_json::json!({ "author_format": { "form": "short", "et_al": null } }) },
                            Preview { label: "Parentheses (1)".to_string(), html: String::new(), choice_value: serde_json::json!({ "author_format": { "form": "long", "et_al": null } }) },
                            Preview { label: "Superscript ¹".to_string(), html: String::new(), choice_value: serde_json::json!({ "author_format": { "form": "long", "et_al": { "min": 1, "use_first": 1 } } }) },
                        ]
                    )
                },
                // Default formatting (e.g. Endnote)
                _ if self.author_format.is_none() && self.class.is_some() => {
                    (
                        Some(Question {
                            id: "author_format".to_string(),
                            text: "Choose a formatting pattern".to_string(),
                            description: None,
                        }),
                        vec![
                            Preview { label: "Standard".to_string(), html: String::new(), choice_value: serde_json::json!({ "author_format": { "form": "long", "et_al": { "min": 3, "use_first": 1 } } }) },
                            Preview { label: "Full".to_string(), html: String::new(), choice_value: serde_json::json!({ "author_format": { "form": "long", "et_al": null } }) },
                        ]
                    )
                },
                _ => (None, vec![]),
            }
        } else {
            (None, vec![])
        };

        DecisionPackage {
            missing_fields,
            question,
            previews,
            in_text_preview: None,
            note_preview: None,
            bibliography_preview: None,
        }
    }

    /// Renders a live preview based on current intent fields.
    pub fn render_preview(&self) -> String {
        // Preview generation is now handled by the API layer using csln_processor.
        // We return an empty string here to indicate no static preview is available.
        String::new()
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
             Some(CitationClass::Footnote) | Some(CitationClass::Endnote) => Some(csln_core::TemplatePreset::ChicagoAuthorDate), // Placeholder
             Some(CitationClass::AuthorDate) => {
                 // Use bibliography_preset to pick the more specific template if possible
                 match self.bibliography_preset.as_deref() {
                     Some("year-wrapped") => Some(csln_core::TemplatePreset::Apa),
                     Some("flat") => Some(csln_core::TemplatePreset::ChicagoAuthorDate),
                     _ => Some(csln_core::TemplatePreset::Apa),
                 }
             },
             None => None,
        };

        if let Some(p) = preset {
             let wrap = match self.class {
                 Some(CitationClass::AuthorDate) => Some(csln_core::template::WrapPunctuation::Parentheses),
                 _ => None,
             };

             // Match citation_preset to locator formatting
             if let Some("colon-locator") = self.citation_preset.as_deref() {
                 // For now, these templates handle simple cases through their internal presets.
                 // In a more complex engine, we'd add custom locator templates here.
             }

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
                 options: options.clone(),
                 ..Default::default()
             });
             
             // If bibliography is requested, add it too
             if self.has_bibliography.unwrap_or(false) {
                 style.bibliography = Some(csln_core::BibliographySpec {
                     use_preset: Some(p),
                     options,
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
        let package = intent.decide();
        assert!(package.question.is_some());
        assert_eq!(package.question.unwrap().id, "field");
    }

    #[test]
    fn test_to_style_numeric() {
        let mut intent = StyleIntent::default();
        intent.class = Some(CitationClass::Numeric);
        let style = intent.to_style();
        assert!(style.citation.is_some());
        let spec = style.citation.unwrap();
        assert_eq!(spec.use_preset, Some(csln_core::TemplatePreset::Vancouver));
    }

    #[test]
    fn test_to_style_etal() {
        let mut intent = StyleIntent::default();
        intent.class = Some(CitationClass::AuthorDate);
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
#[serde(rename_all = "snake_case")]
pub enum CitationClass {
    AuthorDate,
    Footnote,
    Endnote,
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

    /// Specific citation example (for author-date styles)
    pub in_text_preview: Option<String>,

    /// Specific note example (for note-based styles)
    pub note_preview: Option<String>,

    /// Bibliography entry examples
    pub bibliography_preview: Option<String>,
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
