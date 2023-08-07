use crate::{pages::page_base::PageBase, components::atoms::link::HrefLink};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Contact)]
pub fn contact() -> Html {
    html! {
        <PageBase>
            <p class={"text-xl"}>{"Contact"}</p>
            <p class={"text-sm"}>{"This is the contact page. You can reach me at:"}</p>
            <ul class={"text-sm list-disc list-inside"}>
                <li>
                    <HrefLink href={"https://github.com/PetoMPP"} text={"My github"}/>
                </li>
                <li>
                    <HrefLink href={"https://www.linkedin.com/in/piotr-pietrzyk-5b0b55180/"} text={"My linkedin"}/>
                </li>
                <li>
                    <HrefLink href={"mailto:piotreq22@gmail.com"} text={"My email address"}/>
                </li>
            </ul>
        </PageBase>
    }
}
