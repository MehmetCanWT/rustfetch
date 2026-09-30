use crate::{Info, Module};

pub struct Locale;

impl Module for Locale {
    fn name(&self) -> &'static str {
        "locale"
    }

    fn detect(&self) -> Option<Info> {
        let locale = parse_locale(
            std::env::var("LC_ALL").ok().as_deref(),
            std::env::var("LANG").ok().as_deref(),
        )?;
        Some(Info::new("Locale", locale))
    }
}

pub fn parse_locale(lc_all: Option<&str>, lang: Option<&str>) -> Option<String> {
    let check = |s: Option<&str>| s.map(str::trim).filter(|v| !v.is_empty()).map(String::from);
    check(lc_all).or_else(|| check(lang))
}

pub fn parse_locale_env<F: Fn(&str) -> Option<String>>(get_var: F) -> Option<String> {
    parse_locale(get_var("LC_ALL").as_deref(), get_var("LANG").as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_locale_lc_all_precedence() {
        // LC_ALL overrides LANG
        let result = parse_locale(Some("C.UTF-8"), Some("en_US.UTF-8"));
        assert_eq!(result.as_deref(), Some("C.UTF-8"));
    }

    #[test]
    fn test_parse_locale_lang_fallback() {
        // Standard Fedora Linux default: LC_ALL is unset, LANG is set
        let result = parse_locale(None, Some("en_US.UTF-8"));
        assert_eq!(result.as_deref(), Some("en_US.UTF-8"));

        let result_tr = parse_locale(None, Some("tr_TR.UTF-8"));
        assert_eq!(result_tr.as_deref(), Some("tr_TR.UTF-8"));
    }

    #[test]
    fn test_parse_locale_empty_handling() {
        // Empty strings should be ignored and fall back
        let result = parse_locale(Some("   "), Some("en_US.UTF-8"));
        assert_eq!(result.as_deref(), Some("en_US.UTF-8"));

        assert_eq!(parse_locale(Some(""), Some("")), None);
        assert_eq!(parse_locale(None, None), None);
    }

    #[test]
    fn test_parse_locale_env_helper() {
        let env_map = |key: &str| match key {
            "LANG" => Some("en_GB.UTF-8".to_string()),
            _ => None,
        };
        assert_eq!(parse_locale_env(env_map).as_deref(), Some("en_GB.UTF-8"));
    }
}
