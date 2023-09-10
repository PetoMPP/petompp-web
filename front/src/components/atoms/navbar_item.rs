use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Pos {
    Top,
    Left,
}

#[derive(PartialEq, Properties, Clone)]
pub struct NavbarItemProps {
    pub route: Route,
    pub name: String,
    pub pos: Pos,
    pub hidden_if_large: Option<()>,
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &NavbarItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let curr_route = use_route::<Route>().unwrap();
    let route = props.route.clone();
    let mut class = classes!(
        "btn",
        "btn-neutral",
        match props.pos {
            Pos::Top => "rounded-t-none",
            Pos::Left => "rounded-l-none",
        },
        match props.hidden_if_large {
            Some(_) => "lg:hidden",
            None => "",
        }
    );
    if route == curr_route {
        class.push("btn-active");
    }
    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <button {class} {onclick}>{&props.name}</button>
    }
}
