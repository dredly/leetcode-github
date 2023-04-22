use std::time::{SystemTime, UNIX_EPOCH};


const backoff_times: [f64; 7] = [0.5, 1.0, 1.5, 3.0, 5.0, 10.0, 20.0];


pub fn retry<T, E>(func: fn() -> Result<T, E>) -> Result<T, E> {
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
    const data: [u8; 5] = [1, 2, 3, 4, 5];

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

#[test]
fn test_retry() {
    retry(dummy_func);
}