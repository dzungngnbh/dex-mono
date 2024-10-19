use anyhow::Result;
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "components/ui/combobox/index.stpl")]
pub struct ComboboxButton<'a> {
    pub title: &'a str,
    pub data_comboboxurl: &'a str, // this is used to get the results
    pub data_attributes_ui: String,
}

#[derive(TemplateOnce)]
#[template(path = "components/ui/combobox/results_inner.stpl")]
pub struct ResultsInner<'a> {
    pub search_text: &'a str,

    // for remote combobox
    pub data_combobox_remote_url: &'a str, // optional: "url" or empty

    pub result_items_wrapper_ui: String, // result
}

#[derive(TemplateOnce, Clone, Default)]
#[template(path = "components/ui/combobox/result_items_wrapper.stpl")]
pub struct ResultItemsWrapper {
    pub result_items_ui: String, // compile list of ResultItem
}

// normally we just need to have struct, the reason we have result_item
// because we will have a customized render for it later.
#[derive(TemplateOnce, Clone, Default)]
#[template(path = "components/ui/combobox/result_item.stpl")]
pub struct ResultItem {
    pub id: String, // use for searching as well
    pub value: String,
    pub inner_ui: String,
    pub data_attributes_ui: String,
}

pub fn render_result_items_wrapper(result_items: &Vec<ResultItem>) -> Result<String> {
    let result_items_ui = result_items
        .iter()
        .map(|item| item.clone().render_once().unwrap())
        .collect::<Vec<String>>()
        .join("");

    Ok(ResultItemsWrapper { result_items_ui }.render_once()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::components::ui::combobox::ComboboxButton;

    #[test]
    fn test_render_combobox() -> Result<()> {
        let template = ComboboxButton {
            title: "title",
            data_comboboxurl: "/c/combobox_results_inner?combobox_type=services",
            ..Default::default()
        };
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }

    #[test]
    fn test_render_results() -> Result<()> {
        let result_items = vec![
            ResultItem {
                value: "value".to_string(),
                inner_ui: "label".to_string(),
                ..Default::default()
            },
            ResultItem {
                value: "value".to_string(),
                inner_ui: "label".to_string(),
                ..Default::default()
            },
        ];

        let result_items_wrapper_ui = render_result_items_wrapper(&result_items)?;

        let template = ResultsInner {
            search_text: "Search services",
            data_combobox_remote_url: "",
            result_items_wrapper_ui,
        };
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }

    #[test]
    fn test_render_result_item() -> Result<()> {
        let template = ResultItem {
            value: "value".to_string(),
            inner_ui: "label".to_string(),
            ..Default::default()
        };
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }
}
