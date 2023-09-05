use crate::{
    components::{
        atoms::modal::{ErrorModal, Modal},
        organisms::header::Header,
    },
    data::window::WindowStore,
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

#[function_component(App)]
pub fn app() -> Html {
    let (_, window_dispatch) = use_store::<WindowStore>();
    WindowStore::add_width_event_listener(window_dispatch);

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
