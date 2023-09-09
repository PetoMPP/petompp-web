use crate::{
    api, assign_value_event, async_event,
    components::atoms::modal::show_error,
    data::{
        locales::{LocalesStore, TK},
        session::SessionStore,
    },
    models::credentials::Credentials,
    pages::page_base::PageBase,
    router::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Login)]
pub fn login() -> Html {
    let form_data = use_mut_ref(|| Credentials::default());
    let error_state = use_state_eq(|| Option::None);
    let history = use_navigator().unwrap();
    let (locales_store, _) = use_store::<LocalesStore>();
    let (_, session_dispatch) = use_store::<SessionStore>();

    let onchange_username = assign_value_event!(form_data.name);
    let onchange_password = assign_value_event!(form_data.password);
    let onsubmit = async_event!(
        [prevent SubmitEvent] |form_data, error_state, history, session_dispatch| {
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
                    api::client::Error::Endpoint(_, message) => error_state.set(Some(message)),
                    api::client::Error::Parse(message) | api::client::Error::Network(message) => {
                        show_error(message)
                    }
                },
            }
        }
    );
    html! {
        <PageBase>
        <form class={"form-control m-auto w-5/6 lg:w-3/4 xl:w-1/2"} {onsubmit}>
            <label class={"label"}>
                <span class={"label-text text-lg lg:text-2xl"}>{locales_store.get(TK::Login)}</span>
                <span class={"label-text-alt text-warning lg:text-lg"}>{if let Some(error) = &(*error_state) {error.clone() } else { "".to_string() }}</span>
            </label>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{locales_store.get(TK::Username)}</span>
            </label>
            <input class={"input input-bordered"} autocomplete={"username"} placeholder={locales_store.get(TK::TypeUsername)} type={"text"} onchange={onchange_username}/>
            <label class={"label"}>
                <span class={"label-text lg:text-lg"}>{locales_store.get(TK::Password)}</span>
            </label>
            <input class={"input input-bordered"} autocomplete={"current-password"} placeholder={locales_store.get(TK::TypePassword)} type={"password"} onchange={onchange_password}/>
            <button class={"btn btn-primary lg:text-xl mt-4"}>{locales_store.get(TK::Login)}</button>
        </form>
        </PageBase>
    }
}
