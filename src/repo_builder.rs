use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

use crate::models::EnhancedSubmissionDetails;

const PATH_TO_LANG_INFO_FILE: &str = "./data/lang_info.json";

#[derive(Deserialize)]
struct LangToExtensionMapping {
    lang_name: String,
    extension: String,
}

pub fn initialise_repo(output_dir: &str) -> std::io::Result<()> {
    fs::create_dir_all(&output_dir)?;
    let path_to_readme: PathBuf = [output_dir, "README.md"].iter().collect();
    fs::write(&path_to_readme, "Created with leetcode-repo-maker")?;
    Ok(())
}

pub fn get_last_submission(output_dir: &str) -> Option<usize> {
    let submission_path: PathBuf = [output_dir, ".last_submission"].iter().collect();
    if submission_path.exists() {
        let mut file = File::open(submission_path).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");

        let parsed_value: usize = contents.trim().parse().unwrap();
        println!("last submission read {parsed_value}");
        Some(parsed_value)
    } else {
        None
    }
}

pub fn ovewrite_last_submission(output_dir: &str, last_submission_id: &str) -> std::io::Result<()> {
    let last_submission_path: PathBuf = [output_dir, ".last_submission"].iter().collect();
    fs::write(&last_submission_path, last_submission_id.trim())?;
    Ok(())
}



pub fn add_submission_to_repo(
    base_dir: &str,
    enhanced_submission_details: &EnhancedSubmissionDetails,
    lang_names_to_extensions: &HashMap<String, String>,
) -> std::io::Result<()> {
    let lang_dir_name = &enhanced_submission_details
        .submission_details
        .lang
        .verbose_name;

    // TODO: Use title instead of title-slug to name the directory
    let problem_dir_name = &generate_problem_dir_name(
        &enhanced_submission_details.title_slug,
        &enhanced_submission_details
            .submission_details
            .question
            .question_id,
    );
    let mut path: PathBuf = [base_dir, problem_dir_name, lang_dir_name]
        .into_iter()
        .collect();

    fs::create_dir_all(&path)?;

    let filename = generate_filename(enhanced_submission_details, lang_names_to_extensions);
    path.push(&filename);

    fs::write(path, &enhanced_submission_details.submission_details.code)?;

    Ok(())
}

pub fn get_lang_to_extension_mapping() -> HashMap<String, String> {
    let mut file = File::open(PATH_TO_LANG_INFO_FILE).expect("Failed to open lang info file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file.");

    let mappings: Vec<LangToExtensionMapping> =
        serde_json::from_str(&contents).expect("Failed to deserialize JSON");

    let mut lang_to_extension: HashMap<String, String> = HashMap::new();

    for mapping in mappings {
        lang_to_extension.insert(mapping.lang_name, mapping.extension);
    }

    lang_to_extension
}

fn generate_filename(
    enhanced_submission_details: &EnhancedSubmissionDetails,
    lang_names_to_extensions: &HashMap<String, String>,
) -> String {
    let language_name = &enhanced_submission_details.submission_details.lang.name;
    let binding = "txt".to_string();
    let extension = lang_names_to_extensions
        .get(language_name)
        .unwrap_or(&binding);
    let filename = &enhanced_submission_details.title_slug;
    let submission_id = &enhanced_submission_details.submission_id;
    format!("{filename}_{submission_id}.{extension}")
}

fn generate_problem_dir_name(title_slug: &str, question_id: &str) -> String {
    format!("{question_id:0>4}-{title_slug}")
}
