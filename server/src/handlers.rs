use axum::{http::StatusCode, Json};
use std::fs;

pub async fn ping() -> &'static str {
    "Up!"
}

pub async fn get_note_names() -> Json<Vec<String>> {
    Json(
        fs::read_dir("notes/")
            .unwrap()
            .filter_map(|f| {
                let name = f.unwrap().file_name().into_string().unwrap();
                if name.ends_with(".md") {
                    Some(name[..(name.len() - 3)].to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>(),
    )
}

pub async fn get_note_content(
    Json(payload): Json<serde_json::Value>,
) -> Result<String, StatusCode> {
    if let serde_json::Value::String(s) = payload {
        if fs::try_exists(format!("notes/{s}.md")).unwrap() {
            Ok(fs::read_to_string(format!("notes/{s}.md")).unwrap())
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
