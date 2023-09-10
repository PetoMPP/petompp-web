use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputProps {
    pub label: String,
    pub itype: String,
    pub placeholder: Option<String>,
    pub autocomplete: Option<String>,
    pub onchange: Option<Callback<InputEvent>>,
    pub error: Option<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let input_class = match &props.error {
        Some(_) => "input input-bordered shadow-md input-error text-error",
        None => "input input-bordered shadow-md",
    };
    let span_class = match &props.error {
        Some(_) => "label-text lg:text-lg text-error",
        None => "label-text lg:text-lg",
    };
    html! {
        <>
        <label class={"label"}>
            <span class={span_class}>{&props.label}</span>
        </label>
        <input class={input_class} type={props.itype.clone()} placeholder={props.placeholder.clone()} autocomplete={props.autocomplete.clone()} oninput={props.onchange.clone()} />
        <span class={"text-error mt-1"}>{if let Some(e) = &props.error {e.clone() } else { "".to_string() }}</span>
        </>
    }
}
