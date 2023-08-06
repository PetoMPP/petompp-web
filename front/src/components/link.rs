use stylist::yew::styled_component;
use yew::{Properties, Html, html};


#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps {
    pub href: String,
    pub text: Option<String>,
}

#[styled_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let text = props.text.clone().unwrap_or(props.href.clone());

    html! {
        <a href={props.href.clone()} class={"underline decoration-solid decoration-blue text-blue-500 hover:text-blue-700"}>{text}</a>
    }
}