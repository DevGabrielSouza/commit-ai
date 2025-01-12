use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

#[derive(Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: usize,
}

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub message: MessageResponse,
}

#[derive(Deserialize)]
pub struct MessageResponse {
    pub content: String,
}

pub fn create_request_body(diff: &str) -> OpenAIRequest {
    let instructions = r#"
    You are an AI that generates concise commit messages in the Conventional Commits format.
    Analyze the provided `git diff` and summarize the changes into a single commit message.
    Follow the Conventional Commits format with one of these prefixes: feat, fix, style, refactor, test, chore, or docs.
    Keep the message concise and relevant.

    Examples:
    1. feat: add user authentication feature
    2. fix: resolve crash on startup
    3. style: update button styling for consistency
    4. docs: add documentation for API endpoints

    Git diff:
    "#;

    OpenAIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: format!("{}\n\n{}", instructions, diff),
        }],
        max_tokens: 150,
    }
}



pub async fn generate_commit_message(diff: &str, api_key: &str) -> Result<String> {
    let client = Client::new();
    let body = create_request_body(diff);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        eprintln!("Error from OpenAI API: {} - {}", status, error_text);

        return Err(anyhow!("OpenAI API error: {} - {}", status, error_text));
    }

    let response_data: OpenAIResponse = response.json().await?;
    Ok(response_data.choices[0].message.content.clone())
}
