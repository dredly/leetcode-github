pub const QUERY_SUBMISSION_LIST: &str = r#"
query ($offset: Int!, $limit: Int!) {
    submissionList(offset: $offset, limit: $limit) {
        submissions { id lang statusDisplay title titleSlug}
        hasNext
    }
}
"#;

pub const QUERY_SUBMISSION_DETAILS: &str = r#"
query submissionDetails($submissionId: Int!) {
    submissionDetails(submissionId: $submissionId) {
      code
      question {
        questionId
      }
      lang {
        name
        verboseName
      }
    }
  }
"#;
