//! Streaming response support

use futures::StreamExt;
use matw_ai::{AIProvider, Chunk, CompletionRequest};
use matw_core::{Message, Session};

pub async fn process_streaming<P: AIProvider>(
    provider: &P,
    session: &mut Session,
    on_delta: impl Fn(String),
) -> Result<(), super::agent::AgentError> {
    let request = CompletionRequest {
        messages: session.messages().to_vec(),
        tools: vec![],
        model: "default".to_string(),
        max_tokens: Some(4096),
        temperature: Some(0.7),
        system_prompt: None,
    };

    let stream = provider
        .stream_completion(request)
        .await
        .map_err(|e| super::agent::AgentError::AIProvider(e.to_string()))?;

    let mut response_text = String::new();

    futures::pin_mut!(stream);

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result
            .map_err(|e| super::agent::AgentError::AIProvider(e.to_string()))?;

        match chunk {
            Chunk::Delta(text) => {
                on_delta(text.clone());
                response_text.push_str(&text);
            }
            Chunk::Done => break,
            _ => {}
        }
    }

    // Add final message
    session.add_message(Message::new_assistant(response_text));

    Ok(())
}
