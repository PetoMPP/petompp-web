use crate::pages::page_base::EditablePage;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    const RES_KEY: &str = "about-content";
    html! {
        <EditablePage reskey={RES_KEY.to_string()}/ >
    }
}
