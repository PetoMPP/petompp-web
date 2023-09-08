use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FlagProps {
    pub country: Country,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Country {
    #[default]
    UnitedKingdom,
    Poland,
}

impl Country {
    pub fn key(&self) -> &str {
        match self {
            Self::UnitedKingdom => "en",
            Self::Poland => "pl",
        }
    }

    pub fn get_current() -> Self {
        for lang in web_sys::window().unwrap().navigator().languages().to_vec() {
            let lang = lang.as_string().unwrap().to_lowercase();
            if lang.len() < 2 {
                continue;
            }
            if let Ok(country) = Self::try_from(&lang[..2]) {
                return country;
            }
        }
        Self::default()
    }
}

impl<'a> TryFrom<&'a str> for Country {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "pl" => Ok(Self::Poland),
            "en" => Ok(Self::UnitedKingdom),
            _ => Err(value),
        }
    }
}

impl TryFrom<String> for Country {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str()).map_err(|s| s.to_string())
    }
}

#[function_component(Flag)]
pub fn flag(props: &FlagProps) -> Html {
    html! {
        <div class={"flex items-center w-12 p-[2px]"}>
            <img src={format!("/img/flags/{}.svg", props.country.key())} />
        </div>
    }
}
