use serde::Deserialize;

// --------- SubmissionList Resource ----------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone)]
pub struct Submission {
    pub id: String,
    pub status_display: String,
    pub title_slug: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionList {
    pub submissions: Vec<Submission>,
    pub has_next: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionListResponse {
    pub submission_list: SubmissionList,
}

// --------- SubmissionDetails Resource ----------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Question {
    pub question_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Languages {
    pub name: String,
    pub verbose_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionDetails {
    pub question: Question,
    pub code: String,
    pub lang: Languages,
}

pub struct EnhancedSubmissionDetails {
    pub submission_details: SubmissionDetails,
    pub title_slug: String,
    pub submission_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionDetailsResponse {
    pub submission_details: SubmissionDetails,
}
