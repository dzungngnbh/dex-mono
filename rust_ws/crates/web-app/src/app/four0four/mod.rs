use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "404.stpl")]
pub struct Four0FourPage {}

mod tests {
    

    #[test]
    fn test_render_four0four_template() -> Result<()> {
        let template = Four0FourPage {};
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }
}
