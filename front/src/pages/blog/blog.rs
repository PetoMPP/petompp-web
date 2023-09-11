use crate::pages::page_base::PageBase;
use yew::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <PageBase>
            <h1>{"Blog"}</h1>
            <p>{"This is the blog page."}</p>
        </PageBase>
    }
}
