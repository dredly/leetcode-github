use serde::{Deserialize, Serialize};
use gql_client;
use std::collections::HashMap;
use std::env;

pub const BASE_URL: &str = "https://leetcode.com";
pub const USER_AGENT: &str = "Mozilla/5.0 LeetCode API";

#[derive(Serialize)]
struct Vars {
    offset: u32,
    limit: u32,
}

#[derive(Deserialize)]
struct Submission {
    id: u32,
    lang: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmissionList {
    submissions: Vec<Submission>
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

pub async fn display_submissions() {
    let leetcode_session_cookie = env::var("LEETCODE_SESSION_COOKIE");
    match leetcode_session_cookie {
        Ok(leetcode_session_cookie) => {
            println!("found cookie {}", leetcode_session_cookie);
            let client = reqwest::Client::new();
            let csrf_token = get_csrf(&client).await;

            let mut headers = HashMap::new();
            headers.insert("content-type", "application/json");
            headers.insert("origin", BASE_URL);
            headers.insert("referer", BASE_URL);
            headers.insert("cookie", &leetcode_session_cookie);
            headers.insert("x-csrftoken", &csrf_token);
            headers.insert("user-agent", USER_AGENT);

            let graphql_client =
                gql_client::Client::new_with_headers(String::from(BASE_URL) + "/graphql", headers);

            // Query for minimal data for now, just to test if graphql query works at all
            let query = r#"
                query ($offset: Int!, $limit: Int!) {
                    submissionList(offset: $offset, limit: $limit) {
                        submissions { id lang }
                    }
                }
            "#;

            let vars = Vars {
                offset: 0,
                limit: 20,
            };

            let submission_list = graphql_client.query_with_vars::<SubmissionList, Vars>(query, vars)
                .await.expect("graphql query error").expect("error, submission list not found");

            let submissions = submission_list.submissions;

            println!("all good");
        }
        Err(_) => println!("Didnt find env var"),
    }
}