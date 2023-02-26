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

use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use teloxide::utils::command::ParseError;

pub type FromLanguage = Language;
pub type ToLanguage = Language;

/// /skb йцукен {classic, dvorak} {ru} {en}
pub fn skb_parser(input: String) -> Result<(String, Layout, FromLanguage, ToLanguage), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();
    match args.len() {
        // only text
        1 => Ok((args[0].to_string(), Layout::Classic, Language::Ru, Language::En)),
        // text + layout
        2 => {
            let layout = args[1].try_into().unwrap_or(Layout::Classic);
            Ok((args[0].to_string(), layout, Language::Ru, Language::En))
        },
        // text + layout + from_lang
        3 => {
            let layout = args[1].try_into().unwrap_or(Layout::Classic);
            let lang_from = args[2].try_into().unwrap_or(Language::Ru);
            Ok((args[0].to_string(), layout, lang_from, Language::En))
        },
        // text + layout + from_lang + to_lang
        4 => {
            let layout = args[1].try_into().unwrap_or(Layout::Classic);
            let lang_from = args[2].try_into().unwrap_or(Language::Ru);
            let lang_to = args[3].try_into().unwrap_or(Language::En);
            Ok((args[0].to_string(), layout, lang_from, lang_to))
        },
        _len => Err(
            ParseError::IncorrectFormat(
                "Incorrect format. Should be /skb <text> <layout: classic, dvorak> <lang_from: ru>, <lang_to: en>".into()
            )
        ),
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layout {
    Classic,
    Dvorak,
}

#[derive(Clone, Debug)]
pub enum Language {
    Ru,
    En,
}

pub struct SwitchKeyboard {
    pub layout: Layout,
    pub from_lang: FromLanguage,
    pub to_lang: ToLanguage,
}

impl SwitchKeyboard {
    fn mapping_from_string(mapping_value: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for line in mapping_value.lines() {
            let (char_ru, char_en) = line.split_once(';').unwrap();
            map.insert(char_ru.to_string(), char_en.to_string());
        }
        map
    }

    pub fn switch_layout(&self, value: String) -> String {
        let mapping = Self::mapping_from_string(RU_EN_CLASSIC_MAPPING.to_string());
        let value = value.trim().to_lowercase();

        let mut converted = String::new();
        for char in value.chars() {
            let char_as_string = String::from(char);
            converted.push_str(mapping.get(&char_as_string).unwrap_or(&char_as_string));
        }

        converted.trim().to_lowercase()
    }
}

impl FromStr for Layout {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "classic" => Ok(Layout::Classic),
            "dvorak" => Ok(Layout::Dvorak),
            _ => Err("Allowed: classic, dvorak"),
        }
    }
}

impl TryFrom<&str> for Layout {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "classic" => Ok(Layout::Classic),
            "dvorak" => Ok(Layout::Dvorak),
            _ => Err("Allowed: classic, dvorak"),
        }
    }
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ru" => Ok(Language::Ru),
            "en" => Ok(Language::En),
            _ => Err("Allowed: ru, en"),
        }
    }
}

const RU_EN_CLASSIC_MAPPING: &str = r#"й;q
ц;w
у;e
к;r
е;t
н;y
г;u
ш;i
щ;o
з;p
х;[
ъ;]
ф;a
ы;s
в;d
а;f
п;g
р;h
о;j
л;k
д;l
ж;;
э;'
я;z
ч;x
с;c
м;v
и;b
т;n
ь;m
б;,
ю;.
.;/
ё;`,
";@,
№;#
;;$
:;^
?;&
/;|
"#;
