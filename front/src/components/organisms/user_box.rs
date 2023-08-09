use crate::{router::Route, SessionStore, async_mouse_event};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

/// A component to display a user's information.
/// Or login/register if not logged in.
#[function_component(UserBox)]
pub fn user_box() -> Html {
    let (session_store, _) = use_store::<SessionStore>();
    html! {
        <div class={"flex flex-col text-lg md:text-2xl my-1 gap-1"}>
        { if let Some(user) = &session_store.as_ref().user { html!{ <Logout user={user.name.clone()}/> } } else { html!{ <Login/> } } }
        </div>
    }
}

#[function_component(Login)]
fn login() -> Html {
    html! {
        <>
            <button class={"flex grow items-center px-1 -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-blue-500 hover:to-cyan-300 bg-gradient-to-t from-blue-300 to-cyan-200"}>
                <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
            </button>
            <button class={"flex grow items-center px-1 -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-blue-500 hover:to-cyan-300 bg-gradient-to-t from-blue-300 to-cyan-200"}>
                <Link<Route> to={Route::Register}>{"Register"}</Link<Route>>
            </button>
        </>
    }
}

#[derive(Debug, Clone, Properties, PartialEq)]
struct LogoutProps {
    user: String,
}

#[function_component(Logout)]
fn logout(props: &LogoutProps) -> Html {
    let (_, session_dispatch) = use_store::<SessionStore>();
    let history = use_navigator().unwrap();
    let onclick = async_mouse_event!(session_dispatch, history {
        session_dispatch.reduce(|_| SessionStore { token: None, user: None }.into());
        history.push(&Route::Home);
    });
    html! {
        <>
            <div class={"font-semibold font-mono flex grow items-center px-1 -from-20% rounded-md shadow-sm bg-gradient-to-t from-blue-500 to-cyan-300"}>
                {&props.user}
            </div>
            <button {onclick} class={"flex grow items-center px-1 -from-20% rounded-md shadow-sm hover:bg-gradient-to-t hover:from-red-500 hover:to-orange-300 bg-gradient-to-t from-red-300 to-orange-200"}>
                {"Logout"}
            </button>
        </>
    }
}