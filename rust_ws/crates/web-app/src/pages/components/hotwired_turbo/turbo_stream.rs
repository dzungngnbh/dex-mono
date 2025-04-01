use sailfish::TemplateOnce;

pub const ACTION_UPDATE: &str = "update";
pub const ACTION_REPLACE: &str = "replace";

#[derive(TemplateOnce)]
#[template(path = "components/hotwired_turbo/turbo_stream.stpl")]
pub struct TurboStream<'a> {
    pub action: &'a str,
    pub target: &'a str,
    pub template_ui: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_render() -> Result<()> {
        let turbo_stream = TurboStream {
            action: "replace",
            target: "message_1",
            template_ui: "this div will replace the existing element with the dom id \"message_1\".",
        };
        let res = turbo_stream.render_once()?;
        assert_ne!(res, "");
        Ok(())
    }
}
