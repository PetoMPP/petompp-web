use crate::{
    components::organisms::{navbar::Navbar, header::Header},
    router::{switch, Route},
};
use stylist::yew::styled_component;
use yew::{prelude::*, Renderer};
use yew_router::{BrowserRouter, Switch};

mod components;
mod pages;
mod router;

fn main() {
    Renderer::<App>::new().render();
}

#[styled_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Navbar />
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}
