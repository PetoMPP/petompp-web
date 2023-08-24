use crate::{
    api, assign_value_event, components::atoms::modal::show_error,
    models::credentials::Credentials, pages::page_base::PageBase, router::Route, SessionStore,
};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Login)]
pub fn login() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state_eq(|| Option::None);
    let history = use_navigator().unwrap();
    let (_, session_dispatch) = use_store::<SessionStore>();

    let onchange_username = assign_value_event!(form_data.name);
    let onchange_password = assign_value_event!(form_data.password);
    let onsubmit = {
        let form_data = form_data.clone();
        let error_state = error_state.clone();
        let history = history.clone();
        let session_dispatch = session_dispatch.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let form_data = form_data.clone();
            let error_state = error_state.clone();
            let history = history.clone();
            let session_dispatch = session_dispatch.clone();
            spawn_local(async move {
                {
                    match api::client::Client::login(form_data.borrow().clone()).await {
                        Ok(response) => {
                            session_dispatch.reduce(|_| {
                                SessionStore {
                                    token: Some(response.token),
                                    user: Some(response.user),
                                }
                                .into()
                            });
                            error_state.set(Option::None);
                            history.push(&Route::Home);
                        }
                        Err(error) => match error {
                            api::client::Error::Endpoint(_, message) => {
                                error_state.set(Some(message))
                            }
                            api::client::Error::Parse(message)
                            | api::client::Error::Network(message) => show_error(message),
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
                <span class={"label-text text-lg lg:text-2xl"}>{"Login"}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(error) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{"Username"}</span>
            </label>
            <input class={"input input-bordered"} placeholder={"Username.."} type={"text"} onchange={onchange_username}/>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{"Password"}</span>
            </label>
            <input class={"input input-bordered"} placeholder={"Password.."} type={"password"} onchange={onchange_password}/>
            <button class={"btn btn-primary lg:text-xl mt-4"}>{"Login"}</button>
        </form>
        </PageBase>
    }
}
