use crate::{
    api::client::Client,
    async_event,
    components::atoms::modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
    data::{
        locales::{LocalesStore, TK},
        session::SessionStore,
    },
    handle_api_error,
    models::user::User,
};
use yew::{platform::spawn_local, prelude::*};
use yewdux::prelude::*;

#[function_component(UserManager)]
pub fn user_manager() -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let token = session_store.token.clone().unwrap_or_default();
    let error_state = use_state_eq(|| None);
    let reload = use_state_eq(|| true);
    let user_data = use_state_eq(|| vec![]);
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
    handle_api_error!(error_state, session_dispatch);
    html! {
        <table class={"table"}>
            <thead>
                <tr>
                    <th>{locales_store.get(TK::Id)}</th>
                    <th>{locales_store.get(TK::Name)}</th>
                    <th>{locales_store.get(TK::Actions)}</th>
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
    html! {
        <tr>
            <td>{&props.user.id}</td>
            <td class={"break-all"}>{&props.user.name}</td>
            <td>
                <div class="flex flex-row">
                    <ActivateButton user={props.user.clone()} reload={props.reload.clone()} />
                    <DeleteButton user={props.user.clone()} reload={props.reload.clone()} />
                </div>
            </td>
        </tr>
    }
}

#[function_component(ActivateButton)]
fn activate_button(props: &UserRowProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let error_state = use_state_eq(|| None);
    let token = session_store.token.clone().unwrap_or_default();
    let onclick = async_event!(|props, token, error_state| {
        match Client::activate_user(&token, props.user.id).await {
            Ok(()) => props.reload.emit(()),
            Err(error) => error_state.set(Some(error)),
        }
    });
    handle_api_error!(error_state, session_dispatch);
    let (onclick, class) = match props.user.deleted_at.is_some() || props.user.confirmed {
        true => (
            None,
            "btn btn-sm btn-success px-1 mr-1 btn-disabled aria-disabled",
        ),
        false => (
            Some(show_modal_callback(
                ModalData {
                    title: locales_store.get(TK::Activate),
                    message: locales_store.get(TK::ActivateUserQuestion(props.user.name.clone())),
                    buttons: Buttons::ConfirmCancel(
                        ModalButton::new(locales_store.get(TK::Activate), Some(onclick)),
                        ModalButton::new(locales_store.get(TK::Cancel), None),
                    ),
                },
                dispatch.clone(),
            )),
            "btn btn-sm btn-success px-1 mr-1",
        ),
    };
    html! {
        <button {class} {onclick}>{locales_store.get(TK::Activate)}</button>
    }
}

#[function_component(DeleteButton)]
fn delete_button(props: &UserRowProps) -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    let (session_store, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let error_state = use_state_eq(|| None);
    let token = session_store.token.clone().unwrap_or_default();
    let onclick = async_event!(|props, token, error_state| {
        match Client::delete_user(&token, props.user.id).await {
            Ok(()) => props.reload.emit(()),
            Err(error) => error_state.set(Some(error)),
        }
    });
    handle_api_error!(error_state, session_dispatch);
    let (onclick, class) = match props.user.deleted_at.is_some() {
        true => (
            None,
            "btn btn-sm btn-warning px-1 mr-1 btn-disabled aria-disabled",
        ),
        false => (
            Some(show_modal_callback(
                ModalData {
                    title: locales_store.get(TK::Delete),
                    message: locales_store.get(TK::DeleteUserQuestion(props.user.name.clone())),
                    buttons: Buttons::RiskyCancel(
                        ModalButton::new(locales_store.get(TK::Delete), Some(onclick)),
                        ModalButton::new(locales_store.get(TK::Cancel), None),
                    ),
                },
                dispatch.clone(),
            )),
            "btn btn-sm btn-warning px-1 mr-1",
        ),
    };
    html! {
        <button {class} {onclick}>{locales_store.get(TK::Delete)}</button>
    }
}
