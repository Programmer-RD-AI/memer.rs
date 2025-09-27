//! why Box<str>? the options that we have is either String (which would mean it will be mutable
//! (which we do not indeded to do), owned, etc...) or a Arc or Rc, as they seem unrequired for this setup at the moment, i have
//! utilized a Box<str>

use std::fs;

use crate::config::{BASE_PROMPT_FILE_PATH, PROGRESS_BAR};
use indicatif::ProgressBar;
use log::{debug, info};
use minijinja::{context, Environment};
use uuid::Uuid;

pub fn load_prompt(query: &str) -> Box<str> {
    info!("loading prompt for query: {}", query);
    let mut env = Environment::new();
    let loaded_prompt: &str = &load_file(None);
    env.add_template("memer", loaded_prompt).unwrap();
    let template = env.get_template("memer").unwrap();
    info!("rendering template");
    template
        .render(context!(query => query))
        .unwrap()
        .to_owned()
        .into_boxed_str()
}

fn load_file(path: Option<&str>) -> Box<str> {
    let file_path = path.unwrap_or(BASE_PROMPT_FILE_PATH);
    info!("loading file from path: {}", file_path);
    fs::read_to_string(file_path)
        .unwrap()
        .to_owned()
        .into_boxed_str()
}

pub fn get_default_uuid_directory() -> String {
    let uuid = Uuid::new_v4().to_string();
    debug!("generated uuid for directory: {}", &uuid);
    uuid
}

pub async fn get_progress_bar() -> &'static ProgressBar {
    PROGRESS_BAR
        .get_or_init(|| async { ProgressBar::new(6) })
        .await
}

pub async fn increment_progress_bar() {
    let progres_bar = get_progress_bar().await;
    progres_bar.inc(1);
}
