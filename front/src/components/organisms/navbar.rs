use crate::{
    components::atoms::navbar_item::{NavbarItem, Pos},
    data::locales::{LocalesStore, TK},
    router::Route,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <div class={classes!("flex", "flex-row", "gap-2", "px-0.5", "-mt-2", "rounded-b-md")}>
            <NavbarItem pos={Pos::Top} route={Route::Home} name={locales_store.get(TK::Home)}/>
            <NavbarItem pos={Pos::Top} route={Route::Projects} name={locales_store.get(TK::Projects)}/>
            <NavbarItem pos={Pos::Top} route={Route::About} name={locales_store.get(TK::About)}/>
            <NavbarItem pos={Pos::Top} route={Route::Contact} name={locales_store.get(TK::Contact)}/>
        </div>
    }
}
