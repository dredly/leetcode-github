use std::time::{SystemTime, UNIX_EPOCH};

use futures::Future;


const backoff_times: [f64; 7] = [0.5, 1.0, 1.5, 3.0, 5.0, 10.0, 20.0];

// TODO MAKE ASYNC PRETTY PLEASE
pub fn retry<T, E>(func: fn() -> Future<Result<T, E>>) -> Result<T, E> {
    println!("enter function");

    let mut res = func();

    if res.is_ok() {
        return res;
    }

    for backoff_time in backoff_times {
        println!("using backoff_time={}", backoff_time);
        std::thread::sleep(std::time::Duration::from_millis((backoff_time * 1000.0) as u64));
        res = func();
        if res.is_ok() {
            return res;
        }
    }
    res
}

fn dummy_func() -> Result<u8, &'static str> {
    let parity = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() % 2;

    if parity == 0 {
        Ok(19)
    }
    else {
        Err("STOP get help")
    }
}

fn dummy_func2(whatever: &str) -> Result<u8, &'static str> {
    let parity = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() % 2;

    println!("{}", whatever);

    if parity == 0{
        Ok(19)
    }
    else {
        Err("STOP get help")
    }
}

#[test]
fn test_retry() {
    retry(dummy_func);
}

#[test]
fn test_retry_w_args() {
    fn dummy_func_callback() -> Result<u8, &'static str> {
        dummy_func2("hardcoded thing")
    }
    retry(dummy_func_callback);
}