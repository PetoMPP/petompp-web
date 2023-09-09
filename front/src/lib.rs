use crate::{
    api::client::Client,
    components::{
        atoms::modal::{ErrorModal, Modal},
        organisms::header::Header,
    },
    data::{locales::LocalesStore, user_agent::UserAgentStore, window::WindowStore},
    router::{switch, Route},
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::*;

mod api;
mod components;
mod data;
mod models;
mod pages;
mod router;
mod utils;

#[function_component(App)]
pub fn app() -> Html {
    let error_state = use_state(|| None);
    let (_, window_dispatch) = use_store::<WindowStore>();
    let (user_store, user_agent_dispatch) = use_store::<UserAgentStore>();
    let (locale_store, locale_dispatch) = use_store::<LocalesStore>();
    if user_store.country != locale_store.curr || !locale_store.is_loaded(user_store.country) || error_state.is_some() {
        spawn_local(async move {
            match Client::get_locale(user_store.country.key()).await {
                Ok(data) => {
                    locale_dispatch.reduce_mut(|l| {
                        l.curr = user_store.country.clone();
                        l.load(user_store.country.clone(), data)
                    });
                }
                Err(e) => {
                    error_state.set(Some(e));
                }
            };
        })
    }
    WindowStore::add_width_event_listener(window_dispatch);
    UserAgentStore::add_lang_change_event_listener(user_agent_dispatch);

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
