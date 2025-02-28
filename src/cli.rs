use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize)]
pub struct Args {
    #[arg(long, short = 'a', default_value_t = String::from("llama"))]
    model_architecture: String,
    #[arg(long = "model")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    model_path: Option<PathBuf>,
    #[arg(long, short = 'v')]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokenizer_path: Option<PathBuf>,
    #[arg(long, short = 'r')]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tokenizer_repository: Option<String>,

    #[arg(long, short = 'H', default_value_t = String::from("0.0.0.0"))]
    pub host: String,
    #[arg(long, short, default_value_t = 3000)]
    pub port: usize,
    #[arg(long, short = 'm', default_value_t = true)]
    pub prefer_mmap: bool,
    #[arg(long, short, default_value_t = 2048)]
    pub context_size: usize,
    #[arg(long, short)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub lora_adapters: Option<Vec<PathBuf>>,
    #[arg(long, short, default_value_t = true)]
    pub use_gpu: bool,
    #[arg(long, short)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub gpu_layers: Option<usize>,
    #[arg(long, short)]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub zipkin_endpoint: Option<String>,
}
