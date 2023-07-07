use stylist::yew::styled_component;
use yew::prelude::*;

use crate::pages::page_base::PageBase;

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
        <PageBase>
            <h1>{"Home"}</h1>
            <p>{"Welcome to the home page!"}</p>
        </PageBase>
    }
}
