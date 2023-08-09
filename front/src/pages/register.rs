use crate::{
    api, assign_value_event, async_mouse_event, components::atoms::text_input::TextInput,
    models::credentials::Credentials, pages::page_base::PageBase, router::Route,
};
use std::fmt::Display;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug)]
enum Error {
    Global(String),
    Username(String),
    Password(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Global(e) => write!(f, "{}", e),
            Error::Username(e) => write!(f, "{}", e),
            Error::Password(e) => write!(f, "{}", e),
        }
    }
}

#[styled_component(Register)]
pub fn register() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state(|| Option::None);
    let history = use_navigator().unwrap();

    let onchange_username = assign_value_event!(form_data.name);
    let onchange_password = assign_value_event!(form_data.password);
    let onclick = async_mouse_event!(form_data, error_state, history {
        match api::client::Client::register(form_data.borrow().clone()).await {
            Ok(()) => {
                error_state.set(Option::None);
                history.push(&Route::Login);
            }
            Err(error) => {
                match error {
                    api::client::Error::Endpoint(_, error) => match error {
                        ref e if e.starts_with("Name") => error_state.set(Some(Error::Username(error))),
                        ref e if e.starts_with("Password") => error_state.set(Some(Error::Password(error))),
                        _ => error_state.set(Some(Error::Global(error))),
                    }
                    _ => {
                        error_state.set(Some(Error::Global("Unknown error".to_string())));
                        gloo::console::error!(format!("Error: {}", error));
                    }
                }
            }
        }
    });
    html! {
        <PageBase>
        <div class={"flex flex-col gap-2 w-5/6 md:w-3/4 lg:w-1/2 m-auto"}>
            <div class={"flex flex-row flex-wrap justify-between items-center mb-2"}>
                <p class={"flex text-xl"}>{"Register"}</p>
                { if let Some(Error::Global(error)) = &(*error_state) { html!{<p class={"flex text-red-500 text-xs ml-2 italic"}>{error.clone()}</p>} } else { html!{} }}
            </div>
            <div class={"flex flex-row flex-wrap justify-between items-center mb-2"}>
                <label class={"flex block text-gray-700 text-sm font-bold"}>{"Username"}</label>
                { if let Some(Error::Username(error)) = &(*error_state) { html!{<p class={"flex text-red-500 text-xs ml-2 italic"}>{error.clone()}</p>} } else { html!{} }}
            </div>
            <TextInput placeholder={"Username.."} password={false} onchange={onchange_username}/>
            <div class={"flex flex-row flex-wrap justify-between items-center"}>
                <label class={"flex block text-gray-700 text-sm font-bold mb-2"}>{"Password"}</label>
                { if let Some(Error::Password(error)) = &(*error_state) { html!{<p class={"flex text-red-500 text-xs ml-2 italic"}>{error.clone()}</p>} } else { html!{} }}
            </div>
            <TextInput placeholder={"Password.."} password={true} onchange={onchange_password}/>
            <button class={"bg-cyan-300 hover:bg-cyan-400 text-gray-700 font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"} {onclick}>{"Register"}</button>
        </div>
        </PageBase>
    }
}
