use crate::{
    components::{
        atoms::navbar_item::{NavbarItem, Pos},
        organisms::user_box::UserBox,
    },
    data::locales::{LocalesStore, TK},
    router::Route,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
        <div class={"drawer w-auto"}>
            <MenuButton />
            <MenuDropdown />
        </div>
    }
}

#[function_component(MenuButton)]
fn menu_button() -> Html {
    let style = "-webkit-mask: url(/img/ui/menu.svg) no-repeat center;mask: url(/img/ui/menu.svg) no-repeat center;";
    html! {
        <>
        <input id={"menu-drawer"} type={"checkbox"} class={"drawer-toggle"} />
        <div class={"drawer-content"}>
            <label for={"menu-drawer"} class={"btn btn-outline btn-primary btn-square hover:bg-primary p-1 drawer-button"}>
                <div class={"w-10 h-10 bg-primary hover:bg-primary-content"} {style}/>
            </label>
        </div>
        </>
    }
}

#[function_component(MenuDropdown)]
fn menu_dropdown() -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <div class={"drawer-side z-10"}>
            <label for={"menu-drawer"} class={"drawer-overlay"} />
            <div class={"-ml-1 mt-20 flex flex-col gap-2 min-w-[6rem]"}>
            <NavbarItem hidden_if_large={Some(())} pos={Pos::Left} route={Route::Home} name={locales_store.get(TK::Home)}/>
            <NavbarItem hidden_if_large={Some(())} pos={Pos::Left} route={Route::Projects} name={locales_store.get(TK::Projects)}/>
            <NavbarItem hidden_if_large={Some(())} pos={Pos::Left} route={Route::About} name={locales_store.get(TK::About)}/>
            <NavbarItem hidden_if_large={Some(())} pos={Pos::Left} route={Route::Contact} name={locales_store.get(TK::Contact)}/>
            <UserBox />
            </div>
        </div>
    }
}
