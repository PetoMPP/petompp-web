use crate::{
    api::client::Client,
    components::atoms::{markdown::Markdown, modal::show_error},
    models::user::Role,
    pages::page_base::PageBase,
    router::Route,
    SessionStore,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    const RES_KEY: &str = "home-content";
    let (session_store, _) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let markdown = use_state_eq(|| String::new());
    {
        let markdown = markdown.clone();
        spawn_local(async move {
            match Client::get_resource(RES_KEY, "en").await {
                Ok(md) => {
                    markdown.set(md);
                }
                Err(e) => {
                    show_error(e.to_string());
                }
            }
        });
    }
    let edit_onclick = Callback::from(move |_| {
        navigator.push(&Route::Editor {
            key: RES_KEY.to_string(),
            lang: "en".to_string(),
        });
    });
    let edit_class = match &session_store.user {
        Some(u) if u.role == Role::Admin => {
            "btn absolute top-5 right-5 btn-accent btn-xs btn-outline"
        }
        _ => "hidden",
    };

    html! {
        <PageBase>
            <button class={edit_class} onclick={edit_onclick}>{"Edit"}</button>
            <div class={"flex flex-col lg:w-5/6 w-full mx-auto"}>
                <Markdown markdown={(*markdown).clone()}/>
            </div>
        </PageBase>
    }
}
