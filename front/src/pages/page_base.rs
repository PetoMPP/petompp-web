use crate::{
    api::client::Client,
    components::{
        atoms::{markdown::Markdown, modal::show_error},
        editor::data::Key,
    },
    models::user::Role,
    router::Route,
    SessionStore,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageBaseProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageBase)]
pub fn page_base(props: &PageBaseProps) -> Html {
    html! {
        <div class={"animate-fade-up flex flex-col mt-10 lg:mt-20 min-h-[40rem] w-full mb-6 p-8 rounded-t-xl bg-base-100"}>
            <div class={"flex flex-col lg:w-5/6 w-full mx-auto"}>
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct EditablePageBaseProps {
    pub reskey: Key,
}

#[function_component(EditablePage)]
pub fn editable_page_base(props: &EditablePageBaseProps) -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let navigator = use_navigator().unwrap();
    let reskey = props.reskey.clone();
    let edit_onclick = Callback::from(move |_| {
        navigator.push(&Route::Editor {
            key: reskey.reskey.clone(),
            lang: reskey.lang.clone(),
        });
    });
    let edit_class = match &session_store.user {
        Some(u) if u.role == Role::Admin => {
            "btn absolute top-5 right-5 btn-accent btn-xs btn-outline"
        }
        _ => "hidden",
    };
    let markdown = use_state_eq(|| String::new());
    {
        let markdown = markdown.clone();
        let reskey = props.reskey.clone();
        spawn_local(async move {
            match Client::get_resource(reskey.reskey.as_str(), reskey.lang.as_str()).await {
                Ok(md) => {
                    markdown.set(md);
                }
                Err(e) => {
                    show_error(e.to_string());
                }
            }
        });
    }

    html! {
        <PageBase>
            <button class={edit_class} onclick={edit_onclick}>{"Edit"}</button>
            <Markdown markdown={(*markdown).clone()}/>
        </PageBase>
    }
}
