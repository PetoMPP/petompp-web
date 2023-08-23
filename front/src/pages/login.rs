use crate::{
    api, assign_value_event, async_mouse_event, models::credentials::Credentials,
    pages::page_base::PageBase, router::Route, SessionStore, components::atoms::modal::show_error,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Login)]
pub fn login() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state(|| Option::None);
    let history = use_navigator().unwrap();
    let (_, session_dispatch) = use_store::<SessionStore>();

    let onchange_username = assign_value_event!(form_data.name);
    let onchange_password = assign_value_event!(form_data.password);
    let onclick = async_mouse_event!(form_data, error_state, history, session_dispatch {
        match api::client::Client::login(form_data.borrow().clone()).await {
            Ok(response) => {
                session_dispatch.reduce(|_| SessionStore { token: Some(response.token), user: Some(response.user) }.into());
                error_state.set(Option::None);
                history.push(&Route::Home);
            }
            Err(error) => match error {
                api::client::Error::Endpoint(_, message) => error_state.set(Some(message)),
                api::client::Error::Parse(message) |
                api::client::Error::Network(message) => show_error(message)
            },
        }
    });
    html! {
        <PageBase>
        <div class={"flex flex-col gap-2 w-5/6 lg:w-3/4 lg:w-1/2 m-auto"}>
            <div class={"flex flex-row flex-wrap justify-between items-center mb-2"}>
                <p class={"flex text-xl"}>{"Login"}</p>
            </div>
            <div class={"flex flex-row flex-wrap justify-between items-center mb-2"}>
                <label class={"flex block text-gray-700 text-sm font-bold"}>{"Username"}</label>
                { if let Some(error) = &(*error_state) { html!{<p class={"flex text-red-500 text-xs ml-2 italic"}>{error.clone()}</p>} } else { html!{} }}
            </div>
            <input class={"input"} placeholder={"Username.."} type={"text"} onchange={onchange_username}/>
            <div class={"flex flex-row flex-wrap justify-between items-center"}>
                <label class={"flex block text-gray-700 text-sm font-bold mb-2"}>{"Password"}</label>
            </div>
            <input class={"input"} placeholder={"Password.."} type={"password"} onchange={onchange_password}/>
            <button class={"bg-cyan-300 hover:bg-cyan-400 text-gray-700 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"} {onclick}>{"Login"}</button>
        </div>
        </PageBase>
    }
}
