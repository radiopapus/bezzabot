/*
 * ______          _ _
 * | ___ \        | (_)
 * | |_/ /__ _  __| |_  ___  _ __   __ _ _ __  _   _ ___
 * |    // _` |/ _` | |/ _ \| '_ \ / _` | '_ \| | | / __|
 * | |\ \ (_| | (_| | | (_) | |_) | (_| | |_) | |_| \__ \
 * \_| \_\__,_|\__,_|_|\___/| .__/ \__,_| .__/ \__,_|___/
 *                          | |         | |
 *                          |_|         |_|
 *
 * twitch: twitch.tv/radiopapus
 * github: https://github.com/radiopapus
 * telegram: https://t.me/radiopapus
 *
 * Отказ от ответственности - Использовать только в образовательных целях. Распространяется "как есть".
 *
 * Disclaimer - Only for educational purposes.
 *
 * 2023.
 *
 *
 */

use crate::command::parser::switch_keyboard_parser::Language::{En, Ru};
use crate::command::parser::switch_keyboard_parser::Layout::{Classic, Dvorak};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;
use teloxide::utils::command::ParseError;

#[derive(Clone, Debug)]
pub enum Language {
    Ru,
    En,
}

pub type FromLanguage = Language;
pub type ToLanguage = Language;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layout {
    Classic,
    Dvorak,
}

/// /skb йцукен {classic, dvorak} {ru} {en}
pub fn switch_keyboard_parser(
    input: String,
) -> Result<(String, Layout, FromLanguage, ToLanguage), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        // only text
        1 => Ok((args[0].to_string(), Classic, Ru, En)),
        // text + layout
        2 => {
            let layout = args[1].try_into().unwrap_or(Classic);
            Ok((args[0].to_string(), layout, Ru, En))
        }
        // text + layout + from_lang
        3 => {
            let layout = args[1].try_into().unwrap_or(Classic);
            let lang_from = args[2].try_into().unwrap_or(Language::Ru);
            Ok((args[0].to_string(), layout, lang_from, Language::En))
        }
        // text + layout + from_lang + to_lang
        4 => {
            let layout = args[1].try_into().unwrap_or(Classic);
            let lang_from = args[2].try_into().unwrap_or(Language::Ru);
            let lang_to = args[3].try_into().unwrap_or(Language::En);
            Ok((args[0].to_string(), layout, lang_from, lang_to))
        }
        _len => Err(ParseError::IncorrectFormat(
            "Пример /skb текст <layout: classic, dvorak> <lang_from: ru>, <lang_to: en>".into(),
        )),
    }
}

impl FromStr for Layout {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "classic" => Ok(Classic),
            "dvorak" => Ok(Dvorak),
            _ => Err("Allowed: classic, dvorak"),
        }
    }
}

impl TryFrom<&str> for Layout {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "classic" => Ok(Classic),
            "dvorak" => Ok(Dvorak),
            _ => Err("Allowed: classic, dvorak"),
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ru" => Ok(Ru),
            "en" => Ok(En),
            _ => Err("Allowed: ru, en"),
        }
    }
}
