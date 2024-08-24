use warp::Filter;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use log::{info, error, debug};
use uuid::uuid;
use std::env;

#[derive(Deserialize)]
struct OllamaCompletionRequest {
    model: String,
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<u32>,
    stream: Option<bool>,
}

#[derive(Debug)]
#[derive(Serialize)]
struct OllamaCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug)]
#[derive(Serialize)]
struct Choice {
    text: String,
    index: u32,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Create a Warp filter for the completion endpoint
    let completion_route = warp::path("v1")
        .and(warp::path("completions"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_ollama_completion_request);

    info!("Starting the Ollama-like server at 127.0.0.1:11434");

    // Start the Warp server
    warp::serve(completion_route)
        .run(([127, 0, 0, 1], 11434))
        .await;
}

async fn handle_ollama_completion_request(ollama_request: OllamaCompletionRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let api_endpoint = env::var("API_ENDPOINT").unwrap_or("http://example.com/api".to_string());

    // Log the incoming request
    info!("Received completion request for model: {}", ollama_request.model);
    debug!("Request prompt: {}", ollama_request.prompt);

    // Transform the request into the API request format
    let api_request = ApiRequest {
        query: ollama_request.prompt.clone(),
        max_tokens: ollama_request.max_tokens,
        temperature: ollama_request.temperature,
        top_p: ollama_request.top_p,
    };

    // Log the request being sent to the other API
    info!("Sending request to API: {}", api_endpoint);
    debug!("API request body: {:?}", api_request);

    // Make the request to the other API
    let client = Client::new();
    let response = client.post(&api_endpoint)
        .json(&api_request)
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Log the successful response status
                info!("Received successful response from API");

                // Log the raw response body
                let raw_body = resp.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
                debug!("Raw API response body: {}", raw_body);

                // Deserialize the API response
                let api_response: ApiResponse = serde_json::from_str(&raw_body).unwrap_or_else(|_| ApiResponse {
                    completion: "Failed to parse API response".to_string(),
                });

                // Log the parsed API response
                debug!("Parsed API response: {:?}", api_response);

                // Create Ollama-like completion response
                let ollama_response = OllamaCompletionResponse {
                    id: uuid::Uuid::new_v4().to_string(),
                    object: "text_completion".to_string(),
                    created: chrono::Utc::now().timestamp() as u64,
                    model: ollama_request.model.clone(),
                    choices: vec![
                        Choice {
                            text: api_response.completion.clone(),
                            index: 0,
                        },
                    ],
                };

                // Log the response being sent back to the user
                debug!("Response to user: {:?}", ollama_response);

                // Respond to the user
                Ok(warp::reply::json(&ollama_response))
            } else {
                error!("Failed to get a successful response from the API. Status: {}", resp.status());
                Err(warp::reject::not_found())
            }
        }
        Err(e) => {
            error!("Failed to send request to the API: {}", e);
            Err(warp::reject::not_found())
        }
    }
}

#[derive(Debug)]
#[derive(Serialize)]
struct ApiRequest {
    query: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    completion: String,
}