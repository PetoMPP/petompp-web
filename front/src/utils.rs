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
    macro_rules! async_mouse_event {
        ($($dep:ident),* $block:block) => {
            {
                use yew::prelude::*;
                use yew::platform::spawn_local;
                $(
                    let $dep = $dep.clone();
                )*
                Callback::from(move |_: MouseEvent| {
                    $(
                        let $dep = $dep.clone();
                    )*
                    spawn_local(async move {$block});
                })
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