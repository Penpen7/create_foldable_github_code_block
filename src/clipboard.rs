use anyhow::Result;
use arboard::Clipboard;

pub fn set_clipboard(text: &str) -> Result<()> {
    Clipboard::new().unwrap().set_text(text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_clipboard() {
        let text = "test";
        set_clipboard(text).unwrap();
        let mut clipboard = Clipboard::new().unwrap();
        assert_eq!(clipboard.get_text().unwrap(), text);
    }
}
