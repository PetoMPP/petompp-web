use crate::{
    async_event,
    components::atoms::modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
    data::window::Width,
    data::{
        locales::{LocalesStore, TK},
        session::SessionStore,
    },
    models::user::{Role, User},
    router::Route,
    WindowStore,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

/// A component to display a user's information.
/// Or login/register if not logged in.
#[function_component(UserBox)]
pub fn user_box() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    if let Some(user) = &session_store.as_ref().user {
        return html! {
            <div class={"flex flex-row gap-1"}>
                <UserButton user={user.clone()}/>
                <LogoutButton/>
            </div>
        };
    }
    html! {
        <div class={"flex flex-row gap-1"}>
            <LoginButton/>
            <RegisterButton/>
        </div>
    }
}

#[function_component(LoginButton)]
fn login_button() -> Html {
    html! {
        <Link<Route> to={Route::Login} classes={"btn btn-accent btn-square p-1"}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
                <path d="M0 8a4 4 0 0 1 7.465-2H14a.5.5 0 0 1 .354.146l1.5 1.5a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0L13 9.207l-.646.647a.5.5 0 0 1-.708 0L11 9.207l-.646.647a.5.5 0 0 1-.708 0L9 9.207l-.646.647A.5.5 0 0 1 8 10h-.535A4 4 0 0 1 0 8zm4-3a3 3 0 1 0 2.712 4.285A.5.5 0 0 1 7.163 9h.63l.853-.854a.5.5 0 0 1 .708 0l.646.647.646-.647a.5.5 0 0 1 .708 0l.646.647.646-.647a.5.5 0 0 1 .708 0l.646.647.793-.793-1-1h-6.63a.5.5 0 0 1-.451-.285A3 3 0 0 0 4 5z"/>
                <path d="M4 8a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"/>
            </svg>
        </Link<Route>>
    }
}

#[function_component(LogoutButton)]
fn logout_button() -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let history = use_navigator().unwrap();
    let onclick = async_event!(|session_dispatch, history| {
        session_dispatch.reduce(|_| SessionStore::default().into());
        history.push(&Route::Login);
    });
    let onclick = show_modal_callback(
        ModalData {
            title: locales_store.get(TK::Logout),
            message: locales_store.get(TK::LogoutQuestion),
            buttons: Buttons::RiskyCancel(
                ModalButton::new(locales_store.get(TK::Logout), Some(onclick)),
                ModalButton::new(locales_store.get(TK::Cancel), None),
            ),
        },
        dispatch,
    );
    html! {
        <button {onclick} class={"btn btn-warning btn-square p-1"}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g>
                <path fill="none" d="M0 0h24v24H0z"/>
                <path d="M5 22a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v3h-2V4H6v16h12v-2h2v3a1 1 0 0 1-1 1H5zm13-6v-3h-7v-2h7V8l5 4-5 4z"/>
                </g>
            </svg>
        </button>
    }
}

#[function_component(RegisterButton)]
fn register_button() -> Html {
    html! {
        <Link<Route> to={Route::Register} classes={"btn btn-success btn-square p-1"}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                <polyline points="10 17 15 12 10 7"/>
                <line x1="15" y1="12" x2="3" y2="12"/>
            </svg>
        </Link<Route>>
    }
}

#[derive(PartialEq, Properties, Clone)]
struct UserButtonProps {
    user: User,
}

#[function_component(UserButton)]
fn user_button(props: &UserButtonProps) -> Html {
    let (window, _) = use_store::<WindowStore>();
    let name = if window.width > Width::Small {
        props.user.name.clone()
    } else {
        props
            .user
            .name
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .to_string()
    };
    let to = match props.user.role {
        Role::Admin => Route::AdminPanelRoot,
        _ => Route::Home,
    };
    html! {
        <Link<Route> {to} classes={"btn btn-primary font-mono text-xl normal-case"}>{name}</Link<Route>>
    }
}
