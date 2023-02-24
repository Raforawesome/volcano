use std::fs;

const NOTES_DIR: &'static str = "./notes/";

pub fn get_note_names() -> Vec<String> {
	let names: Vec<String> = vec![];
	if let Ok(res) = fs::read_dir(NOTES_DIR) {
		let res = try {
			for obj in res {
				let entry: std::fs::DirEntry = entry?;
				if !entry.path().is_dir() {
					if let Some(str) = entry.file_name().to_str() {
						names.push(str.to_string());
					}
				}
			}
		};
		res.expect("fs.rs | Failed to read file names.");
	} else {
		fs::create_dir(NOTES_DIR).expect("fs.rs | Failed to create note directory.");
	}
	names
}