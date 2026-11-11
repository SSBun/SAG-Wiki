use sag_core::{models, types::{ModelConfig, ModelProfile, ReasoningEffort}};
fn main() -> anyhow::Result<()> {
    for effort in [None, Some(ReasoningEffort::Low), Some(ReasoningEffort::Medium), Some(ReasoningEffort::High), Some(ReasoningEffort::Xhigh)] {
        let profile: ModelProfile = serde_json::from_value(serde_json::json!({"reasoning_effort":effort}))?;
        let config = ModelConfig {url:"https://api.openai.com/v1".into(),model:"gpt-6.1-sol".into(),reasoning_effort:profile.reasoning_effort,..Default::default()};
        let plan = models::prepare_answer(&config, "Reply OK", &[], &[], 65536, 0)?;
        println!("profile={} request_effort={:?}", serde_json::to_string(&profile)?, plan.payload.get("reasoning_effort"));
    }
    Ok(())
}
