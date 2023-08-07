use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

const LINK_CLASS: &str = "underline decoration-solid decoration-blue text-blue-500 hover:text-blue-700";

#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps {
    pub href: String,
    pub text: Option<String>,
}

#[styled_component(HrefLink)]
pub fn link(props: &LinkProps) -> Html {
    let text = props.text.clone().unwrap_or(props.href.clone());

    html! {
        <a href={props.href.clone()} class={LINK_CLASS}>{text}</a>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct RouteLinkProps {
    pub route: Route,
    pub text: Option<String>,
}

#[styled_component(RouteLink)]
pub fn route_link(props: &RouteLinkProps) -> Html {
    let text = props.text.clone().unwrap_or(props.route.to_string());

    html! {
        <Link<Route> to={props.route.clone()} classes={LINK_CLASS}>{text}</Link<Route>>
    }
}