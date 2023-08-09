use crate::{components::atoms::link::RouteLink, pages::page_base::PageBase, router::Route};
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <PageBase>
            <p class={"text-xl"}>{"About"}</p>
            <p class={"text-sm"}>{"This is a page about me. I'm a software engineer interested in learning more about the web."}</p>
            <p class={"text-sm"}>{"If you want to know more about me, feel free to "}<RouteLink route={Route::Contact} text={"contact me"}/>{"."}</p>
        </PageBase>
    }
}
