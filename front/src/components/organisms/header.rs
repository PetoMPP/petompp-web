use crate::{
    components::{
        atoms::logo::Logo,
        organisms::{menu::Menu, navbar::Navbar, user_box::UserBox},
    },
    Width, WindowStore,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Header)]
pub fn header() -> Html {
    let (window, _) = use_store::<WindowStore>();
    if window.width >= Width::Large {
        return html! {
            <div class={"m-auto w-2/3 justify-between flex flex-row"}>
            <Logo />
            <Navbar />
            <UserBox />
            </div>
        };
    }
    html! {
    <div class={"w-full justify-between flex flex-row px-2"}>
    <div class={"flex flex-row gap-4"}>
        <Menu />
        <Logo />
    </div>
    <UserBox />
    </div>
    }
}
