pub mod macros {
    /// This macro is used to create a callback that will spawn an async block on the local thread.
    #[macro_export]
    macro_rules! async_event {
        (|$($dep:ident),*| $block:block) => {
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
        ([prevent $event_type:ty] | $($dep:ident),*| $block:block) => {
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

    /// I prefer this way of handling cloning dependencies rather than the way it is done in the use_effect_with_deps method.
    #[macro_export]
    macro_rules! use_effect_deps {
        (|$($dep:ident),*| $block:block) => {
            {
                use yew::prelude::*;
                $(
                    let $dep = $dep.clone();
                )*
                use_effect(move || {
                    $(
                        let $dep = $dep.clone();
                    )*
                    $block
                }
                )
            }
        };
    }

    #[macro_export]
    macro_rules! handle_api_error {
        ($error:ident, $session_dispatch: ident, $redirect: expr) => {
            use crate::components::atoms::modal::show_error;
            use crate::router::Route;
            use yew_router::prelude::*;
            if let Some(error) = &*$error {
                if let crate::api::client::ApiError::Endpoint(401..=403, _) = error {
                    $session_dispatch.reduce(|_| {
                        SessionStore {
                            token: None,
                            user: None,
                        }
                        .into()
                    });
                    return html! { <Redirect<Route> to={Route::Login} />};
                }
                show_error(error.to_string(), $redirect);
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
