use crate::{
    components::organisms::user_manager::UserManager,
    models::user::Role,
    pages::{not_found::NotFound, page_base::PageBase},
    SessionStore,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let Some(Role::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html!{<NotFound />};
    };
    html! {
        <PageBase>
        <div class="flex flex-col lg:w-3/4 w-full m-auto">
            <p class={"text-2xl font-bold font-mono mb-2"}>{"User management"}</p>
            <UserManager />
        </div>
        </PageBase>
    }
}
