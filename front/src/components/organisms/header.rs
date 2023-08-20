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
    match window.width {
        Width::ExtraExtraLarge | Width::ExtraLarge => html! {
            <div class={"m-auto w-2/3 justify-between flex flex-row pt-1"}>
            <Logo />
            <Navbar />
            <UserBox />
            </div>
        },
        Width::Large | Width::Medium | Width::Small => html! {
            <div class={"w-full justify-between flex flex-row px-2 pt-1"}>
            <div class={"flex flex-row gap-4"}>
                <Menu />
                <Logo />
            </div>
            <UserBox />
            </div>
        },
    }
}
