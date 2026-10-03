use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub(super) const ENGINE: &str = include_str!("../../math-playground/runtime/engine-core.mjs");
pub(super) const LEGACY_RENDERER: &str = include_str!("../../math-playground/runtime/renderer.mjs");
pub(super) const RENDERER: &str = include_str!("../../math-playground/runtime/renderer-0.2.0.mjs");
pub(super) const MAX_SOURCE_BYTES: usize = 64 * 1024;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SourceFile {
    pub name: String,
    pub content: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Intake {
    pub files: Vec<SourceFile>,
    pub provider_consent: bool,
    #[serde(default)]
    pub ai_provider: super::generation::Provider,
}

impl Intake {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.provider_consent {
            return Err("provider_consent_required");
        }
        if self.files.is_empty() || self.files.len() > 8 {
            return Err("choose_one_to_eight_tex_files");
        }
        let mut names = std::collections::HashSet::new();
        let mut total = 0;
        for file in &self.files {
            if !file.name.ends_with(".tex")
                || file.name.len() > 120
                || file.name.contains(['/', '\\'])
                || file.name.chars().any(char::is_control)
                || !names.insert(&file.name)
            {
                return Err("invalid_or_duplicate_tex_filename");
            }
            if file.content.trim().is_empty()
                || file.content.len() > 32 * 1024
                || file.content.contains('\0')
            {
                return Err("tex_file_empty_or_over_32_kib");
            }
            total += file.content.len();
        }
        if total > MAX_SOURCE_BYTES {
            return Err("source_over_64_kib_select_a_concept_excerpt");
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub(super) enum Kind {
    Circle,
    Sequence,
    Permutation,
    Metaphor,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Concept {
    pub title: String,
    pub kind: Kind,
    pub concept_type: String,
    pub notation: String,
    pub hypotheses: String,
    pub statement: String,
    pub visual_mapping: String,
    pub limitations: String,
    pub source_file: String,
    pub source_quote: String,
    pub palette: String,
}

impl Concept {
    pub fn validate(&self, files: &[SourceFile]) -> Result<(), &'static str> {
        self.validate_fields()?;
        let source = files
            .iter()
            .find(|file| file.name == self.source_file)
            .ok_or("source_anchor_missing")?;
        if self.source_quote.trim().len() < 8 || !source.content.contains(&self.source_quote) {
            return Err("source_quote_not_found");
        }
        Ok(())
    }

    pub fn validate_fields(&self) -> Result<(), &'static str> {
        for text in [
            &self.title,
            &self.concept_type,
            &self.notation,
            &self.hypotheses,
            &self.statement,
            &self.visual_mapping,
            &self.limitations,
            &self.source_file,
            &self.source_quote,
            &self.palette,
        ] {
            if text.trim().is_empty() || text.len() > 6000 || text.contains('\0') {
                return Err("invalid_concept_fields");
            }
        }
        if self.title.len() > 160
            || !["definition", "theorem", "other"].contains(&self.concept_type.as_str())
            || !["candy", "ocean", "sunset"].contains(&self.palette.as_str())
        {
            return Err("invalid_concept_options");
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Exhibit {
    pub id: String,
    pub version: String,
    pub formal_version: String,
    pub concept: Concept,
    pub source_sha256: String,
    pub engine_sha256: String,
    pub renderer_sha256: String,
    pub created_at: i64,
    pub review: String,
    pub repository_commit: Option<String>,
}

pub(super) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn engine(id: &str, concept: &Concept) -> String {
    // Only allowlisted values and a server-generated identifier enter executable code.
    // Source text, source filenames and AI-authored prose are deliberately excluded.
    let config = serde_json::json!({"id": id, "kind": concept.kind, "palette": concept.palette});
    format!("export const config = {config};\n{ENGINE}")
}

pub(super) fn valid_id(id: &str) -> bool {
    id.len() == 32
        && id
            .bytes()
            .all(|value| value.is_ascii_hexdigit() && !value.is_ascii_uppercase())
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub(in crate::math_playground) fn fixture() -> Concept {
        Concept {
            title: "Pi's ribbon".into(),
            kind: Kind::Circle,
            concept_type: "definition".into(),
            notation: "r > 0, C = circumference".into(),
            hypotheses: "Euclidean circle".into(),
            statement: "C = 2πr".into(),
            visual_mapping: "Ribbon unwraps circumference".into(),
            limitations: "Numerical illustration, not a proof".into(),
            source_file: "circle.tex".into(),
            source_quote: "C = 2\\pi r".into(),
            palette: "candy".into(),
        }
    }

    #[test]
    fn intake_rejects_traversal_duplicates_and_missing_consent() {
        let mut intake = Intake {
            ai_provider: super::super::generation::Provider::OpenAI,
            provider_consent: true,
            files: vec![SourceFile {
                name: "../x.tex".into(),
                content: "test".into(),
            }],
        };
        assert!(intake.validate().is_err());
        intake.files[0].name = "x.tex".into();
        assert!(intake.validate().is_ok());
        intake.files.push(intake.files[0].clone());
        assert!(intake.validate().is_err());
        intake.files.pop();
        intake.provider_consent = false;
        assert!(intake.validate().is_err());
    }

    #[test]
    fn model_requires_verbatim_source_evidence_and_allowlisted_options() {
        let mut concept = fixture();
        let source = vec![SourceFile {
            name: "circle.tex".into(),
            content: concept.source_quote.clone(),
        }];
        assert!(concept.validate(&source).is_ok());
        concept.source_quote = "invented statement".into();
        assert!(concept.validate(&source).is_err());
        concept.palette = "');fetch('/secret')".into();
        assert!(concept.validate_fields().is_err());
    }

    #[test]
    fn generated_code_excludes_course_text_and_private_identity() {
        let mut concept = fixture();
        concept.statement = "PRIVATE EXAM QUESTION".into();
        let code = engine("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", &concept);
        assert!(!code.contains("PRIVATE EXAM"));
        assert!(!code.contains("circle.tex"));
        assert!(code.contains("\"kind\":\"circle\""));
        assert_eq!(digest(code.as_bytes()).len(), 64);
    }
}
