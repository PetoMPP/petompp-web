use crate::{
    components::atoms::navbar_item::{NavbarItem, Pos},
    router::Route,
};
use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <div class={classes!("flex", "flex-row", "gap-2", "px-0.5", "-mt-2", "rounded-b-md")}>
            <NavbarItem pos={Pos::Top} route={Route::Home} name={"Home"}/>
            <NavbarItem pos={Pos::Top} route={Route::Projects} name={"Projects"}/>
            <NavbarItem pos={Pos::Top} route={Route::About} name={"About"}/>
            <NavbarItem pos={Pos::Top} route={Route::Contact} name={"Contact"}/>
        </div>
    }
}
