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

use crate::command::parser::switch_keyboard_parser::{FromLanguage, Layout, ToLanguage};
use crate::command::BotCommand;
use std::char::{ToLowercase, ToUppercase};
use std::collections::HashMap;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};

pub struct SwitchKeyboard {
    pub layout: Layout,
    pub from_lang: FromLanguage,
    pub to_lang: ToLanguage,
}

impl SwitchKeyboard {
    fn mapping_from_string(mapping_value: String) -> HashMap<char, char> {
        mapping_value.lines().fold(HashMap::new(), |mut map, line| {
            let (from, to) = line.split_once(';').unwrap_or_default();
            map.insert(from.single_char_or_default(), to.single_char_or_default());
            map
        })
    }

    pub fn switch_layout(&self, text: String) -> String {
        let mapping = Self::mapping_from_string(String::from(RU_EN_CLASSIC_MAPPING));

        text.chars().fold(String::new(), |mut s, char| {
            let lower_cased_char = &char.to_lowercase().single_char_or_default();
            let mut result = *mapping.get(lower_cased_char).unwrap_or(&char);

            if char.is_uppercase() {
                result = result.to_uppercase().single_char_or_default()
            }

            s.push(result);
            s
        })
    }
}

pub async fn skb_handler(
    bot: Bot,
    msg: Message,
    cmd: BotCommand,
) -> ResponseResult<()> {
    let BotCommand::Skb(text, layout, from_lang, to_lang) = cmd else {
        return respond(());
    };

    let skb = SwitchKeyboard {
        from_lang,
        to_lang,
        layout,
    };

    let result = skb.switch_layout(text);

    bot.send_message(msg.chat.id, result).await?;

    respond(())
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

trait SingleChar {
    fn single_char_or_default(self) -> char;
}

impl SingleChar for &str {
    fn single_char_or_default(self) -> char {
        self.chars().last().unwrap_or_default()
    }
}

impl SingleChar for ToLowercase {
    fn single_char_or_default(self) -> char {
        self.last().unwrap_or_default()
    }
}

impl SingleChar for ToUppercase {
    fn single_char_or_default(self) -> char {
        self.last().unwrap_or_default()
    }
}
