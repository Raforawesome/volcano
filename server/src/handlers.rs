use axum::{http::{StatusCode, HeaderMap}, Json};
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
    // Json(payload): Json<serde_json::Value>,
	headers: HeaderMap
) -> Result<String, StatusCode> {
    if headers.contains_key("note") {
		let s: &str = headers.get("note").unwrap().to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
        if fs::try_exists(format!("notes/{s}.md")).unwrap() {
            Ok(fs::read_to_string(format!("notes/{s}.md")).unwrap())
        } else {
			println!("Didn't find requested note '{s}.md'");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
