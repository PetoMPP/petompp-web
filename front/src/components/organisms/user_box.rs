use crate::{
    async_event,
    components::{
        atoms::modal::{show_modal_callback, Buttons, ModalButton, ModalData, ModalStore},
        organisms::menu::close_menu,
    },
    data::{
        locales::{LocalesStore, TK},
        session::SessionStore,
    },
    models::user::{Role, User},
    router::Route,
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
            <>
                <UserButton user={user.clone()}/>
                <LogoutButton/>
            </>
        };
    }
    html! {
        <>
            <LoginButton/>
            <RegisterButton/>
        </>
    }
}

#[function_component(LoginButton)]
fn login_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        close_menu();
        navigator.push(&Route::Login);
    });
    let style = "-webkit-mask: url(/img/ui/login.svg) no-repeat center;mask: url(/img/ui/login.svg) no-repeat center;";
    html! {
        <div class={"btn btn-secondary p-1"} {onclick}>
            <a class={"aspect-square h-full bg-secondary-content"} {style}/>
        </div>
    }
}

#[function_component(LogoutButton)]
fn logout_button() -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let (_, dispatch) = use_store::<ModalStore>();
    let (locales_store, _) = use_store::<LocalesStore>();
    let navigator = use_navigator().unwrap();
    let style = "-webkit-mask: url(/img/ui/logout.svg) no-repeat center;mask: url(/img/ui/logout.svg) no-repeat center;";
    let onclick = async_event!(|session_dispatch, navigator| {
        close_menu();
        session_dispatch.reduce(|_| SessionStore::default().into());
        navigator.push(&Route::Login);
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
        <div class={"btn btn-warning p-1"} {onclick}>
            <a class={"aspect-square h-full bg-warning-content"} {style}/>
        </div>
    }
}

#[function_component(RegisterButton)]
fn register_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        close_menu();
        navigator.push(&Route::Register);
    });
    let style = "-webkit-mask: url(/img/ui/register.svg) no-repeat center;mask: url(/img/ui/register.svg) no-repeat center;";
    html! {
        <div class={"btn btn-accent p-1"} {onclick}>
            <a class={"aspect-square h-full bg-accent-content"} {style}/>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
struct UserButtonProps {
    user: User,
}

#[function_component(UserButton)]
fn user_button(props: &UserButtonProps) -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = match props.user.role {
        Role::Admin => Some(Callback::from(move |_| {
            close_menu();
            navigator.push(&Route::AdminPanelRoot);
        })),
        _ => None,
    };
    html! {
        <a {onclick} class={"btn btn-primary font-mono text-xl normal-case"}>{&props.user.name}</a>
    }
}
