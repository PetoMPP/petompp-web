use crate::{models::user::Role, pages::{not_found::NotFound, page_base::PageBase}, SessionStore};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let Some(Role::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html!{<NotFound />};
    };
    html! {
        <PageBase>
            <div class="flex text-2xl font-bold font-mono lg:w-3/4 w-full m-auto">
                <p>{"Admin Panel"}</p>
                // User management
            </div>
        </PageBase>
    }
}
