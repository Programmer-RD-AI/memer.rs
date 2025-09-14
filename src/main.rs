use async_openai::{
    Client,
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
};
use clap::Parser;
use dotenv::dotenv;

use crate::utils::{get_default_uuid_directory, load_prompt};
mod utils;

#[derive(Parser, Debug)]
#[command(name="memer", author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long, default_value_t=get_default_uuid_directory())]
    folder: String,
}

async fn generate_image(query: &str, dir: &str) -> Result<(), OpenAIError> {
    let client = Client::new();
    let request = CreateImageRequestArgs::default()
        .prompt(load_prompt(query))
        .response_format(ImageResponseFormat::B64Json)
        .size(ImageSize::S1024x1024)
        .model(ImageModel::DallE3)
        .build()?;
    let response = client.images().create(request).await?;
    response.save(dir).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();
    generate_image(&args.query, &args.folder).await.unwrap();
}
