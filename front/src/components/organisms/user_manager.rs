use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    api::client::Client,
    components::atoms::modal::{get_modal_open_callback, ButtonMode, Modal, ModalButton},
    models::user::User,
    router::Route,
    SessionStore,
};
#[function_component(UserManager)]
pub fn user_manager() -> Html {
    let (session_store, session_dispatch) = use_store::<SessionStore>();
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
                Err(error) => error_state.set(Some(error)),
            };
        })
    }
    if let Some(error) = &*error_state {
        if let crate::api::client::Error::Endpoint(401..=403, _) = error {
            session_dispatch.reduce(|_| {
                SessionStore {
                    token: None,
                    user: None,
                }
                .into()
            });
            return html! { <Redirect<Route> to={Route::Login} />};
        }
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
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let error_state = use_state(|| None);
    let token = session_store.token.clone().unwrap_or_default();
    let activate = {
        let error_state = error_state.clone();
        let props = props.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let error_state = error_state.clone();
            let props = props.clone();
            let token = token.clone();
            spawn_local(async move {
                match Client::activate_user(&token, props.user.id).await {
                    Ok(()) => props.reload.emit(()),
                    Err(error) => error_state.set(Some(error)),
                }
            })
        })
    };
    let delete = {
        let error_state = error_state.clone();
        let props = props.clone();
        let token = token.clone();
        Callback::from(move |_| {
            let error_state = error_state.clone();
            let props = props.clone();
            let token = token.clone();
            spawn_local(async move {
                match Client::delete_user(&token, props.user.id).await {
                    Ok(()) => props.reload.emit(()),
                    Err(error) => error_state.set(Some(error)),
                }
            })
        })
    };
    if let Some(crate::api::client::Error::Endpoint(401..=403, _)) = &*error_state {
        session_dispatch.reduce(|_| {
            SessionStore {
                token: None,
                user: None,
            }
            .into()
        });
        return html! { <Redirect<Route> to={Route::Login} />};
    }
    let activate_button = match props.user.confirmed {
        true => {
            html! {<button onclick={activate} class={"btn btn-sm btn-success px-1 mr-1 btn-disabled aria-disabled"}>{"Activated"}</button>}
        }
        false => {
            let id = format!("activate_modal_{}", props.user.id);
            html! {
                <Modal id={id.clone()} title={"Activate"} message={"Do you want to activate this user?"} mode={ButtonMode::ConfirmCancel(ModalButton::new("activate", Some(activate)), ModalButton::new("cancel", None))}>
                    <button onclick={get_modal_open_callback(id)} class={"btn btn-sm btn-success px-1 mr-1"}>{"Activate"}</button>
                </Modal>
            }
        }
    };
    let delete_button = match props.user.deleted_at.is_some() {
        true => {
            html! {<button onclick={delete} class={"btn btn-sm btn-warning px-1 mr-1 btn-disabled aria-disabled"}>{"Deleted"}</button>}
        }
        false => {
            let id = format!("delete_modal_{}", props.user.id);
            html! {
                <Modal id={id.clone()} title={"Delete"} message={"Do you want to delete this user?"} mode={ButtonMode::RiskyCancel(ModalButton::new("delete", Some(delete)), ModalButton::new("cancel", None))}>
                    <button onclick={get_modal_open_callback(id)} class={"btn btn-sm btn-warning px-1 mr-1"}>{"Delete"}</button>
                </Modal>
            }
        }
    };
    html! {
        <tr>
            <td>{&props.user.id}</td>
            <td class={"break-all"}>{&props.user.name}</td>
            <td>
                <div class="flex flex-row">
                    {activate_button}
                    {delete_button}
                </div>
            </td>
        </tr>
    }
}
