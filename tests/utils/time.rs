use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryDecision, RetryPolicy, RetryTransientMiddleware};
use std::time::{Duration, SystemTime};

struct RetryEverySecond {}

impl RetryPolicy for RetryEverySecond {
    fn should_retry(&self, request_start_time: SystemTime, n_past_retries: u32) -> RetryDecision {
        if elapsed(&request_start_time) < Duration::from_secs(90) {
            RetryDecision::Retry {
                execute_after: request_start_time + Duration::from_secs(n_past_retries as u64 + 1),
            }
        } else {
            RetryDecision::DoNotRetry
        }
    }
}

fn elapsed(t: &SystemTime) -> Duration {
    t.elapsed().unwrap_or(Duration::from_secs(0))
}

pub fn wait(url: &str) {
    let client = ClientBuilder::new(Client::new())
        .with(RetryTransientMiddleware::new_with_policy(
            RetryEverySecond {},
        ))
        .build();
    let future = async {
        assert!(
            client.get(url).send().await.is_ok(),
            "could not reach {url} after waiting for 90 seconds"
        );
    };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);
}

pub fn measure<F, R>(label: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start_time = SystemTime::now();
    let result = f();
    println!("{label} (duration: {}s)", elapsed(&start_time).as_secs());
    result
}
