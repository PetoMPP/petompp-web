use crate::components::editor::editor::Editor as EditorInner;
use crate::data::resources::Key;
use crate::pages::page_base::PageBase;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct EditorProps {
    pub reskey: String,
    pub lang: String,
}

impl From<EditorProps> for Key {
    fn from(val: EditorProps) -> Self {
        Key {
            reskey: val.reskey,
            lang: val.lang,
        }
    }
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let reskey: Key = props.clone().into();
    html! {
        <PageBase>
            <EditorInner {reskey} />
        </PageBase>
    }
}
