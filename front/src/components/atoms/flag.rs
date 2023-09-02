use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct FlagProps {
    pub country: Country,
}

#[derive(Clone, PartialEq, Default)]
pub enum Country {
    #[default]
    UnitedKingdom,
    Poland,
}

impl<'a> From<&'a str> for Country {
    fn from(value: &'a str) -> Self {
        match value {
            "pl" => Self::Poland,
            "en" | _ => Self::UnitedKingdom,
        }
    }
}

impl From<String> for Country {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

#[function_component(Flag)]
pub fn flag(props: &FlagProps) -> Html {
    let svg = match &props.country {
        Country::UnitedKingdom => {
            html! {
                <svg viewBox="0 0 60 30">
                    <clipPath id="a">
                        <path d="M0 0v30h60V0z"/>
                    </clipPath>
                    <clipPath id="b">
                        <path d="M30 15h30v15zv15H0zH0V0zV0h30z"/>
                    </clipPath>
                    <g clip-path="url(#a)">
                        <path d="M0 0v30h60V0z" fill="#012169"/>
                        <path d="M0 0l60 30m0-30L0 30" stroke="#fff" stroke-width="6"/>
                        <path d="M0 0l60 30m0-30L0 30" clip-path="url(#b)" stroke="#C8102E" stroke-width="4"/>
                        <path d="M30 0v30M0 15h60" stroke="#fff" stroke-width="10"/><path d="M30 0v30M0 15h60" stroke="#C8102E" stroke-width="6"/>
                    </g>
                </svg>
            }
        }
        Country::Poland => {
            html! {
                <svg viewBox="0 0 16 10">
                    <path fill="#fff" d="M0 0h16v10H0z"/>
                    <path fill="#dc143c" d="M0 5h16v5H0z"/>
                </svg>
            }
        }
    };
    html! {
        <div class={"flex items-center w-12 p-[2px]"}>
            {svg}
        </div>
    }
}
