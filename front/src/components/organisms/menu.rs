use crate::{
    components::atoms::navbar_item::{NavbarItem, Pos},
    router::Route,
};
use yew::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    let show = use_state(|| false);
    let setshow = {
        let show = show.clone();
        Callback::from(move |val: bool| show.set(val))
    };
    html! {
        <div>
            <MenuButton show={*show} setshow={setshow.clone()} />
            <MenuDropdown show={*show} {setshow}/>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct MenuButtonProps {
    show: bool,
    setshow: Callback<bool>,
}

#[function_component(MenuButton)]
fn menu_button(props: &MenuButtonProps) -> Html {
    let onclick = {
        let show = props.show.clone();
        let setshow = props.setshow.clone();
        Callback::from(move |_| setshow.emit(!show))
    };
    let bg_gradient = match props.show {
        true => "bg-gradient-to-b from-cyan-300 to-blue-400",
        false => "bg-gradient-to-b from-cyan-200 to-blue-300",
    };
    let class = classes!("flex", "mt-1", "p-1", "rounded-md", bg_gradient);
    html! {
        <button {onclick} {class}>
            <svg class={"flex md:w-12 md:h-12 w-8 h-8"} viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
            <path fill="#0891b2" d="M4,10h24c1.104,0,2-0.896,2-2s-0.896-2-2-2H4C2.896,6,2,6.896,2,8S2.896,10,4,10z M28,14H4c-1.104,0-2,0.896-2,2 s0.896,2,2,2h24c1.104,0,2-0.896,2-2S29.104,14,28,14z M28,22H4c-1.104,0-2,0.896-2,2s0.896,2,2,2h24c1.104,0,2-0.896,2-2 S29.104,22,28,22z"/>
            </svg>
        </button>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct MenuDropdownProps {
    show: bool,
    setshow: Callback<bool>,
}

#[function_component(MenuDropdown)]
fn menu_dropdown(props: &MenuDropdownProps) -> Html {
    let onclick = {
        let show = props.show.clone();
        let setshow = props.setshow.clone();
        Callback::from(move |_| setshow.emit(!show))
    };
    if !props.show {
        return html! {};
    }
    html! {
        <div {onclick} class={"absolute top-0 left-0 w-full h-full bg-blue-100 bg-opacity-60"}>
            <div class={"flex flex-col absolute left-0 mt-20 rounded-md gap-2"}>
            <NavbarItem pos={Pos::Left} route={Route::Home} name={"Home"}/>
            <NavbarItem pos={Pos::Left} route={Route::Projects} name={"Projects"}/>
            <NavbarItem pos={Pos::Left} route={Route::About} name={"About"}/>
            <NavbarItem pos={Pos::Left} route={Route::Contact} name={"Contact"}/>
            </div>
        </div>
    }
}
