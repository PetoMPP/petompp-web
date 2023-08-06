use stylist::{style, yew::styled_component};
use yew::{prelude::*, Renderer};
use yew_router::{BrowserRouter, Switch};

use crate::{
    components::navbar::Navbar,
    router::{switch, Route},
};

mod components;
mod pages;
mod router;

fn main() {
    Renderer::<App>::new().render();
}

#[styled_component(App)]
fn app() -> Html {
    let style = style!(
        r#"
            margin: 0 auto;
            font-weight: 500;
        "#
    )
    .unwrap();
    html! {
        <>
        <BrowserRouter>
            <div class={style}>
                <h2>{"PetoMPP.NET"}</h2>
            </div>
            <Navbar />
            <Switch<Route> render={switch}/>
        </BrowserRouter>
        </>
    }
}
