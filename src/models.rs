use serde::Deserialize;

// --------- SubmissionList Resource ----------------------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Submission {
    pub id: String,
    lang: String,
    pub status_display: String,
    title: String,
    title_slug: String,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionList {
    pub submissions: Vec<Submission>,
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
    question_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionDetails {
    pub question: Question,
    pub code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionDetailsResponse {
    pub submission_details: SubmissionDetails,
}
