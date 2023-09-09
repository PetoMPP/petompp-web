use crate::{
    router::{AdminRoute, Route},
    use_effect_deps,
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Element;
use yew::prelude::*;
use yew_router::{prelude::*, Routable};

const ID: &str = "markdown-display";

#[derive(Properties, PartialEq)]
pub struct MarkdownDisplayProps {
    pub markdown: String,
    pub interactive: Option<()>, // Makes links clickable
}

#[function_component(Markdown)]
pub fn markdown_display(props: &MarkdownDisplayProps) -> Html {
    let navigator = use_navigator().unwrap();
    let html = markdown::to_html_with_options(
        props.markdown.as_str(),
        &markdown::Options {
            parse: markdown::ParseOptions::gfm(),
            ..markdown::Options::default()
        },
    )
    .unwrap();
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html);
    let interactive = props.interactive.clone();
    let class = match interactive {
        Some(()) => "prose w-full max-w-full",
        None => "prose w-full max-w-full pointer-events-none",
    };
    use_effect_deps!(|interactive| {
        if interactive.is_some() {
            make_links_clickable(navigator.clone());
        }
        || {}
    });

    html! {
        <div {class} id={ID}>
            {Html::VRef(div.into())}
        </div>
    }
}

fn make_links_clickable(navigator: Navigator) {
    let element = get_display_element();
    let links = element.query_selector_all("a").unwrap();
    for i in 0..links.length() {
        let link: Element = links.get(i).unwrap().unchecked_into();
        let Some(href) = link.get_attribute("href") else {
            continue;
        };
        if !href.starts_with('/') {
            continue;
        }
        let Some(route) = Route::recognize(href.as_str()) else {
            continue;
        };
        let onclick = match route {
            Route::AdminPanel => match AdminRoute::recognize(href.as_str()) {
                Some(admin_route) => get_route_onclick(admin_route, navigator.clone()),
                _ => get_route_onclick(route, navigator.clone()),
            },
            _ => get_route_onclick(route, navigator.clone()),
        };
        link.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())
            .unwrap();
        onclick.forget();
    }
}

fn get_route_onclick<T: Routable + std::fmt::Debug + 'static>(
    route: T,
    navigator: Navigator,
) -> Closure<dyn Fn(Event)> {
    let navigator = navigator.clone();
    Closure::new(Box::new(move |e: Event| {
        e.prevent_default();
        navigator.push(&route);
    }))
}

fn get_display_element() -> Element {
    let element: Element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(ID)
        .unwrap()
        .unchecked_into();
    element
}
