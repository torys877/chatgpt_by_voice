use std::include_bytes;

mod recorder;
mod ui;
mod openai;

pub const FILE_NAME: &str = "recorded.wav";

pub const OPENAI_PROMPT_TOKENS_COUNT: u32 = 100;
pub const OPENAI_MODEL: &str = "text-davinci-003";
pub const OPENAI_TEMPERATURE: u32 = 0;

fn main() {
    let _ = ui::render();
}

pub fn get_file_path() -> String {
    return format!("{}/{}", env!("CARGO_MANIFEST_DIR"), FILE_NAME);
}

pub fn get_font() -> Option<&'static [u8]> {
    Some(include_bytes!(
        "../assets/font/arial.ttf"
    ))
}
