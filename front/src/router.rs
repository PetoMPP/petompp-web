use crate::pages::{about::About, contact::Contact, home::Home, projects::Projects};
use std::fmt::Display;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Projects => html! {<Projects />},
        Route::About => html! {<About />},
        Route::Contact => html! {<Contact />},
    }
}
