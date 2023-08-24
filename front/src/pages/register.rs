use crate::{
    api, assign_value_event, components::atoms::modal::show_error,
    models::credentials::Credentials, pages::page_base::PageBase, router::Route,
};
use std::fmt::Display;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq)]
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

#[function_component(Register)]
pub fn register() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state_eq(|| Option::None);
    let history = use_navigator().unwrap();

    let onchange_username = assign_value_event!(form_data.name);
    let onchange_password = assign_value_event!(form_data.password);
    let onsubmit = {
        let form_data = form_data.clone();
        let error_state = error_state.clone();
        let history = history.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form_data = form_data.clone();
            let error_state = error_state.clone();
            let history = history.clone();
            spawn_local(async move {
                {
                    match api::client::Client::register(form_data.borrow().clone()).await {
                        Ok(()) => {
                            error_state.set(Option::None);
                            history.push(&Route::Login);
                        }
                        Err(error) => match error {
                            api::client::Error::Endpoint(_, error) if error.starts_with("Name") => error_state.set(Some(Error::Username(error))),
                            api::client::Error::Endpoint(_, error) if error.starts_with("Password") => error_state.set(Some(Error::Password(error))),
                            api::client::Error::Endpoint(_, error) => error_state.set(Some(Error::Global(error))),
                            e => {
                                show_error(e.to_string());
                            }
                        },
                    }
                }
            });
        })
    };
    html! {
        <PageBase>
        <form class={"form-control m-auto w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{"Register"}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(Error::Global(error)) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{"Username"}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(Error::Username(error)) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <input class={"input input-bordered"} placeholder={"Username.."} type={"text"} onchange={onchange_username}/>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{"Password"}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(Error::Password(error)) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <input class={"input input-bordered"} placeholder={"Password.."} type={"password"} onchange={onchange_password}/>
            <button class={"btn btn-primary lg:text-xl mt-4"}>{"Register"}</button>
        </form>
        </PageBase>
    }
}
