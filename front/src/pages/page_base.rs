use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    pub children: Children,
}

#[styled_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    let style = style!(
        r#"
            background-color: rgba(255, 255, 255, 0.5);
            width: 80%;
            margin: 1rem auto;
            padding: 1rem;
        "#
    )
    .unwrap();
    html! {
        <div class={style}>
            {props.children.clone()}
            </div>
    }
}
