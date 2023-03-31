// This module should eventually fetch a list of
// notes from the server, but for now it'll just return a dummy value.

#[cfg(target_family = "windows")]
use reqwest;
#[cfg(target_family = "unix")]
use reqwest;
#[cfg(target_family = "wasm")]
use reqwest_wasm as reqwest;

use std::collections::HashMap;

pub fn get_note_content(title: &str) -> String {
    if title == "Note 1" {
        FILE1.into()
    } else if title == "Note 2" {
        FILE2.into()
    } else {
        FILE3.into()
    }
}

const FILE1: &str = r#"
# Test Note 1
## Sample math class note

### a)
Hint: Divide through by highest power of x in the denominator
$$\lim_{x\to \infty} \frac{2x+1}{x^2}$$
$$= \lim_{x\to \infty} \frac{2x+1}{x^2} \times \frac{\frac{1}{x^2}}{\frac{1}{x^2}}$$
$$= \lim_{x\to \infty} \frac{\frac{2x}{x^2} + \frac{1}{x^2}}{\frac{x^2}{x^2}}$$
$$= \lim_{x\to \infty} \frac{\frac{2x}{x^2} + \frac{1}{x^2}}{1}$$
$$= 0$$
"#;

const FILE2: &str = r#"
# Test Note 2
## More sample math class notes

### b)
$$\lim_{x\to \infty} \frac{4x-7}{9x+1}$$
$$\lim_{x\to \infty} \frac{\frac{4x}{x}-\frac{7}{x}}{\frac{9x}{x}+\frac{1}{x}}$$
$$= \frac{4}{9}$$
"#;

const FILE3: &str = r#"
# Test Note 3
## Social science notes (jk its math)

### c)
$$\lim_{x\to \infty} \frac{2x^3+3x^2-1}{x^2-1}$$
$$\lim_{x\to infin} \frac{2x+3-\frac{1}{x^2}}{1-\frac{1}{x^2}}$$
$$= \infty$$
"#;

pub fn get_test_notes() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let _ = map.insert("Note 1".into(), "v".into());
    map
}
