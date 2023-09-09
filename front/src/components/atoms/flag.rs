use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FlagProps {
    pub country: Country,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, EnumIter, Display)]
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
        <img src={format!("/img/flags/{}.svg", props.country.key())} class={"flex items-center h-8 w-12 rounded-full"} />
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FlagSelectProps {
    pub country: Country,
    pub onselectedchanged: Option<Callback<Country>>,
}

#[function_component(FlagSelect)]
pub fn flag_select(props: &FlagSelectProps) -> Html {
    let country = use_state(|| props.country.clone());
    let get_onclick = |c: &Country| {
        let changed = props.onselectedchanged.clone();
        let country = country.clone();
        let c = c.clone();
        Callback::from(move |_| {
            country.set(c.clone());
            changed.as_ref().map(|cb| cb.emit(c.clone()));
        })
    };
    html! {
        <div class={"dropdown"}>
            <label tabindex={"0"}>
                <Flag country={(*country).clone()} />
            </label>
            <ul tabindex={"0"} class={"dropdown-content z-[1]"}>
            { for Country::iter()
                .filter(|c| c != &*country)
                .map(|country|
                    html! {
                        <li>
                        <button onclick={get_onclick(&country)} class={"flex flex-row w-max"}>
                            <Flag country={country.clone()} />
                        </button>
                        </li>
                    }
                )
            }
            </ul>
        </div>
    }
}
