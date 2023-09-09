use crate::{
    data::{resources::Key, user_agent::UserAgentStore},
    pages::page_base::EditablePage,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    const RES_KEY: &str = "contact-content";
    let (user_store, _) = use_store::<UserAgentStore>();
    let key = Key {
        reskey: RES_KEY.to_string(),
        lang: user_store.country.key().to_string(),
    };
    html! {
        <EditablePage reskey={key}/ >
    }
}
