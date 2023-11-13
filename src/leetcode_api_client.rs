use futures::{stream, StreamExt};
use tokio::time::sleep;
use std::time::Duration;
use gql_client;
use serde::Serialize;
use std::collections::HashMap;
use std::env;

use crate::graphql_queries;
use crate::models::{
    EnhancedSubmissionDetails, Submission, SubmissionDetailsResponse, SubmissionListResponse,
};

const PAGINATION_LIMIT: u32 = 20;
const CONCURRENT_REQUESTS: usize = 1;
const BASE_URL: &str = "https://leetcode.com";
const USER_AGENT: &str = "Mozilla/5.0 LeetCode API";

#[derive(Serialize)]
struct PaginationVars {
    offset: u32,
    limit: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct QueryBySubmissionIdVars {
    submission_id: u32,
}

pub async fn get_graphql_client() -> gql_client::Client {
    let leetcode_session_cookie = env::var("LEETCODE_SESSION_COOKIE")
        .expect("Could not find env var LEETCODE_SESSION_COOKIE");
    let client = reqwest::Client::new();
    let csrf_token = get_csrf(&client).await;

    let cookie_value = format!(
        "csrftoken={}; LEETCODE_SESSION={};",
        csrf_token.clone(),
        leetcode_session_cookie
    );

    let mut headers = HashMap::new();
    headers.insert("content-type", "application/json");
    headers.insert("origin", BASE_URL);
    headers.insert("referer", BASE_URL);
    headers.insert("cookie", &cookie_value);
    headers.insert("x-csrftoken", csrf_token.as_str());
    headers.insert("user-agent", USER_AGENT);

    gql_client::Client::new_with_headers(String::from(BASE_URL) + "/graphql", headers)
}

pub async fn get_all_submission_details(
    graphql_client: &gql_client::Client,
    last_submission: Option<usize>,
) -> Vec<EnhancedSubmissionDetails> {
    stream::iter(get_all_accepted_submissions(graphql_client, last_submission).await)
        .map(|submission| get_enhanced_submission_details(graphql_client, submission))
        .buffer_unordered(CONCURRENT_REQUESTS)
        .collect::<Vec<_>>()
        .await
}

async fn get_csrf(client: &reqwest::Client) -> String {
    client
        .get(BASE_URL)
        .header("user-agent", USER_AGENT)
        .send()
        .await
        .expect("Error sending request")
        .headers()
        .get("set-cookie")
        .expect("Could not find set cookie header")
        .to_str()
        .expect("Error reading cookie as string")
        .split_once(';')
        .unwrap()
        .0
        .split_once('=')
        .unwrap()
        .1
        .to_owned()
}

async fn get_accepted_submissions(
    graphql_client: &gql_client::Client,
    offset: u32,
    last_submission: Option<usize>,
) -> (Vec<Submission>, bool) {
    let vars: PaginationVars = PaginationVars {
        offset,
        limit: PAGINATION_LIMIT,
    };
    let mut should_continue;
    let submissions_response = graphql_client
        .query_with_vars::<SubmissionListResponse, PaginationVars>(
            graphql_queries::QUERY_SUBMISSION_LIST,
            vars,
        )
        .await
        .expect("graphql query error")
        .expect("error, submission list not found");

    println!("{submissions_response:?}");

    let accepted_submissions = submissions_response
        .submission_list
        .submissions
        .into_iter()
        .filter(|s| s.status_display == "Accepted")
        .collect::<Vec<_>>();

    should_continue = submissions_response.submission_list.has_next;

    if let (Some(last_submission_locally_id), Some(last_accepted_submission)) =
        (last_submission, accepted_submissions.last())
    {
        if let Ok(last_accepted_submission_id) = last_accepted_submission.id.trim().parse::<usize>()
        {
            if last_submission_locally_id >= last_accepted_submission_id {
                should_continue = false;
            }
        } else {
            println!("last_accepted_submission={last_accepted_submission:?} does not have an id that can be parsed as a usize");
        }
    }

    let unseen_accepted_submissions: Vec<_> = accepted_submissions
        .iter()
        .filter(|s| {
            last_submission.is_none()
                || last_submission.unwrap()
                    < s.id
                        .trim()
                        .parse::<usize>()
                        .expect("cannot read id as usize")
        })
        .cloned()
        .collect();

    (unseen_accepted_submissions, should_continue)
}

async fn get_all_accepted_submissions(
    graphql_client: &gql_client::Client,
    last_submission: Option<usize>,
) -> Vec<Submission> {
    let mut all_accepted_submission: Vec<Submission> = vec![];
    let mut offset: u32 = 0;
    let mut has_submissions_left = true;

    // accepted_submission
    while has_submissions_left {
        let (accepted_submission, has_next) =
            get_accepted_submissions(graphql_client, offset, last_submission).await;
        offset += PAGINATION_LIMIT;
        has_submissions_left = has_next;
        println!("has_next={:?}", has_next);

        println!("accepted_submission={:?}", accepted_submission.len());
        all_accepted_submission.extend(accepted_submission);
    }
    println!(
        "all_accepted_submission={:?}",
        all_accepted_submission.len()
    );
    all_accepted_submission
}

async fn try_to_get_submission_details(
    graphql_client: &gql_client::Client,
    vars: QueryBySubmissionIdVars,
) -> Result<Option<SubmissionDetailsResponse>, gql_client::GraphQLError> {
    graphql_client
        .query_with_vars::<SubmissionDetailsResponse, QueryBySubmissionIdVars>(
            graphql_queries::QUERY_SUBMISSION_DETAILS,
            vars,
        )
        .await
}

async fn get_enhanced_submission_details(
    graphql_client: &gql_client::Client,
    submission: Submission,
) -> EnhancedSubmissionDetails {
    let submission_id = submission
        .id
        .parse::<u32>()
        .expect("could not parse submission id into integer");

    let vars = QueryBySubmissionIdVars { submission_id };

    println!("get_enhanced_submission_details: submission_id={submission_id}");
    sleep(Duration::from_secs(2)).await;

    let submission_details = try_to_get_submission_details(graphql_client, vars)
        .await
        .expect(&format!("graphql query error {submission_id}"))
        .expect("error, submission list not found")
        .submission_details;

    EnhancedSubmissionDetails {
        submission_details,
        title_slug: submission.title_slug,
        submission_id: submission.id,
    }
}
