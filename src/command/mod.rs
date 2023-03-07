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

pub mod datetime_from_unix;
pub mod radix;
pub mod switch_keyboard;
pub mod winner;

use crate::command::radix::{FromRadix, ToRadix};
use crate::command::switch_keyboard::{FromLanguage, Layout, ToLanguage};
use radix::radix_parser;
use switch_keyboard::skb_parser;
use teloxide::utils::command::BotCommands;
use winner::winner_parser;

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum BotCommand {
    #[command(description = "Отображает этот текст.")]
    Help,

    #[command(
        parse_with = skb_parser,
        description = "Превращает йцукен -> qwerty. Пример: /skb <text>."
    )]
    Skb(String, Layout, FromLanguage, ToLanguage),

    #[command(
        description = "Превращает unix timestamp в дату в формате %Y-%m-%d %H:%M:%S. Пример: /utime ts -> , где ts число секунд с 01 января 1970."
    )]
    Utime { timestamp: String },

    #[command(
        parse_with = winner_parser,
        description = r#"Выбирает случайный id из списка. Пример: /winner 1 2 3 4 5"#
    )]
    Winner(String),

    #[command(description = "URL safe base64 encoder. /b64e string")]
    B64E(String),

    #[command(description = "URL safe base64 decoder. /b64d string")]
    B64D(String),

    #[command(
        parse_with = radix_parser,
        description = "Radix converter. /radix 2 16 1111"
    )]
    Radix(FromRadix, ToRadix, String),
    // JsonPretty { ::serde_json::to_string_pretty(&obj)
    //     value: String
    // },
    // Jwt { value: String },
    // #[command(description = "Запрос в wikipedia. Пример: /wikit простое число.")]
    // Wikit { query: String },
}
