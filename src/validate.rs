use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_launch_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    for f in ["client_id", "redirect_uri", "scope", "authorization_endpoint", "token_endpoint"] {
        if v.get(f).and_then(|x| x.as_str()).is_none_or(|s| s.is_empty()) {
            r.push(ValidationIssue::error(f, "SMART confidential launch config field required", "launch-field"));
        }
    }
    Ok(r)
}
