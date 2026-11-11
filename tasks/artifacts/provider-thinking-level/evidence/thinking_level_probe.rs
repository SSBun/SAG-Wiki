use sag_core::{models, types::{ModelConfig, ReasoningEffort}};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key = std::env::var("OPENAI_API_KEY")?;
    let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build()?;
    let model = "gpt-6.1-sol";
    let metadata = client.get(format!("https://api.openai.com/v1/models/{model}")).bearer_auth(&key).timeout(std::time::Duration::from_secs(20)).send().await?;
    println!("model_metadata_status={}", metadata.status());
    anyhow::ensure!(metadata.status().is_success(), "Model metadata unavailable");
    let mut runtime = sag_protocol::runtime::RuntimeSettings::default();
    runtime.retry_count = 0;
    for effort in [ReasoningEffort::Low, ReasoningEffort::Medium, ReasoningEffort::High, ReasoningEffort::Xhigh, ReasoningEffort::Max] {
        let config = ModelConfig { url:"https://api.openai.com/v1".into(), model:model.into(), reasoning_effort:Some(effort), ..Default::default() };
        let mut plan = models::prepare_answer(&config, "Reply exactly OK.", &[], &[], 65536, 1024)?;
        plan.payload["store"] = serde_json::json!(false);
        plan.payload["max_completion_tokens"] = serde_json::json!(1024);
        anyhow::ensure!(plan.payload["reasoning_effort"] == serde_json::to_value(effort)?, "Incorrect effort mapping");
        let answer = models::answer_with_runtime(&client, &config, &key, &plan, 45, &runtime, &CancellationToken::new(), |_| {}).await?;
        anyhow::ensure!(!answer.text.trim().is_empty(), "Empty answer");
        println!("effort={:?} response={:?} input_tokens={:?} output_tokens={:?}", effort, answer.text, answer.input_tokens, answer.output_tokens);
    }
    let config = ModelConfig { url:"https://api.openai.com/v1".into(), model:model.into(), ..Default::default() };
    let plan = models::prepare_answer(&config, "OK", &[], &[], 65536, 0)?;
    anyhow::ensure!(plan.payload.get("reasoning_effort").is_none(), "Default must omit effort");
    println!("default_omits_effort=true");
    Ok(())
}
