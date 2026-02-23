use std::time::Instant;
use myid::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::from_env(None)?;
    let client = MyIdClient::new(cfg)?;

    let n: usize = std::env::var("N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(500);

    let started = Instant::now();
    let mut tasks = Vec::with_capacity(n);

    for _ in 0..n {
        let c = client.clone();
        tasks.push(tokio::spawn(async move { c.get_token().await }));
    }

    let mut ok = 0usize;
    let mut err = 0usize;

    for t in tasks {
        match t.await? {
            Ok(_) => ok += 1,
            Err(e) => {
                err += 1;
                eprintln!("get_token error: {e}");
            }
        }
    }

    println!(
        "N={n} ok={ok} err={err} elapsed_ms={}",
        started.elapsed().as_millis()
    );

    Ok(())
}
