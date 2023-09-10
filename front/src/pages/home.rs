use crate::pages::page_base::EditablePage;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    const RES_KEY: &str = "home-content";
    html! {
        <EditablePage reskey={RES_KEY.to_string()}/ >
    }
}
