use crate::{
    components::{atoms::modal::ErrorModal, organisms::header::Header},
    router::{switch, Route},
};
use models::user::User;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::*;

mod api;
mod components;
mod models;
mod pages;
mod router;
mod utils;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "session", storage_tab_sync)]
pub struct SessionStore {
    user: Option<User>,
    token: Option<String>,
}

#[derive(Default, PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Width {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
    ExtraExtraLarge,
}

impl From<f64> for Width {
    fn from(value: f64) -> Self {
        match value as u32 {
            0..=767 => Width::Small,
            768..=1023 => Width::Medium,
            1024..=1279 => Width::Large,
            1280..=1535 => Width::ExtraLarge,
            1536.. => Width::ExtraExtraLarge,
        }
    }
}

#[derive(Default, PartialEq, Clone, Debug, Store)]
pub struct WindowStore {
    pub width: Width,
}

#[function_component(App)]
pub fn app() -> Html {
    let (_, window_dispatch) = use_store::<WindowStore>();
    let window = web_sys::window().unwrap();
    window_dispatch.reduce_mut(|w| {
        let width = window.inner_width().unwrap().as_f64().unwrap().into();
        if w.width != width {
            w.width = width;
        }
    });
    let onwindowresize = {
        let window_dispatch = window_dispatch.clone();
        Closure::<dyn Fn(Event)>::new(Box::new(move |e: Event| {
            let window: web_sys::Window = e.target().unwrap().dyn_into().unwrap();
            window_dispatch.reduce_mut(|w| {
                let width = window.inner_width().unwrap().as_f64().unwrap().into();
                if w.width != width {
                    w.width = width;
                }
            });
        }))
    };
    window
        .add_event_listener_with_callback("resize", onwindowresize.as_ref().unchecked_ref())
        .unwrap();
    onwindowresize.forget();
    html! {
        <BrowserRouter>
            <body class={"min-h-screen"}>
                <Header />
                <img src={"/img/coast.jpg"} class={"w-full w-max-full h-max-full absolute top-10 opacity-40 my-4 h-90"} />
                <div class={"m-auto w-5/6 xl:w-2/3 flex flex-col items-center"}>
                    <Switch<Route> render={switch}/>
                </div>
            </body>
            <ErrorModal />
        </BrowserRouter>
    }
}
