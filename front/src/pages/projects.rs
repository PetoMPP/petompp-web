use stylist::yew::styled_component;
use yew::prelude::*;

use crate::pages::page_base::PageBase;

#[styled_component(Projects)]
pub fn projects() -> Html {
    html! {
        <PageBase>
            <h1>{"Projects"}</h1>
            <p>{"This is where I will put my projects."}</p>
        </PageBase>
    }
}
