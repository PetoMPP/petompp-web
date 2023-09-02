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
            <div class={"sticky top-0 z-50 w-full bg-base-100 pb-2"}>
            <div class={"m-auto w-2/3 justify-between flex flex-row pt-1 bg-base-100"}>
            <Logo />
            <Navbar />
            <UserBox />
            </div>
            </div>
        },
        Width::Large | Width::Medium | Width::Small => html! {
            <div class={"sticky top-0 z-50 w-full bg-base-100 pb-2"}>
            <div class={"w-full justify-between flex flex-row px-2 pt-1 bg-base-100"}>
            <div class={"flex flex-row gap-4"}>
                <Menu />
                <Logo />
            </div>
            <UserBox />
            </div>
            </div>
        },
    }
}
