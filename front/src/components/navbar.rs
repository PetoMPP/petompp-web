use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::{use_navigator, use_route};

use crate::router::Route;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let style = style!(
        r#"
            display: flex;
            margin: 0 auto;
            width: 100%;
        "#
    )
    .unwrap();
    html! {
        <div class={style}>
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

#[styled_component(NavbarItem)]
fn navbar_item(props: &NavbarItemProps) -> Html {
    let navigator = use_navigator().unwrap();
    let get_style = |is_hovered: bool| {
        let background = match is_hovered {
            true => format!(
                "linear-gradient({}, {} 60%, {} 140%)",
                "#fddfdf", "#fff1f1", "#c38d8d"
            ),
            false => format!(
                "linear-gradient({}, {} 60%, {} 140%)",
                "#ffc4c4", "#ffd3d3", "#795858"
            ),
        };
        format!(
            r#"
            display: flex;
            width: 100%;
            background: {};
            padding: min(1.5vw, 1em, 1rem);
            justify-content: center;
            border: none;
        "#,
            background
        )
    };
    let curr_route = use_route::<Route>().unwrap();
    let is_hover = use_state(|| false);

    let onmouseenter = Callback::from({
        let is_hover = is_hover.clone();
        move |_| is_hover.set(true)
    });
    let onmouseleave = Callback::from({
        let is_hover = is_hover.clone();
        move |_| is_hover.set(false)
    });
    let style = Style::new(get_style(curr_route == props.route || *is_hover)).unwrap();
    let route = props.route.clone();
    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <button class={style} {onclick} {onmouseenter} {onmouseleave}><h5>{&props.name}</h5></button>
    }
}
