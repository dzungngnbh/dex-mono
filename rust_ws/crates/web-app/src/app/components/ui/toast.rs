use anyhow::Result;
use ecow::EcoString;
use sailfish::TemplateOnce;

/// This will be included once at root layout
#[derive(TemplateOnce, Default)]
#[template(path = "components/ui/toast.stpl")]
pub struct ToastListUI {}

#[derive(TemplateOnce, Default)]
#[template(path = "components/ui/toast.item.stpl")]
pub struct ToastItemUI<'a> {
    title: Option<&'a str>,
    description: Option<&'a str>,
}

pub fn render() -> Result<String> {
    let template = ToastListUI::default();
    let html = template.render_once().unwrap();
    Ok(html)
}

pub fn render_toast_item(title: EcoString, description: EcoString) -> Result<String> {
    let template = ToastItemUI {
        title: Some(title.as_str()),
        description: Some(description.as_str()),
    };
    Ok(template.render_once()?)
}

// sugar
pub fn render_clipboard_toast(value: &str) -> Result<String> {
    render_toast_item(
        EcoString::from(format!("{} is copied to your clipboard", value)),
        EcoString::from("You can now paste it anywhere you want"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_toast_item() -> Result<()> {
        let html = render_toast_item(
            EcoString::from("test-title"),
            EcoString::from("test-description"),
        )?;
        dbg!(html);
        Ok(())
    }
}
