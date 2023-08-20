use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

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
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &NavbarItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let curr_route = use_route::<Route>().unwrap();
    let route = props.route.clone();
    let mut class = classes!(
        "btn",
        "btn-neutral",
        "rounded-none",
        match props.pos {
        Pos::Top => "rounded-b-lg",
        Pos::Left => "rounded-r-lg"
    });
    if route == curr_route {
        class.push("btn-active");
    }
    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <button {class} {onclick}>{&props.name}</button>
    }
}
