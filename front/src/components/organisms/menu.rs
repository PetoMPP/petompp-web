use crate::{
    components::organisms::user_box::UserBox,
    data::locales::{LocalesStore, TK},
    router::Route,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
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

pub fn close_menu() {
    let element: HtmlInputElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("menu-drawer")
        .unwrap()
        .unchecked_into();
    element.set_checked(false);
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
    let navigator = use_navigator().unwrap();
    let get_onclick = |route: Route| {
        Callback::from(move |_| {
            close_menu();
            navigator.push(&route);
        })
    };
    html! {
        <div class={"drawer-side z-10"}>
            <label for={"menu-drawer"} class={"drawer-overlay"} />
            <div class={"-ml-1 mt-20 flex flex-col gap-2 min-w-[6rem]"}>
                <a onclick={get_onclick.clone()(Route::Home)} class={"lg:hidden btn btn-neutral borderen-none bordered-r-lg"}>{locales_store.get(TK::Home)}</a>
                <a onclick={get_onclick.clone()(Route::BlogRoot)} class={"lg:hidden btn btn-neutral borderen-none bordered-r-lg"}>{locales_store.get(TK::Blog)}</a>
                <a onclick={get_onclick.clone()(Route::Projects)} class={"lg:hidden btn btn-neutral borderen-none bordered-r-lg"}>{locales_store.get(TK::Projects)}</a>
                <a onclick={get_onclick.clone()(Route::About)} class={"lg:hidden btn btn-neutral borderen-none bordered-r-lg"}>{locales_store.get(TK::About)}</a>
                <a onclick={get_onclick.clone()(Route::Contact)} class={"lg:hidden btn btn-neutral borderen-none bordered-r-lg"}>{locales_store.get(TK::Contact)}</a>
                <UserBox />
            </div>
        </div>
    }
}
