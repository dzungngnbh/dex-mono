use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default, Clone)]
#[template(path = "components/link.stpl")]
pub struct LinkUI {
    pub href: String,
    pub hx_target: String,
    pub class: Option<String>,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_render() -> Result<()> {
        let res = LinkUI {
            href: "https://www.google.com".to_string(),
            hx_target: "#mainContent".to_string(),
            class: None,
            text: "Text".to_string(),
        }
        .render_once()?;
        Ok(())
    }
}
