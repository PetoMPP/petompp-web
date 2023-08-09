use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::{use_navigator, use_route};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={"flex flex-row w-full justify-between gap-1 md:gap-4"}>
            {for get_navbar_items().iter().map(|i| i.clone()) }
        </div>
    }
}

fn get_navbar_items() -> Vec<Html> {
    vec![Route::Home, Route::Projects, Route::About, Route::Contact]
        .iter()
        .map(|r| html! {<NavbarItem name={r.to_string()} route={r.clone()}/> })
        .collect()
}

#[derive(PartialEq, Properties, Clone)]
struct NavbarItemProps {
    name: String,
    route: Route,
}

#[function_component(NavbarItem)]
fn navbar_item(props: &NavbarItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let curr_route = use_route::<Route>().unwrap();
    let route = props.route.clone();
    let bg_gradient = match route == curr_route {
        true => "bg-gradient-to-b from-cyan-300 to-blue-400",
        false => "bg-gradient-to-b from-cyan-200 to-blue-300 hover:bg-gradient-to-b hover:from-cyan-300 hover:to-blue-400",
    };
    let class = format!("flex grow my-1 p-2 min-w-60 text-lg rounded-md justify-center shadow-sm {}", bg_gradient);
    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <button {class} {onclick}>{&props.name}</button>
    }
}
