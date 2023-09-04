use crate::components::{atoms::markdown::Markdown, editor::editor::InnerProps};
use yew::prelude::*;

#[function_component(Display)]
pub fn display(props: &InnerProps) -> Html {
    let value = match props.state.value.as_str() {
        "" => "# Hello World!".to_string(),
        val => val.to_string(),
    };

    html! {
        <div class={"p-4 rounded-b-lg"}><Markdown markdown={value} /></div>
    }
}
