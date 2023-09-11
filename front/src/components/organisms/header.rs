use crate::{
    components::{
        atoms::{flag::FlagSelect, logo::Logo},
        organisms::menu::Menu,
    },
    data::locales::{LocalesStore, TK},
    router::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Header)]
pub fn header() -> Html {
    let (locales_store, locales_dispatch) = use_store::<LocalesStore>();
    let onselectedchanged = Callback::from(move |c| locales_dispatch.reduce_mut(|s| s.curr = c));
    html! {
        <div class={"sticky top-0 z-50 w-full bg-base-200 opacity-90 pb-2"}>
        <div class={"w-full flex flex-row px-2 pt-1 m-auto"}>
        <div class={"w-full flex flex-row justify-between items-center relative"}>
            <div class={"flex flex-row gap-2"}>
                <Menu />
                <Logo />
            </div>
                <div class={"hidden lg:block lg:absolute lg:top-0 lg:right-0 lg:left-0 lg:m-auto lg:w-fit"}>
                    <div class={"flex flex-row gap-2 px-0.5 -mt-2"}>
                        <Link<Route> to={Route::Home} classes={"btn btn-neutral borderen-none bordered-b-lg"}>{locales_store.get(TK::Home)}</Link<Route>>
                        <Link<Route> to={Route::Blog} classes={"btn btn-neutral borderen-none bordered-b-lg"}>{locales_store.get(TK::Blog)}</Link<Route>>
                        <Link<Route> to={Route::Projects} classes={"btn btn-neutral borderen-none bordered-b-lg"}>{locales_store.get(TK::Projects)}</Link<Route>>
                        <Link<Route> to={Route::About} classes={"btn btn-neutral borderen-none bordered-b-lg"}>{locales_store.get(TK::About)}</Link<Route>>
                        <Link<Route> to={Route::Contact} classes={"btn btn-neutral borderen-none bordered-b-lg"}>{locales_store.get(TK::Contact)}</Link<Route>>
                    </div>
                </div>
            <FlagSelect country={locales_store.curr} {onselectedchanged}/>
        </div>
        </div>
        </div>
    }
}
