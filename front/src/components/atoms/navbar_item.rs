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
    let bg_gradient = match route == curr_route {
        true => "from-cyan-300 to-blue-400",
        false => "from-cyan-200 to-blue-300 hover:from-cyan-300 hover:to-blue-400",
    };
    let class = match props.pos {
        Pos::Top => classes!("rounded-b-md", "bg-gradient-to-b", bg_gradient),
        Pos::Left => classes!("rounded-r-md", "bg-gradient-to-r", bg_gradient),
    };
    let class = classes!(
        "flex",
        "p-2",
        "text-lg",
        "justify-center",
        "items-center",
        "shadow-sm",
        class
    );
    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <button {class} {onclick}>{&props.name}</button>
    }
}
