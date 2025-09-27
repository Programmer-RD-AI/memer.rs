use indicatif::ProgressBar;
use tokio::sync::OnceCell;

pub const BASE_PROMPT_FILE_PATH: &str = "src/prompts/base.jinja";
pub static PROGRESS_BAR: OnceCell<ProgressBar> = OnceCell::const_new();

