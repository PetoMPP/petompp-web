use crate::{
    components::editor::data::Key, data::user_agent::UserAgentStore, pages::page_base::EditablePage,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    const RES_KEY: &str = "home-content";
    let (user_store, _) = use_store::<UserAgentStore>();
    let key = Key {
        reskey: RES_KEY.to_string(),
        lang: user_store.country.key().to_string(),
    };
    html! {
        <EditablePage reskey={key}/ >
    }
}
