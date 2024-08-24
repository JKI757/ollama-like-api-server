use serde::{Deserialize, Serialize};

// Shared Structs
#[derive(Deserialize, Serialize, Debug)]
struct GenerateRequest {
    model: String,
    prompt: Option<String>,
    suffix: Option<String>,
    images: Option<Vec<String>>,
    format: Option<String>,
    options: Option<serde_json::Value>,
    system: Option<String>,
    template: Option<String>,
    context: Option<Vec<u8>>,
    stream: Option<bool>,
    raw: Option<bool>,
    keep_alive: Option<String>,
}

#[derive(Serialize, Debug)]
struct GenerateResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    context: Option<Vec<u8>>,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_count: Option<u32>,
    prompt_eval_duration: Option<u64>,
    eval_count: Option<u32>,
    eval_duration: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: Option<bool>,
    tools: Option<Vec<Tool>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
    images: Option<Vec<String>>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Tool {
    r#type: String,
    function: ToolFunction,
}

#[derive(Deserialize, Serialize, Debug)]
struct ToolFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct ToolCall {
    function: ToolFunction,
}

#[derive(Serialize, Debug)]
struct ChatResponse {
    model: String,
    created_at: String,
    message: ChatMessage,
    done: bool,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_count: Option<u32>,
    prompt_eval_duration: Option<u64>,
    eval_count: Option<u32>,
    eval_duration: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
struct CreateModelRequest {
    name: String,
    modelfile: Option<String>,
    path: Option<String>,
    stream: Option<bool>,
}

#[derive(Serialize, Debug)]
struct ModelOperationResponse {
    status: String,
}

#[derive(Serialize, Debug)]
struct ListModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Serialize, Debug)]
struct ModelInfo {
    name: String,
    modified_at: String,
    size: u64,
    digest: String,
    details: ModelDetails,
}

#[derive(Serialize, Debug)]
struct ModelDetails {
    format: String,
    family: String,
    families: Option<Vec<String>>,
    parameter_size: String,
    quantization_level: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShowModelRequest {
    name: String,
    verbose: Option<bool>,
}

#[derive(Serialize, Debug)]
struct ShowModelResponse {
    modelfile: String,
    parameters: String,
    template: String,
    details: ModelDetails,
    model_info: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
struct CopyModelRequest {
    source: String,
    destination: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct DeleteModelRequest {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct PullModelRequest {
    name: String,
    insecure: Option<bool>,
    stream: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PushModelRequest {
    name: String,
    insecure: Option<bool>,
    stream: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
struct EmbedRequest {
    model: String,
    input: serde_json::Value,
    truncate: Option<bool>,
    options: Option<serde_json::Value>,
    keep_alive: Option<String>,
}

#[derive(Serialize, Debug)]
struct EmbedResponse {
    model: String,
    embeddings: Vec<Vec<f32>>,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u32,
}

#[derive(Serialize, Debug)]
struct RunningModelsResponse {
    models: Vec<RunningModelInfo>,
}

#[derive(Serialize, Debug)]
struct RunningModelInfo {
    name: String,
    model: String,
    size: u64,
    digest: String,
    details: ModelDetails,
    expires_at: String,
    size_vram: u64,
}


use warp::reply::json;
use warp::reply::Json;
use warp::http::StatusCode;
use serde_json::json;
use warp::Filter;

async fn handle_generate(body: Bytes) -> Result<Json, warp::Rejection> {
    let result: Result<GenerateRequest, _> = serde_json::from_slice(&body);

    match result {
        Ok(req) => {
            log::info!("Received generate request: {:?}", req);
            let response = GenerateResponse {
                model: req.model.clone(),
                created_at: chrono::Utc::now().to_rfc3339(),
                response: format!("Generated text for prompt: {:?}", req.prompt),
                done: true,
                context: Some(vec![1, 2, 3]),
                total_duration: Some(5000000000),
                load_duration: Some(1000000000),
                prompt_eval_count: Some(26),
                prompt_eval_duration: Some(200000000),
                eval_count: Some(100),
                eval_duration: Some(3000000000),
            };
            Ok(warp::reply::json(&response))
        }
        Err(_) => {
            log::warn!("Failed to parse request as JSON");
            let error_response = json!({
                "error": "Invalid JSON"
            });
            Ok(warp::reply::json(&error_response))
        }
    }
}

async fn handle_chat(req: ChatRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received chat request: {:?}", req);
    let response = ChatResponse {
        model: req.model.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
        message: ChatMessage {
            role: "assistant".to_string(),
            content: "This is a response from the chat model.".to_string(),
            images: None,
            tool_calls: None,
        },
        done: true,
        total_duration: Some(5000000000),
        load_duration: Some(1000000000),
        prompt_eval_count: Some(26),
        prompt_eval_duration: Some(200000000),
        eval_count: Some(100),
        eval_duration: Some(3000000000),
    };
    Ok(warp::reply::json(&response))
}

async fn handle_create_model(req: CreateModelRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received create model request: {:?}", req);
    let response = ModelOperationResponse {
        status: "success".to_string(),
    };
    Ok(warp::reply::json(&response))
}

async fn handle_list_models() -> Result<Json, warp::Rejection> {
    log::info!("Listing local models");
    let response = ListModelsResponse {
        models: vec![ModelInfo {
            name: "llama3:latest".to_string(),
            modified_at: chrono::Utc::now().to_rfc3339(),
            size: 3825819519,
            digest: "fe938a131f40e6f6d40083c9f0f430a515233eb2edaa6d72eb85c50d64f2300e".to_string(),
            details: ModelDetails {
                format: "gguf".to_string(),
                family: "llama".to_string(),
                families: None,
                parameter_size: "7B".to_string(),
                quantization_level: "Q4_0".to_string(),
            },
        }],
    };
    Ok(warp::reply::json(&response))
}

async fn handle_show_model(req: ShowModelRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received show model request: {:?}", req);
    let response = ShowModelResponse {
        modelfile: "# Modelfile".to_string(),
        parameters: "num_ctx 4096".to_string(),
        template: "{{ .Prompt }}".to_string(),
        details: ModelDetails {
            format: "gguf".to_string(),
            family: "llama".to_string(),
            families: Some(vec!["llama".to_string()]),
            parameter_size: "7B".to_string(),
            quantization_level: "Q4_0".to_string(),
        },
        model_info: serde_json::json!({
            "architecture": "llama",
            "parameter_count": 8030261248u64,
        }),
    };
    Ok(warp::reply::json(&response))
}

async fn handle_copy_model(req: CopyModelRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received copy model request: {:?}", req);
    let response = ModelOperationResponse {
        status: "success".to_string(),
    };
    Ok(warp::reply::json(&response))
}

async fn handle_delete_model(req: DeleteModelRequest) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Received delete model request: {:?}", req);
    Ok(warp::reply::with_status("Model deleted", StatusCode::OK))
}

async fn handle_pull_model(req: PullModelRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received pull model request: {:?}", req);
    let response = ModelOperationResponse {
        status: "success".to_string(),
    };
    Ok(warp::reply::json(&response))
}

async fn handle_push_model(req: PushModelRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received push model request: {:?}", req);
    let response = ModelOperationResponse {
        status: "success".to_string(),
    };
    Ok(warp::reply::json(&response))
}

async fn handle_embed(req: EmbedRequest) -> Result<Json, warp::Rejection> {
    log::info!("Received embed request: {:?}", req);
    let response = EmbedResponse {
        model: req.model.clone(),
        embeddings: vec![
            vec![0.010071029, -0.0017594862, 0.05007221, 0.04692972, 0.054916814],
            vec![-0.0098027075, 0.06042469, 0.025257962, -0.006364387, 0.07272725],
        ],
        total_duration: 14143917,
        load_duration: 1019500,
        prompt_eval_count: 8,
    };
    Ok(warp::reply::json(&response))
}

async fn handle_list_running_models() -> Result<Json, warp::Rejection> {
    log::info!("Listing running models");
    let response = RunningModelsResponse {
        models: vec![RunningModelInfo {
            name: "mistral:latest".to_string(),
            model: "mistral:latest".to_string(),
            size: 5137025024,
            digest: "2ae6f6dd7a3dd734790bbbf58b8909a606e0e7e97e94b7604e0aa7ae4490e6d8".to_string(),
            details: ModelDetails {
                format: "gguf".to_string(),
                family: "llama".to_string(),
                families: Some(vec!["llama".to_string()]),
                parameter_size: "7.2B".to_string(),
                quantization_level: "Q4_0".to_string(),
            },
            expires_at: chrono::Utc::now().to_rfc3339(),
            size_vram: 5137025024,
        }],
    };
    Ok(warp::reply::json(&response))
}

use serde_json::Value;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    env_logger::init();

    let generate_route = warp::path("api")
        .and(warp::path("generate"))
        .and(warp::post())
        .and(warp::body::bytes())
        .and_then(handle_generate);

    let chat_route = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_chat);

    let create_model_route = warp::path("api")
        .and(warp::path("create"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_create_model);

    let list_models_route = warp::path("api")
        .and(warp::path("tags"))
        .and(warp::get())
        .and_then(handle_list_models);

    let show_model_route = warp::path("api")
        .and(warp::path("show"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_show_model);

    let copy_model_route = warp::path("api")
        .and(warp::path("copy"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_copy_model);

    let delete_model_route = warp::path("api")
        .and(warp::path("delete"))
        .and(warp::delete())
        .and(warp::body::json())
        .and_then(handle_delete_model);

    let pull_model_route = warp::path("api")
        .and(warp::path("pull"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_pull_model);

    let push_model_route = warp::path("api")
        .and(warp::path("push"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_push_model);

    let embed_route = warp::path("api")
        .and(warp::path("embed"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_embed);

    let list_running_models_route = warp::path("api")
        .and(warp::path("ps"))
        .and(warp::get())
        .and_then(handle_list_running_models);

    warp::serve(
        generate_route
            .or(chat_route)
            .or(create_model_route)
            .or(list_models_route)
            .or(show_model_route)
            .or(copy_model_route)
            .or(delete_model_route)
            .or(pull_model_route)
            .or(push_model_route)
            .or(embed_route)
            .or(list_running_models_route),
    )
    .run(([127, 0, 0, 1], 11434))
    .await;
}