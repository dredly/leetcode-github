use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Submission {
    pub id: String,
    lang: String,
    pub statusDisplay: String,
    title: String,
    titleSlug: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]

pub struct SubmissionList {
    pub submissions: Vec<Submission>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct Data {
    pub submission_list: SubmissionList
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct question {
    questionId: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct submissionDetails {
    pub runtime: u32,
    pub runtimeDisplay: String,
    pub runtimePercentile: f32,
    pub memoryDisplay: String, 
    pub question: question,
    pub code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct SubmissionDetailsResponse {
    pub submission_details: submissionDetails
}