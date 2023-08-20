use crate::{
    components::atoms::navbar_item::{NavbarItem, Pos},
    router::Route,
};
use yew::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
        <div class={"drawer"}>
            <MenuButton />
            <MenuDropdown />
        </div>
    }
}


#[function_component(MenuButton)]
fn menu_button() -> Html {
    html! {
        <>
        <input id={"menu-drawer"} type={"checkbox"} class={"drawer-toggle"} />
        <div class={"drawer-content"}>
            <label for={"menu-drawer"} class={"btn btn-primary btn-square p-1 drawer-button"}>
                <svg viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
                <path fill="#345ca8" d="M4,10h24c1.104,0,2-0.896,2-2s-0.896-2-2-2H4C2.896,6,2,6.896,2,8S2.896,10,4,10z M28,14H4c-1.104,0-2,0.896-2,2 s0.896,2,2,2h24c1.104,0,2-0.896,2-2S29.104,14,28,14z M28,22H4c-1.104,0-2,0.896-2,2s0.896,2,2,2h24c1.104,0,2-0.896,2-2 S29.104,22,28,22z"/>
                </svg>
            </label>
        </div>
        </>
    }
}


#[function_component(MenuDropdown)]
fn menu_dropdown() -> Html {
    html! {
        <div class={"drawer-side z-10"}>
            <label for={"menu-drawer"} class={"drawer-overlay"} />
            <div class={"-ml-1 mt-20 flex flex-col gap-2"}>
            <NavbarItem pos={Pos::Left} route={Route::Home} name={"Home"}/>
            <NavbarItem pos={Pos::Left} route={Route::Projects} name={"Projects"}/>
            <NavbarItem pos={Pos::Left} route={Route::About} name={"About"}/>
            <NavbarItem pos={Pos::Left} route={Route::Contact} name={"Contact"}/>
            </div>
        </div>
    }
}
