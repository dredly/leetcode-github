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

#[test]
fn test_create_dir_structure() {
    create_output("/home/lap", 2, "Two-Sum", "Python", "print(5)")
}
