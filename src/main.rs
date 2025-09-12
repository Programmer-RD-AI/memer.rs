use async_openai::{
    Client,
    types::{CreateImageRequestArgs, ImageResponseFormat, ImageSize},
};
use clap::Parser;
use std::error::Error;
use uuid::Uuid;

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

async fn generate_image(query: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateImageRequestArgs::default()
        .prompt(query)
        .response_format(ImageResponseFormat::Url)
        .size(ImageSize::S1024x1024)
        .user("memer")
        .build()?;
    let response = client.images().create(request).await?;
    response.save(dir).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    generate_image(&args.query, &args.file).await.unwrap();
}
