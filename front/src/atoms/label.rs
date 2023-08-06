use stylist::yew::styled_component;
use yew::prelude::*;

const LABEL_CLASS: &str = "block text-gray-700 text-sm font-bold mb-2";

#[derive(PartialEq, Properties, Clone)]
pub struct LabelProps {
    pub text: String,
}

#[styled_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    html! {
        <label class={LABEL_CLASS}>{props.text.clone()}</label>
    }
}
