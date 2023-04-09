use gql_client;
use serde::Serialize;
use std::collections::HashMap;
use std::{env, future};
use std::future::IntoFuture;
use std::iter::Filter;
use futures::future::join_all;

use crate::graphql_queries;
use crate::models::{Submission, SubmissionDetailsResponse, SubmissionListResponse, SubmissionDetails};
use crate::repo_builder;

pub const BASE_URL: &str = "https://leetcode.com";
pub const USER_AGENT: &str = "Mozilla/5.0 LeetCode API";

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

pub async fn get_csrf(client: &reqwest::Client) -> String {
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
        .split_once(";")
        .unwrap()
        .0
        .split_once("=")
        .unwrap()
        .1
        .to_owned()
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

async fn get_accepted_submissions(graphql_client: &gql_client::Client) -> Vec<Submission> {
    // Query for minimal data for now, just to test if graphql query works at all
    let vars = PaginationVars {
        offset: 0,
        limit: 20,
    };

    let accepted_submissions = graphql_client
        .query_with_vars::<SubmissionListResponse, PaginationVars>(
            graphql_queries::QUERY_SUBMISSION_LIST,
            vars,
        )
        .await
        .expect("graphql query error")
        .expect("error, submission list not found")
        .submission_list
        .submissions
        .into_iter()
        .filter(|s| s.status_display == "Accepted").collect::<Vec<_>>();

    accepted_submissions
}

async fn display_submission_details(
    graphql_client: &gql_client::Client,
    submission_id: u32,
    submission_title_slug: &str,
    output_dir: String,
) {
    let vars = QueryBySubmissionIdVars {
        submission_id: submission_id,
    };
    let submission_data = graphql_client
        .query_with_vars::<SubmissionDetailsResponse, QueryBySubmissionIdVars>(
            graphql_queries::QUERY_SUBMISSION_DETAILS,
            vars,
        )
        .await
        .expect("graphql query error")
        .expect("error, submission list not found");
    println!("submission_data: {:#?}", submission_data);
    // repo_builder::create_output(
    //     &output_dir,
    //     submission_id,
    //     submission_data
    //         .submission_details
    //         .question
    //         .question_id
    //         .parse()
    //         .unwrap(),
    //     submission_title_slug,
    //     &submission_data.submission_details.lang.name,
    //     &submission_data.submission_details.code,
    // );
}

pub async fn get_submission_details(
    graphql_client: &gql_client::Client,
    submission: Submission,
) -> SubmissionDetails {
    let submission_id = submission.id
    .parse::<u32>()
    .expect("could not parse submission id into integer");

    let vars = QueryBySubmissionIdVars {
        submission_id: submission_id,
    };
    graphql_client
        .query_with_vars::<SubmissionDetailsResponse, QueryBySubmissionIdVars>(
            graphql_queries::QUERY_SUBMISSION_DETAILS,
            vars,
        )
        .await
        .expect("graphql query error")
        .expect("error, submission list not found").submission_details
}


pub async fn get_all_submission_details(
    graphql_client: &gql_client::Client,
    output_dir: String,
) {
    let accepted_solution_details_futures = &get_accepted_submissions(graphql_client).await.into_iter()
        .map(|x| get_submission_details(graphql_client, x)).into_iter();

    let res = join_all(accepted_solution_details_futures);

}
