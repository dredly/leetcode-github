/* WIP, idea is to do something like

├── 0001-Two-Sum
│   ├── Python3
│   │   ├── two_sum_0.py
│   │   ├── two_sum_1.py
│   ├── Rust
│   │   ├── two_sum_0.rs
*/
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::LANGUAGE_NAMES_TO_EXTENSIONS;
use crate::models::SubmissionDetails;

pub fn create_output(
    output_dir: &str,
    submission_id: u32,
    problem_id: i32,
    problem_blob: &str,
    language: &str,
    code: &str,
) {
    let problem_str = &format!("{problem_id:0>4}-{problem_blob}");
    let mut path: PathBuf = [output_dir, problem_str, language].iter().collect();

    fs::create_dir_all(&path);
    let program_file = format!("{problem_blob}_{submission_id}.py");
    path.push(&program_file);
    fs::write(&path, code);
}

pub fn display_submission_name(submission_details: &SubmissionDetails) {
    let language_name = &submission_details.lang.name;
    let extension = *LANGUAGE_NAMES_TO_EXTENSIONS.get(language_name.as_str()).unwrap_or(&"txt");
    let filename = &format!("test_question.{extension}");
    println!("This submission will be saved as {}", filename);
}

#[test]
fn test_create_dir_structure() {
    create_output("/home/lap", 2, 55, "Python", "py", "print(5)")
}
