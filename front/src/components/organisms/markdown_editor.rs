use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::atoms::markdown::MarkdownDisplay;

#[function_component(MarkdownEditor)]
pub fn markdown_editor() -> Html {
    let (store, _) = use_store::<EditorStore>();
    let class = match store.vertical {
        true => "flex flex-col w-full bg-base-100 rounded-b-2xl",
        false => "flex flex-row w-full bg-base-100 rounded-b-2xl",
    };
    html! {
        <div class={"rounded-2xl bg-base-200 flex flex-col border border-base-300 shadow-2xl min-h-full grow"}>
            <div class={"flex flex-col w-full h-full"}>
                <div class={"flex flex-row rounded-t-2xl justify-between p-2 gap-1 items-center"}>
                    <p class={"text-2xl"}>{"Markdown Editor"}</p>
                    <MarkdownEditorControl />
                </div>
                <div class={class}>
                    <Editor />
                    <Display />
                </div>
            </div>
        </div>
    }
}

#[derive(Store, Clone, Debug, PartialEq, Default)]
struct EditorStore {
    value: String,
    vertical: bool,
}

#[function_component(MarkdownEditorControl)]
fn markdown_editor_control() -> Html {
    let (_, dispatch) = use_store::<EditorStore>();
    let onclick = {
        Callback::from(move |_| {
            dispatch.reduce_mut(|s| {
                s.vertical = !s.vertical;
            });
        })
    };
    html! {
        <div class={"flex flex-row gap-2"}>
            <button class={"btn btn-primary btn-xs btn-outline"} {onclick}>{"Flip"}</button>
            <button class={"btn btn-success btn-xs btn-outline"}>{"Save"}</button>
            <button class={"btn btn-warning btn-xs btn-outline"}>{"Discard"}</button>
        </div>
    }
}

#[function_component(Editor)]
fn editor() -> Html {
    let (store, dispatch) = use_store::<EditorStore>();
    let onchange = {
        let dispatch = dispatch.clone();
        Callback::from(move |e: Event| {
            let element: HtmlInputElement = e.target_unchecked_into();
            dispatch.reduce_mut(|s| {
                s.value = element.value();
            });
        })
    };
    let onkeydown = Callback::from(|e: KeyboardEvent| {
        let element: HtmlInputElement = e.target_unchecked_into();
        if e.key() == "Tab" {
            e.prevent_default();
            let start = element
                .selection_start()
                .unwrap_or_default()
                .unwrap_or_default();
            let end = element
                .selection_end()
                .unwrap_or_default()
                .unwrap_or_default();
            let value = element.value();
            let new_value = format!(
                "{}{}{}",
                &value.as_str()[..(start as usize)],
                "    ",
                &value[(end as usize)..]
            );
            element.set_value(new_value.as_str());
            element.set_selection_start(Some(start + 4)).unwrap();
            element.set_selection_end(Some(start + 4)).unwrap();
        }
    });
    let onkeyup = {
        let dispatch = dispatch.clone();
        Callback::from(move |e: KeyboardEvent| {
            let element: HtmlInputElement = e.target_unchecked_into();
            element.set_attribute("style", "height: 0px;").unwrap();
            element
                .set_attribute(
                    "style",
                    format!("height: {}px;", element.scroll_height()).as_str(),
                )
                .unwrap();

            dispatch.reduce_mut(|s| {
                s.value = element.value();
            });
        })
    };
    let class = match store.vertical {
        true => "flex flex-col m-6 border border-base-300 shadow-lg rounded-lg",
        false => "flex flex-col m-6 border border-base-300 shadow-lg rounded-lg w-1/2",
    };
    html! {
        <div {class}>
            <EditorControl />
            <textarea {onchange} {onkeydown} {onkeyup} class={"textarea rounded-none rounded-b-lg overflow-hidden resize-none leading-normal"}></textarea>
        </div>
    }
}

#[function_component(EditorControl)]
fn editor_control() -> Html {
    html! {
        <div class={"flex flex-row gap-2 rounded-t-lg px-4 bg-base-200"}>
            <button class={"btn btn-primary btn-outline btn-xs my-3"}>{"Link"}</button>
            <button class={"btn btn-primary btn-outline btn-xs my-3"}>{"Italic"}</button>
            <button class={"btn btn-primary btn-outline btn-xs my-3"}>{"Bold"}</button>
            <button class={"btn btn-primary btn-outline btn-xs my-3"}>{"Img"}</button>
        </div>
    }
}

#[function_component(Display)]
fn display() -> Html {
    let (store, _) = use_store::<EditorStore>();
    let value = match store.value.as_str() {
        "" => "# Hello World!".to_string(),
        val => val.to_string(),
    };
    let class = match store.vertical {
        true => "border border-base-300 shadow-lg rounded-lg m-6",
        false => "border border-base-300 shadow-lg rounded-lg m-6 w-1/2",
    };

    html! {
        <div {class}>
            <div class={"flex flex-row gap-2 rounded-t-lg px-4 bg-base-200"}>
                <p class={"my-3"}>{"Preview"}</p>
            </div>
            <div class={"p-2"}>
            <MarkdownDisplay markdown={value} />
            </div>
        </div>
    }
}
