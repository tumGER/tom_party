use rocket_contrib::json::JsonValue;

pub fn error_message(content: &str) -> JsonValue {
    json!({
        "error": content,
        "worked": false
     })
}
