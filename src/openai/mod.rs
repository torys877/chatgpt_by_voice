use reqwest;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::blocking::multipart::Form;
use core::result::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map, Number};

#[derive(Serialize, Deserialize, Debug)]
pub  struct OpenAiSpeechToTextRes {
    pub text: Option<String>,
    pub error: Option<OpenAiError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiCompletionRes {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Option<Vec<OpenAiChoices>>,
    pub usage: Option<OpenAiUsage>,
    pub error: Option<OpenAiError>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiChoices {
    pub text: Option<String>,
    pub index: Option<u64>,
    pub logprobs: Option<String>,
    pub finish_reason: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiUsage {
    pub prompt_tokens: Option<u64>,
    pub completion_tokens: Option<u64>,
    pub total_tokens: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAiError {
    pub message: Option<String>,
    pub r#type: Option<String>,
    pub param: Option<String>,
    pub code: Option<String>,
}

pub fn send_speech_to_text(file_path: String, api_key: String) -> Result<OpenAiSpeechToTextRes, serde_json::Error>
{
    let api_key_header = format!("Bearer {}", api_key);
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(api_key_header.as_str()).unwrap());
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("multipart/form-data"));

    let form = Form::new()
        .text("model", "whisper-1")
        .file("file", file_path)
        .unwrap();

    let client = reqwest::blocking::ClientBuilder::new().default_headers(headers).build().unwrap();

    let res = client.post("https://api.openai.com/v1/audio/transcriptions")
        .multipart(form)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let open_ai_res: OpenAiSpeechToTextRes = serde_json::from_str::<OpenAiSpeechToTextRes>(res.as_str()).unwrap();

    Ok(open_ai_res)
}

pub fn send_completion(question: String, api_key: String) -> Result<OpenAiCompletionRes, serde_json::Error>
{
    println!("QUESTION ==> {}", question);

    let mut body_request = Map::new();
    body_request.insert("model".to_string(), Value::String(String::from(super::OPENAI_MODEL)));
    body_request.insert("prompt".to_string(), Value::String(question));
    body_request.insert("max_tokens".to_string(), Value::Number(Number::from(super::OPENAI_PROMPT_TOKENS_COUNT)));
    body_request.insert("temperature".to_string(), Value::Number(Number::from(super::OPENAI_TEMPERATURE)));

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let api_key_header = format!("Bearer {}", api_key);
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(api_key_header.as_str()).unwrap());

    let client = reqwest::blocking::ClientBuilder::new().default_headers(headers).build().unwrap();

    let res = client.post("https://api.openai.com/v1/completions")
        .json(&body_request)
        .send()
        .unwrap()
        .text()
        .unwrap();

    println!("{}", res);

    let open_ai_res: OpenAiCompletionRes = serde_json::from_str::<OpenAiCompletionRes>(res.as_str()).unwrap();

    Ok(open_ai_res)
}
