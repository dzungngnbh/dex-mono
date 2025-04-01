use crate::pages::components::link::LinkUI;
use anyhow::Result;
use sailfish::TemplateOnce;

pub const SEPARATOR: &str = "<span>›</span>";

pub struct SpanUI {
    pub text: String,
}
impl SpanUI {
    fn render_once(&self) -> Result<String> {
        Ok(format!("<span>{}</span>", self.text))
    }
}

pub enum EitherElement {
    Span(SpanUI),
    Link(LinkUI),
}

#[derive(TemplateOnce)]
#[template(path = "components/breadcrumb.stpl")]
pub struct Breadcrumb {
    pub elements_ui: String,
}

pub fn render(els: &Vec<EitherElement>) -> Result<String> {
    let mut res = String::new();

    for (i, el) in els.iter().enumerate() {
        match el {
            EitherElement::Span(span_ui) => {
                res.push_str(span_ui.render_once()?.as_str());
            }
            EitherElement::Link(link_ui) => {
                let link_ui_str = link_ui.clone().render_once()?;
                res.push_str(link_ui_str.as_str());
            }
        }
        if i != els.len() - 1 {
            res.push_str(SEPARATOR);
        }
    }

    Ok(Breadcrumb { elements_ui: res }.render_once()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_render() -> Result<()> {
        let els: Vec<EitherElement> = vec![
            EitherElement::Link(LinkUI {
                href: "/dashboard".to_string(),
                hx_target: "#mainContent".to_string(),
                class: Some("font-build".to_string()),
                text: "Dashboard".to_string(),
            }),
            EitherElement::Span(SpanUI {
                text: "Strategy".to_string(),
            }),
            EitherElement::Span(SpanUI {
                text: "Strategy Name".to_string(),
            }),
        ];

        let res = render(&els);
        Ok(())
    }
}
