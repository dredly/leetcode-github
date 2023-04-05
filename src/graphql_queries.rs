pub const query_submission_list: &str = r#"
query ($offset: Int!, $limit: Int!) {
    submissionList(offset: $offset, limit: $limit) {
        submissions { id lang statusDisplay title titleSlug}
    }
}
"#;

pub const query_submission_details: &str = r#"
query submissionDetails($submissionId: Int!) {
    submissionDetails(submissionId: $submissionId) {
      runtime
      runtimeDisplay
      memoryDisplay
      code
      question {
        questionId
      }
    }
  }
  
"#;

