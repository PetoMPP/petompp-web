use crate::{
    data::locales::{LocalesStore, TK},
    pages::page_base::PageBase,
};
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let (locales_store, _) = use_store::<LocalesStore>();
    html! {
        <PageBase>
            <p class={"text-xl"}>{locales_store.get(TK::Projects)}</p>
            <p class={"text-sm"}>{locales_store.get(TK::ProjectsDescription)}</p>
        </PageBase>
    }
}
