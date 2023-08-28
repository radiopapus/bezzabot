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
pub mod encdec;
pub mod radix;
pub mod switch_keyboard;
pub mod tracking;
pub mod winner;

use crate::command::encdec::EncDecFormat;
use crate::command::radix::{FromRadix, ToRadix};
use crate::command::switch_keyboard::{FromLanguage, Layout, ToLanguage};
use encdec::encdec_parser;
use radix::radix_parser;
use switch_keyboard::skb_parser;
use teloxide::utils::command::BotCommands;
use tracking::tracking_parser;
use winner::winner_parser;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BezzabotError {
    #[error("fetch data error ")]
    FetchError(#[from] reqwest::Error),
    #[error("serde data error ")]
    SerdeError(#[from] serde_json::Error),
    #[error("parse data error `{0}`")]
    ParseError(String),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
}

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

    #[command(description = "Генерирует qr code по тексту. Пример: /qr text")]
    Qr { text: String },

    #[command(
        parse_with = tracking_parser,
        description = "Статус доставки через почту России. Пример: /tracking номер_заказа"
    )]
    Tracking(String),

    #[command(
        parse_with = winner_parser,
        description = r#"Выбирает случайный id из списка. Пример: /winner 1 2 3 4 5"#
    )]
    Winner(String),

    #[command(
        parse_with = encdec_parser,
        description = r#"Кодирует текст по заданному формату . /enc -f b64 text. Доступные форматы b64, url."#
    )]
    Encode(String, EncDecFormat),

    #[command(
        parse_with = encdec_parser,
        description = r#"Декодирует текст по заданному формату . /dec -f b64 text. Доступные форматы b64, url."#
    )]
    Decode(String, EncDecFormat),

    #[command(description = "Json pretty print. /jp json_string")]
    Jp(String),

    #[command(
        parse_with = radix_parser,
        description = "Radix converter. /radix 2 16 1111"
    )]
    Radix(FromRadix, ToRadix, String),
    // #[command(description = "Запрос в wikipedia. Пример: /wikit простое число.")]
    // Wikit { query: String },
}
