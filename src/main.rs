use crate::utils::{get_default_uuid_directory, load_prompt, get_progress_bar, increment_progress_bar};
use async_openai::{
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
    Client,
};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use dotenv::dotenv;
use log::{debug, info, error};
mod config;
mod utils;

#[derive(Parser, Debug)]
#[command(name="memer", author, version, about, long_about=None)]
struct Args {
    #[arg(long = "query")]
    query: String,

    #[arg(short, long, default_value_t=get_default_uuid_directory())]
    folder: String,

    #[command(flatten)]
    verbosity: Verbosity,
}

async fn generate_image(query: &str, dir: &str) -> Result<String, OpenAIError> {
    info!("generating image for query: {}", query);
    increment_progress_bar().await;
    let client = Client::new();
    increment_progress_bar().await;
    debug!("creating image generation request");
    let request = CreateImageRequestArgs::default()
        .prompt(load_prompt(query))
        .response_format(ImageResponseFormat::B64Json)
        .size(ImageSize::S1024x1024)
        .model(ImageModel::DallE3)
        .build()?;
    increment_progress_bar().await;
    debug!("sending request to OpenAI");
    increment_progress_bar().await;
    let response = client.images().create(request).await?;
    increment_progress_bar().await;
    info!("saving image to directory: {}", dir);
    let saved_paths = response.save(dir).await?;
    increment_progress_bar().await;
    info!("image saved successfully");
    let saved_path = saved_paths.get(0).ok_or_else(|| error!("tet"));
    Ok(saved_path.unwrap().to_string_lossy().to_string())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Args::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    let genrated_path = generate_image(&cli.query, &cli.folder).await.unwrap();
    get_progress_bar()
        .await
        .finish_with_message(format!("Image Generated to: {}", genrated_path));
}
