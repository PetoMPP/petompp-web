use stylist::{style, yew::styled_component};
use yew::prelude::*;

use crate::pages::page_base::PageBase;

#[styled_component(Contact)]
pub fn contact() -> Html {
    let list_style = style!(
        r#"
            padding: 0 0 0 1.5rem;
            list-style-type: disc;
        "#
    )
    .unwrap();
    html! {
        <PageBase>
            <h1>{"Contact"}</h1>
            <p>{"This is the contact page."}</p>
            <p>{"You can reach me at:"}</p>
            <ul class={list_style}>
                <li>
                    <a href="https://github.com/PetoMPP">{"my github"}</a>
                </li>
                <li>
                    <a href="https://www.linkedin.com/in/piotr-pietrzyk-5b0b55180/">{"my linkedin"}</a>
                </li>
                <li>
                    <a href="mailto:piotreq22@gmail.com">{"my email address"}</a>
                </li>
            </ul>
        </PageBase>
    }
}
