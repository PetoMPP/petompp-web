use crate::pages::{
    about::About,
    admin::{admin_panel::AdminPanel, user_management::UserManagement},
    blog::blog::Blog,
    contact::Contact,
    editor::Editor,
    home::Home,
    login::Login,
    not_found::NotFound,
    projects::Projects,
    register::Register,
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
    #[at("/blog")]
    BlogRoot,
    #[at("/blog/*")]
    Blog,
    #[not_found]
    #[at("/404")]
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
        Route::BlogRoot | Route::Blog => html! { <Blog />},
        Route::NotFound => html! {  <NotFound />},
    }
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

pub fn admin_switch(route: AdminRoute) -> Html {
    match route {
        AdminRoute::AdminPanel => html! { <AdminPanel />},
        AdminRoute::UserManagement => html! { <UserManagement />},
        AdminRoute::NotFound => html! {<NotFound />},
    }
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum BlogRoute {
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[at("/blog/edit/:id/:lang")]
    BlogPostEditor { id: String, lang: String },
    #[at("/blog/new")]
    BlogPostNew,
    #[at("/blog/tags/:tag")]
    BlogByTag { tag: String },
    #[not_found]
    #[at("/blog/404")]
    NotFound,
}

pub fn blog_switch(route: BlogRoute) -> Html {
    match route {
        BlogRoute::Blog => html! { <Blog />},
        BlogRoute::BlogPost { id } => todo!() /*html! { <BlogPost {id} />}*/,
        BlogRoute::BlogPostEditor { id, lang } => todo!() /*html! { <BlogPostEditor {id} {lang} />}*/,
        BlogRoute::BlogPostNew => todo!() /*html! { <BlogPostNew />}*/,
        BlogRoute::BlogByTag { tag } => todo!() /*html! { <Blog tag={Some(tag)} />}*/,
        BlogRoute::NotFound => html! {<NotFound />},
    }
}
