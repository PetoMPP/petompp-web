use crate::components::{atoms::markdown::Markdown, editor::editor::InnerProps};
use yew::prelude::*;

#[function_component(Display)]
pub fn display(props: &InnerProps) -> Html {
    html! {
        <div class={"p-4 rounded-b-lg"}><Markdown markdown={props.state.value.clone()} /></div>
    }
}
