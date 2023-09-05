use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum Width {
    #[default]
    Small,
    Medium,
    Large,
    ExtraLarge,
    ExtraExtraLarge,
}

impl From<f64> for Width {
    fn from(value: f64) -> Self {
        match value as u32 {
            0..=767 => Width::Small,
            768..=1023 => Width::Medium,
            1024..=1279 => Width::Large,
            1280..=1535 => Width::ExtraLarge,
            1536.. => Width::ExtraExtraLarge,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Store)]
pub struct WindowStore {
    pub width: Width,
}

impl WindowStore {
    pub fn add_width_event_listener(dispatch: Dispatch<Self>) {
        let window = web_sys::window().unwrap();
        let onwindowresize = {
            let window_dispatch = dispatch.clone();
            Closure::<dyn Fn(Event)>::new(Box::new(move |e: Event| {
                window_dispatch.reduce_mut(|w| {
                    w.width = e
                        .target_unchecked_into::<web_sys::Window>()
                        .inner_width()
                        .unwrap()
                        .as_f64()
                        .unwrap()
                        .into()
                });
            }))
        };
        window
            .add_event_listener_with_callback("resize", onwindowresize.as_ref().unchecked_ref())
            .unwrap();
        onwindowresize.forget();
    }
}

impl Default for WindowStore {
    fn default() -> Self {
        Self {
            width: web_sys::window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap()
                .into(),
        }
    }
}
