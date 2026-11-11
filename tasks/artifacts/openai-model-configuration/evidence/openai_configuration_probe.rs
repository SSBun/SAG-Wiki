use sag_core::{models, types::{Candidate, DecisionProtocol, ModelConfig}};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let key = std::env::var("OPENAI_API_KEY")?;
    let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build()?;
    let answer = ModelConfig { url: "https://api.openai.com/v1".into(), model: "gpt-4.1-mini".into(), ..Default::default() };
    println!("chat_connection={}", models::test_connection(&client, "answer", &answer, &key, 40).await?);
    let decisions = ModelConfig { model: "gpt-6-luna".into(), decision_protocol: DecisionProtocol::Openai, ..answer };
    println!("decision_connection={}", models::test_connection(&client, "jev", &decisions, &key, 40).await?);
    let candidates = [(1, "The project uses SQLite for its local database."), (2, "The sky is blue.")].into_iter().map(|(id, body)| Candidate {
        previous_score: None, id, document_id: id, path: format!("/synthetic/{id}.md"), title: format!("Synthetic {id}"), heading: String::new(), content: body.into(), start_line: 1, end_line: 1, source_ranges: vec![], source_spans: vec![], bm25: 0.0, score: 0.0, order: id as usize, hash: String::new(),
    }).collect();
    let mut runtime = sag_protocol::runtime::RuntimeSettings::default();
    runtime.retry_count = 0;
    let scores = models::score_with_runtime(&client, &decisions, &key, "Which database does the project use?", candidates, &[], 40, &runtime, &CancellationToken::new()).await?;
    println!("candidate_scores={:?}", scores.iter().map(|candidate| (candidate.id, candidate.score)).collect::<Vec<_>>());
    anyhow::ensure!(scores.len() == 2 && scores[0].score > scores[1].score, "Unexpected synthetic ranking");
    Ok(())
}
