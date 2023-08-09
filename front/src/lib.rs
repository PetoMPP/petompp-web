use crate::{
    components::organisms::{header::Header, navbar::Navbar},
    router::{switch, Route},
};
use models::user::User;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use yewdux::store::Store;

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

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Navbar />
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
