use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "components/ui/image.stpl")]
pub struct ImageUI {
    pub src: String,
    pub alt: String,
    pub width: u32,
    pub height: u32,
    pub class: String,
}
