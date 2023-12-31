use anyhow::Result;
use arboard::Clipboard;

pub fn set_clipboard(text: &str) -> Result<()> {
    Clipboard::new().unwrap().set_text(text)?;
    Ok(())
}
