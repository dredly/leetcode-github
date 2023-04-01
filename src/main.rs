use clap::Parser;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String
}

const BASE_URL: &str = "https://leetcode.com";
const USER_AGENT: &str = "Mozilla/5.0 LeetCode API";


async fn get_csrf(client: &reqwest::Client) -> String {
    client.get(BASE_URL)
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
        .unwrap().0
        .split_once("=")
        .unwrap().1
        .to_owned()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let leetcode_session_cookie = env::var("LEETCODE_SESSION_COOKIE");
    match leetcode_session_cookie {
        Ok(leetcode_session_cookie) => {
            println!("found cookie {}", leetcode_session_cookie);
            let client = reqwest::Client::new();
            let csrf_token =  get_csrf(&client).await;
            let resp = client.post(String::from(BASE_URL) + "/graphql")
                .body("")
                .header("content-type", "application/json")
                .header("origin", BASE_URL)
                .header("referer", BASE_URL)
                .header("cookie", "chocolate")
                .header("x-csrftoken", &csrf_token)
                .header("user-agent", USER_AGENT)
                .send()
                .await
                .expect("Error sending request");
            println!("{}", resp.status());
            println!("the token is {}", csrf_token);
        }, 
        Err(_) => println!("Didnt find env var")
    }

    println!("Selected output path {}", args.output);
}
