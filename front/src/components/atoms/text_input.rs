use stylist::yew::styled_component;
use yew::prelude::*;

const TEXT_INPUT_CLASS: &str = "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline";

#[derive(PartialEq, Properties, Clone)]
pub struct TextInputProps {
    pub password: bool,
    pub placeholder: Option<String>,
}

#[styled_component(TextInput)]
pub fn label(props: &TextInputProps) -> Html {
    let i_type = if props.password { "password" } else { "text" };
    html! {
        <input class={TEXT_INPUT_CLASS} type={i_type} placeholder={props.placeholder.clone()} />
    }
}
