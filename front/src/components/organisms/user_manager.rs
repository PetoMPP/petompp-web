use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

use crate::{api::client::Client, models::user::User, SessionStore};
#[function_component(UserManager)]
pub fn user_manager() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let token = session_store.token.clone().unwrap_or_default();
    let error_state = use_state(|| None);
    let reload = use_state(|| true);
    let user_data = use_state(|| vec![]);
    let mark_to_reload = {
        let reload = reload.clone();
        Callback::from(move |_| reload.set(true))
    };
    if *reload {
        reload.set(false);
        let error_state = error_state.clone();
        let user_data = user_data.clone();
        spawn_local(async move {
            match Client::get_users(&token).await {
                Ok(users) => user_data.set(users),
                Err(error) => error_state.set(Some(error.to_string())),
            };
        })
    }
    if let Some(error) = &*error_state {
        return html! {<p class={"flex text-error text-xs ml-2 italic"}>{error}</p>};
    }
    html! {
        <table class={"table"}>
            <thead>
                <tr>
                    <th>{"ID"}</th>
                    <th>{"Name"}</th>
                    <th>{"Actions"}</th>
                </tr>
            </thead>
            <tbody class={"items-center"}>
                {for user_data.iter().map(|user| html!{<UserRow user={user.clone()} reload={mark_to_reload.clone()} />})}
            </tbody>
        </table>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct UserRowProps {
    pub user: User,
    pub reload: Callback<()>,
}

#[function_component(UserRow)]
fn user_row(props: &UserRowProps) -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    let token = session_store.token.clone().unwrap_or_default();
    let activate = {
        let props = props.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let props = props.clone();
            let token = token.clone();
            spawn_local(async move {
                match Client::activate_user(&token, props.user.id).await {
                    Ok(()) => props.reload.emit(()),
                    Err(error) => gloo::console::error!(error.to_string()),
                }
            })
        })
    };
    let delete = {
        let props = props.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let props = props.clone();
            let token = token.clone();
            spawn_local(async move {
                match Client::delete_user(&token, props.user.id).await {
                    Ok(()) => props.reload.emit(()),
                    Err(error) => gloo::console::error!(error.to_string()),
                }
            })
        })
    };
    let (activate_class, activate_text) = match props.user.confirmed {
        true => (classes!("btn", "btn-sm", "btn-success", "px-1", "mr-1", "btn-disabled", "aria-disabled"), "Activated"),
        false => (classes!("btn", "btn-sm", "btn-success", "px-1", "mr-1"), "Activate"),
    };
    let (delete_class, delete_text) = match props.user.deleted_at.is_some() {
        true => (classes!("btn", "btn-sm", "btn-warning", "px-1", "mr-1", "btn-disabled", "aria-disabled"), "Deleted"),
        false => (classes!("btn", "btn-sm", "btn-warning", "px-1", "mr-1"), "Delete"),
    };
    html! {
        <tr>
            <td>{&props.user.id}</td>
            <td class={"break-all"}>{&props.user.name}</td>
            <td>
                <div class="flex flex-row">
                    <button onclick={activate} class={activate_class}>{activate_text}</button>
                    <button onclick={delete} class={delete_class}>{delete_text}</button>
                </div>
            </td>
        </tr>
    }
}
