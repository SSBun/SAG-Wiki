use sag_core::{models, types::{ModelConfig, ReasoningEffort}};
use std::io::{Read, Write};

async fn inspect(host: &str, model: &str, effort: Option<ReasoningEffort>) -> anyhow::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let server = std::thread::spawn(move || -> anyhow::Result<serde_json::Value> {
        let (mut socket, _) = listener.accept()?;
        socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
        let mut bytes = vec![];
        let (offset, length) = loop {
            let mut part = [0; 4096];
            let count = socket.read(&mut part)?;
            anyhow::ensure!(count > 0, "Incomplete headers");
            bytes.extend_from_slice(&part[..count]);
            if let Some(index) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
                let headers = String::from_utf8_lossy(&bytes[..index]);
                let length = headers.lines().find_map(|line| line.to_ascii_lowercase().strip_prefix("content-length:").map(|size| size.trim().parse::<usize>())).transpose()?.unwrap_or(0);
                break (index + 4, length);
            }
        };
        while bytes.len() < offset + length {
            let mut part = [0; 4096];
            let count = socket.read(&mut part)?;
            anyhow::ensure!(count > 0, "Incomplete body");
            bytes.extend_from_slice(&part[..count]);
        }
        let payload = serde_json::from_slice(&bytes[offset..offset+length])?;
        let body = r#"{"choices":[{"finish_reason":"stop","message":{"content":"OK"}}]}"#;
        write!(socket, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body)?;
        Ok(payload)
    });
    let client = reqwest::Client::builder().no_proxy().resolve(host, address).redirect(reqwest::redirect::Policy::none()).build()?;
    let config = ModelConfig {url:format!("http://{host}:{}", address.port()),model:model.into(),reasoning_effort:effort,..Default::default()};
    let result = models::test_connection(&client, "answer", &config, "synthetic-key", 5).await?;
    let payload = server.join().unwrap()?;
    println!("model={model} effort={effort:?} result={result} payload={payload}");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    inspect("api.302.ai", "qwen3.6-flash", Some(ReasoningEffort::High)).await?;
    inspect("api.302.ai", "qwen3.6-flash", None).await?;
    inspect("api.deepseek.com", "deepseek-flash", Some(ReasoningEffort::High)).await?;
    inspect("api.deepseek.com", "deepseek-flash", None).await?;
    let config = ModelConfig {url:"http://127.0.0.1:1".into(),model:"unknown-model".into(),reasoning_effort:Some(ReasoningEffort::High),..Default::default()};
    match models::test_connection(&reqwest::Client::new(), "answer", &config, "synthetic-key", 1).await {
        Ok(_) => anyhow::bail!("Unsupported effort unexpectedly succeeded"),
        Err(error) => println!("unsupported_effort_error={error}"),
    }
    Ok(())
}
