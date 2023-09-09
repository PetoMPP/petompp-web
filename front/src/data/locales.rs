use crate::components::atoms::flag::Country;
use std::collections::{HashMap, HashSet};
use strum::{EnumIter, IntoEnumIterator};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store)]
pub struct LocalesStore {
    pub curr: Country,
    pl: HashMap<String, String>,
    en: HashMap<String, String>,
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
            TK::ActivateUserQuestion(s) | TK::DeleteUserQuestion(s) => val.replace("%{}", &s),
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

#[derive(Debug, Clone, EnumIter)]
pub enum TK {
    #[allow(non_camel_case_types)]
    __version,
    Home,
    Projects,
    About,
    Contact,
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
    // TODO: Errors
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
