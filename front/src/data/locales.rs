use crate::{api::error::validation::PasswordRequirements, components::atoms::flag::Country};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strum::{EnumIter, IntoEnumIterator};
use wasm_bindgen::{prelude::Closure, JsCast};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct LocalesStore {
    pub curr: Country,
    #[serde(skip_serializing, skip_deserializing)]
    pl: HashMap<String, String>,
    #[serde(skip_serializing, skip_deserializing)]
    en: HashMap<String, String>,
}

impl Default for LocalesStore {
    fn default() -> Self {
        Self {
            curr: Country::get_current(),
            pl: Default::default(),
            en: Default::default(),
        }
    }
}

impl LocalesStore {
    pub fn is_loaded(&self, country: Country) -> bool {
        match country {
            Country::Poland => !self.pl.is_empty(),
            Country::UnitedKingdom => !self.en.is_empty(),
        }
    }

    pub fn get(&self, key: TK) -> String {
        let val = match self.curr {
            Country::Poland => self.pl.get(&key.to_string()),
            Country::UnitedKingdom => self.en.get(&key.to_string()),
        }
        .cloned()
        .unwrap_or_default();
        match key {
            TK::ActivateUserQuestion(s)
            | TK::DeleteUserQuestion(s)
            | TK::E_Auth_MissingClaim(s)
            | TK::E_Auth_InvalidFormat(s)
            | TK::E_Auth_JwtError(s)
            | TK::E_Database(s)
            | TK::E_DatabaseConnection(s)
            | TK::E_UserNameTaken(s)
            | TK::E_UserNotFound(s)
            | TK::E_UserNotConfirmed(s)
            | TK::E_Validation_Username_InvalidCharacters(s)
            | TK::E_Validation_Query_InvalidColumn(s) => val.replace("%{0}", &s),
            TK::E_Auth_TokenExpiredS(s) => val.replace("%{0}", &s.to_string()),
            TK::E_Validation_Username_InvalidLength(min, max) => val
                .replace("%{0}", &min.to_string())
                .replace("%{1}", &max.to_string()),
            TK::E_Validation_PasswordRequirement(min, max, s) => val
                .replace("%{0}", &min.to_string())
                .replace("%{1}", &max.to_string())
                .replace("%{2}", &s),
            TK::E_Validation_ResourceData_KeyMismatch(exp, act) => {
                val.replace("%{0}", &exp).replace("%{1}", &act)
            }
            TK::E_Validation_Password(pr) => pr.into_localized(self),
            _ => val,
        }
    }

    pub fn load(&mut self, country: Country, data: HashMap<String, String>) {
        let data = data.iter().map(|(k, v)| (k.into(), v.clone())).collect();
        if let Err(diff) = Self::validate_data(&data) {
            gloo::console::warn!(&format!(
                "Invalid data for country {:?}:\nmissing: {:?}\nextra: {:?}",
                country, diff.missing, diff.extra
            ));
        }
        match country {
            Country::Poland => self.pl = data,
            Country::UnitedKingdom => self.en = data,
        };
    }

    pub fn add_lang_change_event_listener(dispatch: Dispatch<Self>) {
        let closure = Closure::wrap(Box::new(move || {
            dispatch.reduce_mut(|state| state.curr = Country::get_current())
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("languagechange", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn validate_data(data: &HashMap<String, String>) -> Result<(), DataDiff> {
        let tks = TK::iter().map(|tk| tk.to_string()).collect::<HashSet<_>>();
        let data_tks = data.keys().cloned().collect::<HashSet<_>>();
        let missing = tks.difference(&data_tks).cloned().collect::<Vec<_>>();
        let extra = data_tks.difference(&tks).cloned().collect::<Vec<_>>();
        match missing.is_empty() && extra.is_empty() {
            true => Ok(()),
            false => Err(DataDiff { missing, extra }),
        }
    }
}

struct DataDiff {
    missing: Vec<String>,
    extra: Vec<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, EnumIter)]
pub enum TK {
    __version,
    Home,
    Projects,
    About,
    Contact,
    Ok,
    Cancel,
    Save,
    SaveChanges,
    SaveChangesQuestion,
    Discard,
    DiscardChanges,
    DiscardChangesQuestion,
    Logout,
    LogoutQuestion,
    Login,
    Register,
    Username,
    TypeUsername,
    Password,
    TypePassword,
    UserManagement,
    Id,
    Name,
    Actions,
    Activate,
    ActivateUserQuestion(String),
    Delete,
    DeleteUserQuestion(String),
    ProjectsDescription,
    Edit,
    Editor,
    Preview,
    SaveDraft,
    Blog,
    E_Auth_MissingClaim(String),
    E_Auth_InvalidFormat(String),
    E_Auth_TokenExpiredS(i32),
    E_Auth_JwtError(String),
    E_Database(String),
    E_DatabaseConnection(String),
    E_UserNameTaken(String),
    E_UserNotFound(String),
    E_InvalidCredentials,
    E_UserNotConfirmed(String),
    E_Validation_Username_InvalidLength(i32, i32),
    E_Validation_Username_InvalidCharacters(String),
    E_Validation_Password(PasswordRequirements),
    E_Validation_PasswordRequirement(i32, i32, String),
    E_Validation_PasswordRequirement_ContainsLowercase,
    E_Validation_PasswordRequirement_ContainsUppercase,
    E_Validation_PasswordRequirement_ContainsNumber,
    E_Validation_PasswordRequirement_ContainsSpecialCharacter,
    E_Validation_Query_InvalidColumn(String),
    E_Validation_ResourceData_KeyMismatch(String, String),
    E_Validation_ResourceData_KeyMissing,
    E_Validation_ResourceData_ValueMissing,
}

impl std::fmt::Display for TK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = format!("{:?}", self);
        if let Some(pos) = val.find('(') {
            write!(f, "{}", &val[..pos])
        } else {
            write!(f, "{}", val)
        }
    }
}
