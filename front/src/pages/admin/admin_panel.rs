use crate::{
    models::user::Role,
    pages::{not_found::NotFound, page_base::PageBase},
    router::AdminRoute,
    SessionStore,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(AdminPanel)]
pub fn admin_panel() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let Some(Role::Admin) = session_store.as_ref().user.as_ref().map(|u| &u.role) else {
        return html!{<NotFound />};
    };
    html! {
        <PageBase>
            <div class="flex flex-col lg:w-3/4 w-full m-auto">
                <p class={"text-2xl font-bold font-mono mb-2"}>{"Admin Panel"}</p>
                <p class={"text-sm mb-2"}>{"Welcome to the admin panel. Here you can manage users and other things."}</p>
                <Link<AdminRoute> classes={"btn glass"} to={AdminRoute::UserManagement}>{"User management"}</Link<AdminRoute>>
            </div>
        </PageBase>
    }
}
