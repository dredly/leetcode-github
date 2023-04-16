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
use crate::models::EnhancedSubmissionDetails;

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

pub fn add_submission_to_repo(base_dir: &str, enhanced_submission_details: &EnhancedSubmissionDetails) -> std::io::Result<()> {
    let lang_dir_name = &enhanced_submission_details.submission_details.lang.verbose_name;
    
    // TODO: Use title instead of title-slug to name the directory
    let problem_dir_name = &generate_problem_dir_name(&enhanced_submission_details.title_slug, &enhanced_submission_details.submission_details.question.question_id); 
    let mut path: PathBuf = [base_dir, problem_dir_name, lang_dir_name].into_iter().collect();
    
    fs::create_dir_all(&path)?;
    
    let filename = generate_filename(enhanced_submission_details);
    path.push(&filename);
    
    fs::write(path, &enhanced_submission_details.submission_details.code)?;

    Ok(())
}

pub fn initialise_repo(output_dir: &str) -> std::io::Result<()> {
    fs::create_dir_all(&output_dir)?;
    let path_to_readme: PathBuf = [output_dir, "README.md"].iter().collect();
    fs::write(&path_to_readme, "hello there")?;
    Ok(())
}

fn generate_filename(enhanced_submission_details: &EnhancedSubmissionDetails) -> String {
    let language_name = &enhanced_submission_details.submission_details.lang.name;
    let extension = *LANGUAGE_NAMES_TO_EXTENSIONS.get(language_name.as_str()).unwrap_or(&"txt");
    let filename = &enhanced_submission_details.title_slug;
    let submission_id = &enhanced_submission_details.submission_id;
    format!("{filename}_{submission_id}.{extension}")
}

fn generate_problem_dir_name(title_slug: &str, question_id: &str) -> String {
    format!("{question_id:0>4}-{title_slug}")
}

#[test]
fn test_create_dir_structure() {
    create_output("/home/lap", 2, 55, "Python", "py", "print(5)")
}
