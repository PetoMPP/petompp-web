use crate::{
    api::client::Client,
    components::{
        atoms::modal::{ErrorModal, Modal},
        organisms::header::Header,
    },
    data::locales::LocalesStore,
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
    let (locale_store, locale_dispatch) = use_store::<LocalesStore>();
    if !locale_store.is_loaded(locale_store.curr) || error_state.is_some() {
        let locale_dispatch = locale_dispatch.clone();
        spawn_local(async move {
            match Client::get_locale(locale_store.curr.key()).await {
                Ok(data) => {
                    locale_dispatch.reduce_mut(|l| l.load(locale_store.curr, data));
                }
                Err(e) => {
                    error_state.set(Some(e));
                }
            };
        })
    }
    LocalesStore::add_lang_change_event_listener(locale_dispatch);

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
