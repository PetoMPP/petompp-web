use crate::pages::page_base::PageBase;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Home)]
pub fn home() -> Html {
    html! {
        <PageBase>
            <p class={"text-xl"}>{"Home"}</p>
            <p class={"text-sm"}>{"Welcome to the home page!"}</p>
        </PageBase>
    }
}
