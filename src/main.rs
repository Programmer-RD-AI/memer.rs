use async_openai::{
    Client,
    error::OpenAIError,
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
};
use clap::Parser;
use dotenv::dotenv;
use uuid::Uuid;
mod utils;
fn get_default_uuid_file_name(file_type: Option<String>) -> String {
    format!(
        "{}.{}",
        Uuid::new_v4(),
        file_type.unwrap_or_else(|| "png".to_string()),
    )
}

#[derive(Parser, Debug)]
#[command(name="memer", version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long, default_value_t=get_default_uuid_file_name(None))]
    file: String,
}

async fn generate_image(query: &str, dir: &str) -> Result<(), OpenAIError> {
    let client = Client::new();
    let request = CreateImageRequestArgs::default()
        .prompt(query)
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
    generate_image(&args.query, &args.file).await.unwrap();
}
