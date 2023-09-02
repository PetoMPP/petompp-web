use crate::pages::{
    about::About,
    admin::{admin_panel::AdminPanel, user_management::UserManagement},
    contact::Contact,
    home::Home,
    login::Login,
    not_found::NotFound,
    projects::Projects,
    register::Register, editor::Editor,
};
use std::fmt::Display;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/admin")]
    AdminPanelRoot,
    #[at("/admin/*")]
    AdminPanel,
    #[at("/editor/:key/:lang")]
    Editor { key: String, lang: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum AdminRoute {
    #[at("/admin")]
    AdminPanel,
    #[at("/admin/user_management")]
    UserManagement,
    #[not_found]
    #[at("/admin/404")]
    NotFound,
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Projects => html! {<Projects />},
        Route::About => html! {<About />},
        Route::Contact => html! {<Contact />},
        Route::Login => html! {<Login />},
        Route::Register => html! {<Register />},
        Route::AdminPanelRoot | Route::AdminPanel => {
            html! { <Switch<AdminRoute> render={admin_switch} />}
        }
        Route::Editor { key, lang } => html! { <Editor reskey={key} lang={lang} />},
        Route::NotFound => html! {  <NotFound />},
    }
}

pub fn admin_switch(route: AdminRoute) -> Html {
    match route {
        AdminRoute::AdminPanel => html! { <AdminPanel />},
        AdminRoute::UserManagement => html! { <UserManagement />},
        AdminRoute::NotFound => html! {<NotFound />},
    }
}
