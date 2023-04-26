use std::time::{SystemTime, UNIX_EPOCH};

use futures::Future;

const backoff_times: [f64; 7] = [0.5, 1.0, 1.5, 3.0, 5.0, 10.0, 20.0];

pub async fn retry_with_backoff<T, E, F>(func: fn() -> F) -> Result<T, E>
where
    F: Future<Output = Result<T, E>>,
{
    println!("enter function");

    let res = func().await;

    if res.is_ok() {
        return res;
    }

    for backoff_time in backoff_times {
        println!("using backoff_time={}", backoff_time);
        std::thread::sleep(std::time::Duration::from_millis(
            (backoff_time * 1000.0) as u64,
        ));
        let res_retried = func().await;
        if res_retried.is_ok() {
            return res_retried;
        }
    }
    res
}

async fn dummy_async_func() -> Result<u8, &'static str> {
    let parity = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % 2;

    if parity == 0 {
        Ok(19)
    } else {
        Err("STOP get help")
    }
}

#[tokio::test]
async fn test_retry() {
    retry_with_backoff(dummy_async_func)
        .await
        .expect("Retry backoff timed out");
}

async fn dummy_async_fn_with_arg(whatever: &str) -> Result<u8, &'static str> {
    let parity = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % 2;

    println!("{}", whatever);

    if parity == 0 {
        Ok(19)
    } else {
        Err("STOP get help")
    }
}

#[tokio::test]
async fn test_retry_with_arg() {
    retry_with_backoff(|| dummy_async_fn_with_arg("hello there"))
        .await
        .expect("Retry backoff timed out");
}
