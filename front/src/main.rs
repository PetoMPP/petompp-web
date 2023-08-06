use crate::{
    components::navbar::Navbar,
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
        <>
        <BrowserRouter>
            <div class={"w-full flex justify-center bg-gradient-to-r from-cyan-300 via-blue-500 to-cyan-300 py-4 my-1 text-5xl rounded-lg shadow-md"}>{"PetoMPP.NET"}</div>
            <Navbar />
            <Switch<Route> render={switch}/>
        </BrowserRouter>
        </>
    }
}
