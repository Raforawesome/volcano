use std::{ffi::OsString, fs, path::PathBuf, io::Read};

const NOTES_DIR: &'static str = "./notes/";

fn os_to_str(s: OsString) -> Option<String> {
    s.to_str().and_then(|s| Some(s.to_string()))
}

unsafe fn unch_os_to_str(s: OsString) -> String {
    s.to_str().unwrap().to_string()
}

fn or_create_dir() {
	let path: PathBuf = PathBuf::from(NOTES_DIR);
	let _ = fs::create_dir(&path);
}

// Currently this just supports first level notes (no directories)
pub unsafe fn get_note_names() -> Result<Vec<String>, std::io::Error> {
	or_create_dir();
    let names: Vec<String> = fs::read_dir(NOTES_DIR)?
		.filter_map(Result::ok)
		.filter_map(|obj| {
			if !obj.path().is_dir() { 
				let name: String = unch_os_to_str(obj.file_name());
				if name.ends_with(".md") { Some(name) } else { None }
			} else { 
				None 
			}
		})
		.collect();
	Ok(names)
}

pub fn get_note_from_name(name: &str) -> Result<String, std::io::Error> {
	let mut path: PathBuf = PathBuf::from(NOTES_DIR);
	path.push(name);
	let mut buffer: String = String::new();
	let mut file = fs::File::open(&path)?;
	file.read_to_string(&mut buffer)?;
	Ok(buffer)
}

// TODO: Create a recursing function to walk directories too
// pub fn get_note_paths() -> Vec<PathBuf> {
// 	let names: Vec<PathBuf> = vec![];
// 	if let Ok(res) = fs::read_dir(NOTES_DIR) {
// 		let res = try {
// 			for obj in res {
// 				let entry: std::fs::DirEntry = entry?;
// 				let entry_path: PathBuf = entry.path();
// 				if entry_path.is_dir() {
// 					let mut path: PathBuf = entry.path();
// 					// Unfinished
// 				} else {
// 					names.push(entry_path);
// 				}
// 			}
// 		};
// 		res.expect("fs.rs | Failed to read file names.");
// 	} else {
// 		fs::create_dir(NOTES_DIR).expect("fs.rs | Failed to create note directory.");
// 	}
// 	names
// }
