pub mod macros {
    #[macro_export]
    macro_rules! assign_value_event {
        ($_struct:ident.$field:ident) => {{
            use web_sys::HtmlInputElement;
            let _struct = $_struct.clone();
            Callback::from(move |e: Event| {
                let target_element = e.target_unchecked_into::<HtmlInputElement>();
                _struct.borrow_mut().$field = target_element.value();
            })
        }};
    }

    /// This macro is used to create a MouseEvent callback that will spawn an async block on the local thread.
    #[macro_export]
    macro_rules! async_event {
        ($($dep:ident),*, $block:block) => {
            {
                use yew::prelude::*;
                use yew::platform::spawn_local;
                $(
                    let $dep = $dep.clone();
                )*
                Callback::from(move |_| {
                    $(
                        let $dep = $dep.clone();
                    )*
                    spawn_local(async move {$block});
                })
            }
        };
        ([prevent $event_type:ty], $($dep:ident),*, $block:block) => {
            {
                use yew::prelude::*;
                use yew::platform::spawn_local;
                $(
                    let $dep = $dep.clone();
                )*
                Callback::from(move |e: $event_type| {
                    e.prevent_default();
                    $(
                        let $dep = $dep.clone();
                    )*
                    spawn_local(async move {$block});
                })
            }
        };
    }

    #[macro_export]
    macro_rules! handle_api_error {
        ($error:ident, $session_dispatch: ident) => {
            if let Some(error) = &*$error {
                if let crate::api::client::Error::Endpoint(401..=403, _) = error {
                    $session_dispatch.reduce(|_| {
                        SessionStore {
                            token: None,
                            user: None,
                        }
                        .into()
                    });
                    return html! { <Redirect<Route> to={Route::Login} />};
                }
                show_error(error.to_string());
            }
        };
    }
}

pub mod ext {
    use yew::Callback;

    pub trait Mergable {
        fn merge(self, other: Self) -> Self;
    }

    impl<T: Clone + 'static> Mergable for Callback<T> {
        fn merge(self, other: Self) -> Self {
            Callback::from(move |e: T| {
                self.emit(e.clone());
                other.emit(e);
            })
        }
    }
}
