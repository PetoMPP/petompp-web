use crate::{
    data::{locales::LocalesStore, resources::Key, session::SessionStore},
    models::user::Role,
    pages::{not_found::NotFound, page_base::EditablePage},
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    const RES_KEY: &str = "admin-panel-content";
    let (session_store, _) = use_store::<SessionStore>();
    let Some(Role::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html!{<NotFound />};
    };
    let (locales_store, _) = use_store::<LocalesStore>();
    let key = Key {
        reskey: RES_KEY.to_string(),
        lang: locales_store.curr.key().to_string(),
    };
    html! {
        <EditablePage reskey={key}/ >
    }
}
