//! Unit tests for HTTP error status and payload compatibility.

use super::*;
use serde_json::json;

#[test]
fn application_error_responses_keep_status_and_json_shape() {
    for (error, expected_status) in [
        (ApplicationError::Validation("bad input".into()), StatusCode::BAD_REQUEST),
        (ApplicationError::NotFound("missing".into()), StatusCode::NOT_FOUND),
        (ApplicationError::Repository("storage".into()), StatusCode::INTERNAL_SERVER_ERROR),
    ] {
        let message = error.to_string();
        let (status, Json(payload)) = map_application_error(error);
        assert_eq!(status, expected_status);
        assert_eq!(
            serde_json::to_value(&payload).expect("serializable API error"),
            json!({"code": "API_ERROR", "message": message})
        );
    }
}
