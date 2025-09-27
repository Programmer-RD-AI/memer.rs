use crate::utils::{get_default_uuid_directory, load_prompt};
use async_openai::{
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
    Client,
};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use dotenv::dotenv;
use log::{debug, info};
mod utils;

#[derive(Parser, Debug)]
#[command(name="memer", author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long, default_value_t=get_default_uuid_directory())]
    folder: String,

    #[command(flatten)]
    verbosity: Verbosity,
}

async fn generate_image(query: &str, dir: &str) -> Result<(), OpenAIError> {
    info!("generating image for query: {}", query);
    let client = Client::new();
    debug!("creating image generation request");
    let request = CreateImageRequestArgs::default()
        .prompt(load_prompt(query))
        .response_format(ImageResponseFormat::B64Json)
        .size(ImageSize::S1024x1024)
        .model(ImageModel::DallE3)
        .build()?;
    debug!("sending request to OpenAI");
    let response = client.images().create(request).await?;
    info!("saving image to directory: {}", dir);
    response.save(dir).await?;
    info!("image saved successfully");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Args::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbosity.log_level_filter())
        .init();
    generate_image(&cli.query, &cli.folder).await.unwrap();
}
