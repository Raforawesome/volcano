use rsass::{output::{Format, Style}};
use std::fs;

const CSS_DIR: &str = "src/css/";

fn main() {
	println!("cargo:rerun-if-changed={CSS_DIR}");
	fs::read_dir(CSS_DIR)
		.expect("Failed to read CSS directory: ")
		.filter_map(Result::ok)
		.filter(|obj| !obj.path().is_dir() 
			&& obj.file_name().into_string().unwrap().ends_with(".scss"))
		.for_each(|obj| {
			let css: String = fs::read_to_string(obj.path()).unwrap();

			let scss: String;
			unsafe {
				scss = String::from_utf8_unchecked(
					rsass::compile_scss(css.as_bytes(), 
					Format {
						style: Style::Compressed,
						..Default::default()
					}
					).unwrap()
				);
			}

			fs::write(CSS_DIR.to_owned() + 
				&obj.file_name().to_str().unwrap().replace(".scss", ".css"), &scss).unwrap();
			println!("written to {}", obj.file_name().to_str().unwrap().replace(".scss", ".css"));
		});
}
