use stylist::yew::styled_component;
use yew::prelude::*;

const BUTTON_CLASS: &str = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline";

#[derive(PartialEq, Properties, Clone)]
pub struct ButtonProps {
    pub text: String,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={BUTTON_CLASS}>{props.text.clone()}</button>
    }
}
