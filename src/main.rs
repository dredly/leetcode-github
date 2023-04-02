use clap::Parser;
use gql_client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String,
}

const BASE_URL: &str = "https://leetcode.com";
const USER_AGENT: &str = "Mozilla/5.0 LeetCode API";

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
        .split_once(";")
        .unwrap()
        .0
        .split_once("=")
        .unwrap()
        .1
        .to_owned()
}

#[derive(Serialize)]
struct Vars {
    offset: u32,
    limit: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmissionList {
    has_next: Option<String>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
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
                        hasNext
                    }
                }
            "#;

            let vars = Vars {
                offset: 0,
                limit: 20,
            };

            let submission_list = graphql_client.query_with_vars::<SubmissionList, Vars>(query, vars)
                .await.expect("graphql query error").expect("error, submission list not found");

            println!("hasNext: {}", submission_list.has_next.unwrap_or("null".to_string()));
        }
        Err(_) => println!("Didnt find env var"),
    }

    println!("Selected output path {}", args.output);
}
