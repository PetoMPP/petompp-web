use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct LinkProps {
    pub href: String,
    pub text: Option<String>,
}

#[function_component(HrefLink)]
pub fn link(props: &LinkProps) -> Html {
    let text = props.text.clone().unwrap_or(props.href.clone());

    html! {
        <a href={props.href.clone()} class={"link-accent"}>{text}</a>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct RouteLinkProps {
    pub route: Route,
    pub text: Option<String>,
}

#[function_component(RouteLink)]
pub fn route_link(props: &RouteLinkProps) -> Html {
    let text = props.text.clone().unwrap_or(props.route.to_string());

    html! {
        <Link<Route> to={props.route.clone()} classes={"link-accent"}>{text}</Link<Route>>
    }
}
