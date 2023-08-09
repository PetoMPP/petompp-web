use crate::pages::page_base::PageBase;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <PageBase>
            <p class={"text-xl"}>{"Home"}</p>
            <p class={"text-sm"}>{"Welcome to the home page!"}</p>
        </PageBase>
    }
}
