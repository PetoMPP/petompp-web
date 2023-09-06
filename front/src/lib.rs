use crate::{
    components::{
        atoms::modal::{ErrorModal, Modal},
        organisms::header::Header,
    },
    data::{user_agent::UserAgentStore, window::WindowStore},
    router::{switch, Route},
};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::*;

mod api;
mod components;
mod data;
mod models;
mod pages;
mod router;
mod utils;

rust_i18n::i18n!("locales", fallback = "en");

#[function_component(App)]
pub fn app() -> Html {
    let (_, window_dispatch) = use_store::<WindowStore>();
    let (user_store, user_agent_dispatch) = use_store::<UserAgentStore>();
    WindowStore::add_width_event_listener(window_dispatch);
    UserAgentStore::add_lang_change_event_listener(user_agent_dispatch);
    rust_i18n::set_locale(user_store.country.key());

    html! {
        <BrowserRouter>
            <body class={"min-h-screen"}>
                <Header />
                <img src={"/img/coast.jpg"} class={"w-full w-max-full h-max-full absolute top-10 opacity-40 my-4 h-90"} />
                <div class={"m-auto w-5/6 xl:w-2/3 flex flex-col items-center"}>
                    <Switch<Route> render={switch}/>
                </div>
            </body>
            <Modal />
            <ErrorModal />
        </BrowserRouter>
    }
}
