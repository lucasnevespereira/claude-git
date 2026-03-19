use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Request {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct Response {
    content: Option<Vec<ContentBlock>>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

#[derive(Deserialize)]
struct ApiError {
    message: String,
}

pub fn call(api_key: &str, model: &str, input: &str, prompt: &str) -> Result<String, String> {
    let body = Request {
        model: model.to_string(),
        max_tokens: 256,
        messages: vec![Message {
            role: "user".to_string(),
            content: format!("{input}\n\n{prompt}"),
        }],
    };

    let client = reqwest::blocking::Client::new();
    let resp: Response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("API request failed: {e}"))?
        .json()
        .map_err(|e| format!("failed to parse API response: {e}"))?;

    if let Some(err) = resp.error {
        return Err(format!("API error: {}", err.message));
    }

    resp.content
        .and_then(|c| c.into_iter().next())
        .map(|b| b.text)
        .ok_or_else(|| "empty response from API".to_string())
}
