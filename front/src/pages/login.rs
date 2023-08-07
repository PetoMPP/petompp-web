use crate::{
    components::atoms::{button::Button, label::Label, text_input::TextInput},
    pages::page_base::PageBase,
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Login)]
pub fn login() -> Html {
    html! {
        <PageBase>
            <div class={"flex flex-col gap-2 w-5/6 md:w-1/2 m-auto"}>
                <p class={"text-xl"}>{"Login"}</p>
                <Label text={"Username"}/>
                <TextInput placeholder={"Username.."} password={false}/>
                <Label text={"Password"}/>
                <TextInput placeholder={"Password.."} password={true}/>
                <Button text={"Login"}/>
            </div>
        </PageBase>
    }
}
