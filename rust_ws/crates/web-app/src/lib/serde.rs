use ecow::EcoString;
use serde::{Deserialize, Deserializer, de};
use std::{fmt, str::FromStr};

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub fn empty_ecostring_as_none<'de, D>(deserializer: D) -> Result<Option<EcoString>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = EcoString::deserialize(deserializer)?;
    if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
}
