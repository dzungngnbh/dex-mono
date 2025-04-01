use crate::pages::components::hotwired_turbo::{TurboStream, ACTION_UPDATE};
use anyhow::Result;
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone)]
#[template(path = "components/title.stpl")]
pub struct TitleUi {
    pub title: String,
}

/// Updated title in real time
impl TitleUi {
    pub fn render_turbo(&self) -> Result<String> {
        let ui_str = self.clone().render_once()?;
        let turbo_stream = TurboStream {
            action: ACTION_UPDATE,
            target: "page-title",
            template_ui: &ui_str,
        };
        Ok(turbo_stream.render_once()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render() -> Result<()> {
        let title_ui = TitleUi {
            title: "test".to_string(),
        };
        let res = title_ui.render_turbo()?;
        assert_ne!(res, "");
        Ok(())
    }
}
