//! Stable structured boundary for applications that embed ClearSign.
//!
//! Every client receives the same response shape from the same Rust reviewer.
//! Presentation code may decide how to lay the response out, but it never
//! decides what a transaction means or how severe a finding is.

#![forbid(unsafe_code)]

use clearsign::{DomainVersion, Severity};
use serde::{Deserialize, Serialize};

/// Version of the serialized response contract.
pub const API_VERSION: u32 = 1;

/// Largest Safe JSON record accepted by any ClearSign application boundary.
pub const MAX_RECORD_BYTES: usize = clearsign_safe_json::MAX_RECORD_BYTES;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSection {
    pub title: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewFinding {
    pub number: u32,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredAcknowledgement {
    pub number: u32,
    pub code: String,
}

/// Response returned by every application boundary.
///
/// Refusals intentionally use the same envelope as successful reviews. A
/// client cannot turn malformed or unsupported input into an empty success.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResponse {
    pub api_version: u32,
    pub ok: bool,
    pub severity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_confirmed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_bound: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sections: Vec<ReviewSection>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<ReviewFinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_acknowledge: Vec<RequiredAcknowledgement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl ReviewResponse {
    pub fn refusal(message: impl Into<String>) -> Self {
        Self {
            api_version: API_VERSION,
            ok: false,
            severity: String::from("refused"),
            message: Some(message.into()),
            hash: None,
            hash_confirmed: None,
            domain: None,
            domain_source: None,
            chain_id_source: None,
            chain_bound: None,
            sections: Vec::new(),
            findings: Vec::new(),
            must_acknowledge: Vec::new(),
            text: None,
        }
    }
}

/// Review Safe Transaction Service JSON using the public integer conventions.
///
/// `chain_id` 0 means the record must provide it. `version` 0 means determine
/// the domain from the included hash, 1 means Safe v1.1.x, and 3 means v1.3.0+.
pub fn review_safe_json(json_text: &str, chain_id: u64, version: u32) -> ReviewResponse {
    let version = match version {
        1 => Some(DomainVersion::Legacy),
        3 => Some(DomainVersion::V1_3Plus),
        _ => None,
    };
    let chain_id = if chain_id == 0 { None } else { Some(chain_id) };

    match clearsign_safe_json::review_json(json_text, chain_id, version) {
        Ok(reviewed) => {
            let severity = match reviewed.review.highest_severity() {
                Some(Severity::Critical) => "critical",
                Some(Severity::Blind) => "blind",
                Some(Severity::Warning) => "warning",
                Some(Severity::Info) | None => "clear",
            };
            ReviewResponse {
                api_version: API_VERSION,
                ok: true,
                severity: String::from(severity),
                message: None,
                hash: Some(clearsign::hex::encode_prefixed(&reviewed.hash)),
                hash_confirmed: Some(reviewed.hash_confirmed),
                domain: Some(String::from(match reviewed.version {
                    DomainVersion::Legacy => "v1.1.x",
                    DomainVersion::V1_3Plus => "v1.3.0 or later",
                })),
                domain_source: Some(String::from(reviewed.version_source)),
                chain_id_source: Some(String::from(reviewed.chain_id_source)),
                chain_bound: Some(!matches!(reviewed.version, DomainVersion::Legacy)),
                sections: reviewed
                    .review
                    .sections
                    .iter()
                    .map(|section| ReviewSection {
                        title: section.title.clone(),
                        fields: section.fields.clone(),
                    })
                    .collect(),
                findings: reviewed
                    .review
                    .numbered_findings()
                    .into_iter()
                    .map(|(number, finding)| ReviewFinding {
                        number,
                        severity: String::from(finding.severity.label()),
                        code: String::from(finding.code),
                        message: finding.message.clone(),
                    })
                    .collect(),
                must_acknowledge: reviewed
                    .review
                    .required_acknowledgements()
                    .into_iter()
                    .map(|(number, code)| RequiredAcknowledgement {
                        number,
                        code: String::from(code),
                    })
                    .collect(),
                text: Some(reviewed.review.render()),
            }
        }
        Err(message) => ReviewResponse::refusal(message),
    }
}

/// Serialize a response for native and WebAssembly boundaries.
pub fn review_safe_json_string(
    json_text: &str,
    chain_id: u64,
    version: u32,
) -> Result<String, serde_json::Error> {
    serde_json::to_string(&review_safe_json(json_text, chain_id, version))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYBIT: &str = include_str!("../../clearsign-cli/tests/fixtures/bybit-safe-tx.json");

    #[test]
    fn bybit_has_the_same_critical_structured_verdict() {
        let response = review_safe_json(BYBIT, 1, 0);
        assert!(response.ok);
        assert_eq!(response.api_version, API_VERSION);
        assert_eq!(response.severity, "critical");
        assert!(
            response
                .findings
                .iter()
                .any(|finding| finding.code == "SAFE_DELEGATECALL")
        );
        let numbers: Vec<u32> = response
            .findings
            .iter()
            .map(|finding| finding.number)
            .collect();
        assert_eq!(
            numbers,
            (1..=u32::try_from(numbers.len()).unwrap_or(0)).collect::<Vec<_>>()
        );
        assert_eq!(response.hash_confirmed, Some(true));
    }

    #[test]
    fn malformed_input_is_a_refusal_not_an_api_error() {
        let response = review_safe_json("{", 1, 0);
        assert!(!response.ok);
        assert_eq!(response.severity, "refused");
        assert!(response.message.is_some());
    }

    #[test]
    fn serialized_fields_keep_the_versioned_client_contract() -> Result<(), String> {
        let text = review_safe_json_string(BYBIT, 1, 0).map_err(|error| error.to_string())?;
        let value: serde_json::Value =
            serde_json::from_str(&text).map_err(|error| error.to_string())?;
        assert_eq!(value.get("apiVersion").and_then(|v| v.as_u64()), Some(1));
        assert_eq!(
            value.get("hashConfirmed").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert!(value.get("mustAcknowledge").is_some());
        let response: ReviewResponse =
            serde_json::from_str(&text).map_err(|error| error.to_string())?;
        for required in &response.must_acknowledge {
            assert!(response.findings.iter().any(|finding| {
                finding.number == required.number && finding.code == required.code
            }));
        }
        Ok(())
    }
}
