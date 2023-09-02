use crate::{components::editor::data::Key, pages::page_base::EditablePage};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    const RES_KEY: &str = "home-content";
    let key = Key {
        reskey: RES_KEY.to_string(),
        lang: "en".to_string(),
    };
    html! {
        <EditablePage reskey={key}/ >
    }
}
