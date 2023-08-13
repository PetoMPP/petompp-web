use crate::{
    async_mouse_event,
    models::user::{Role, User},
    router::Route,
    SessionStore,
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
                <div class={"flex flex-row text-lg lg:text-2xl pt-1 gap-1"}>
                    <UserButton user={user.clone()}/>
                    <LogoutButton/>
                </div>
        };
    }
    html! {
            <div class={"flex flex-row text-lg lg:text-2xl pt-1 gap-1"}>
                <LoginButton/>
                <RegisterButton/>
            </div>
    }
}

#[function_component(LoginButton)]
fn login_button() -> Html {
    html! {
        <div class={"flex flex-row text-lg lg:text-2xl gap-1"}>
            <button class={"flex grow items-center px-1 -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-orange-500 hover:to-yellow-300 bg-gradient-to-t from-orange-300 to-yellow-200"}>
            <Link<Route> to={Route::Login}>
                <svg xmlns="http://www.w3.org/2000/svg" class={"md:w-12 w-8"} viewBox="0 0 16 16">
                    <path d="M0 8a4 4 0 0 1 7.465-2H14a.5.5 0 0 1 .354.146l1.5 1.5a.5.5 0 0 1 0 .708l-1.5 1.5a.5.5 0 0 1-.708 0L13 9.207l-.646.647a.5.5 0 0 1-.708 0L11 9.207l-.646.647a.5.5 0 0 1-.708 0L9 9.207l-.646.647A.5.5 0 0 1 8 10h-.535A4 4 0 0 1 0 8zm4-3a3 3 0 1 0 2.712 4.285A.5.5 0 0 1 7.163 9h.63l.853-.854a.5.5 0 0 1 .708 0l.646.647.646-.647a.5.5 0 0 1 .708 0l.646.647.646-.647a.5.5 0 0 1 .708 0l.646.647.793-.793-1-1h-6.63a.5.5 0 0 1-.451-.285A3 3 0 0 0 4 5z"/>
                    <path d="M4 8a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"/>
                </svg>
            </Link<Route>>
            </button>
        </div>
    }
}

#[function_component(LogoutButton)]
fn logout_button() -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let history = use_navigator().unwrap();
    let onclick = async_mouse_event!(session_dispatch, history {
        session_dispatch.reduce(|_| SessionStore { token: None, user: None }.into());
        history.push(&Route::Home);
    });
    html! {
        <div class="flex flex-row text-lg lg:text-2xl gap-1">
            <button {onclick} class={"flex grow items-center -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-red-500 hover:to-orange-300 bg-gradient-to-t from-red-300 to-orange-200"}>
                <svg class={"md:w-12 w-8"} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g>
                <path fill="none" d="M0 0h24v24H0z"/>
                    <path d="M5 22a1 1 0 0 1-1-1V3a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v3h-2V4H6v16h12v-2h2v3a1 1 0 0 1-1 1H5zm13-6v-3h-7v-2h7V8l5 4-5 4z"/>
                    </g>
                </svg>
            </button>
        </div>
    }
}

#[function_component(RegisterButton)]
fn register_button() -> Html {
    html! {
        <button class={"flex grow items-center px-1 -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-green-500 hover:to-green-300 bg-gradient-to-t from-green-300 to-green-200"}>
        <Link<Route> to={Route::Register}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class={"md:w-12 w-8"} fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                <polyline points="10 17 15 12 10 7"/>
                <line x1="15" y1="12" x2="3" y2="12"/>
            </svg>
        </Link<Route>>
        </button>
    }
}

#[derive(PartialEq, Properties, Clone)]
struct UserButtonProps {
    user: User,
}

#[function_component(UserButton)]
fn user_button(props: &UserButtonProps) -> Html {
    match props.user.role {
        Role::Admin => html! {
            <button class={"font-semibold font-mono flex grow items-center px-1 -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-blue-500 hover:to-cyan-300 bg-gradient-to-t from-blue-300 to-cyan-200"}>
                <Link<Route> to={Route::AdminPanel}>{&props.user.name}</Link<Route>>
            </button>
        },
        Role::User => html! {
            <div class={"font-semibold font-mono flex grow items-center px-1 -from-20% rounded-md shadow-sm bg-gradient-to-t from-blue-500 to-cyan-300"}>
                {&props.user.name}
            </div>
        },
    }
}
