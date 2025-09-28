use anyhow::{anyhow, Context, Result};
use async_openai::{
    types::{CreateImageRequestArgs, ImageModel, ImageResponseFormat, ImageSize},
    Client,
};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use dotenv::dotenv;
use log::{debug, error, info};
use memer_rs::utils::{
    get_default_uuid_directory, get_progress_bar, increment_progress_bar, load_prompt,
};

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

async fn generate_image(query: &str, dir: &str) -> Result<String> {
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
        .build()
        .context("Error Building Request")?;
    increment_progress_bar().await;
    debug!("sending request to OpenAI");
    increment_progress_bar().await;
    let response = client
        .images()
        .create(request)
        .await
        .context("Error Getting Response")?;
    increment_progress_bar().await;
    info!("saving image to directory: {}", dir);
    let saved_paths = response.save(dir).await.context("Error Saving")?;
    increment_progress_bar().await;
    info!("image saved successfully");
    let saved_path = saved_paths.get(0).ok_or_else(|| {
        error!("no saved path found");
        anyhow!("no saved path found")
    })?;
    Ok(saved_path.to_string_lossy().to_string())
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
