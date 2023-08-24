use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownDisplayProps {
    pub markdown: String,
}

#[function_component(MarkdownDisplay)]
pub fn markdown_display(props: &MarkdownDisplayProps) -> Html {
    let html = markdown::to_html_with_options(
        props.markdown.as_str(),
        &markdown::Options {
            parse: markdown::ParseOptions::gfm(),
            ..markdown::Options::default()
        },
    )
    .unwrap();
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html);
    div.set_attribute("class", "prose w-full max-w-full").unwrap();

    Html::VRef(div.into())
}
