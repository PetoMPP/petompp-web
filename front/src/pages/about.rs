use crate::{
    data::{resources::Key, user_agent::UserAgentStore},
    pages::page_base::EditablePage,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    const RES_KEY: &str = "about-content";
    let (user_store, _) = use_store::<UserAgentStore>();
    let key = Key {
        reskey: RES_KEY.to_string(),
        lang: user_store.country.key().to_string(),
    };
    html! {
        <EditablePage reskey={key}/ >
    }
}
