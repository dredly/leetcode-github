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

pub fn create_dir_structure(problem_id: i32, problem_blob: &str, language: &str, language_extension: &str) -> std::io::Result<()> {
    // 
    let problem_blob_lower = problem_blob.to_lowercase();
    let desired_path = &format!("/home/lap/{problem_id}-{problem_blob}/{language}/{problem_blob_lower}.{language_extension}");

    fs::create_dir_all(Path::new(desired_path))?;
    Ok(())
}


#[test]
fn test_create_dir_structure() -> std::io::Result<()> {
    create_dir_structure(2, "Two-Sum", "Python", "py")
}






