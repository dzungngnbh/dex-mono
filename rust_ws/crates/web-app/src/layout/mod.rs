use anyhow::Result;
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "layout.stpl")]
pub struct AppLayout {
    // metadata
    pub title: String,
    pub description: String,
    pub head_extra: String,       // extra head
    pub body_extra_attrs: String, // body extra attrs

    // main pages
    pub children: String,
    pub main_sidebar_ui: String,
}

#[derive(TemplateOnce, Default)]
#[template(path = "static_layout.stpl")]
pub struct StaticLayout {
    pub title: String,
    pub description: String,
    pub children: String,
}

impl AppLayout {
    /// Create new app layout
    /// TODO: notice: we consume children as String
    pub fn new(
        head: String,
        body_extra_attrs: &str,
        title: &str,
        description: &str,
        children: &str,
    ) -> Result<Self> {
        Ok(Self {
            title: title.to_string(),
            description: description.to_string(),
            head_extra: head,
            body_extra_attrs: body_extra_attrs.to_string(),
            children: children.to_string(),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_index_template() -> Result<()> {
        let template = AppLayout::new(
            "test".to_string(),
            "body_extra_attrs",
            "test",
            "test",
            "children",
        )?;
        let html = template.render_once()?;
        println!("{}", html);

        Ok(())
    }

    #[test]
    fn test_render_static_template() -> Result<()> {
        let template = StaticLayout {
            title: "title".to_string(),
            description: "description".to_string(),
            children: "children".to_string(),
        };
        let html = template.render_once().unwrap();

        Ok(())
    }
}
