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

pub mod handler;
pub mod parser;

use teloxide::utils::command::BotCommands;

use crate::command::parser::radix_parser::{FromRadix, ToRadix};
use crate::command::parser::switch_keyboard_parser::{FromLanguage, Layout, ToLanguage};
use parser::encdec_parser::encdec_parser;
use parser::encdec_parser::EncDecFormat;
use parser::radix_parser::radix_parser;
use parser::switch_keyboard_parser::switch_keyboard_parser;
use parser::tracking_parser::tracking_parser;
use parser::winner_parser::winner_parser;

#[derive(BotCommands, Debug, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
pub enum BotCommand {
    #[command(
    parse_with = encdec_parser,
    description = r#"Декодирует текст по заданному формату . /dec -f b64 text. Доступные форматы b64, url."#
    )]
    Decode(String, EncDecFormat),

    #[command(
    parse_with = encdec_parser,
    description = r#"Кодирует текст по заданному формату . /enc -f b64 text. Доступные форматы b64, url."#
    )]
    Encode(String, EncDecFormat),

    #[command(description = "Отображает этот текст.")]
    Help,

    #[command(description = "Json pretty print. /jp json_string")]
    Jp(String),

    #[command(description = "Генерирует qr code по тексту. Пример: /qr text")]
    Qr(String),

    #[command(
    parse_with = radix_parser,
    description = "Radix converter. /radix 2 16 1111"
    )]
    Radix(FromRadix, ToRadix, String),

    #[command(
    parse_with = switch_keyboard_parser,
    description = "Превращает йцукен -> qwerty. Пример: /skb <text>."
    )]
    Skb(String, Layout, FromLanguage, ToLanguage),

    #[command(
    parse_with = tracking_parser,
    description = "Статус доставки через почту России. Пример: /tracking номер_заказа"
    )]
    Tracking(String),

    #[command(
        description = "Превращает unix timestamp в дату в формате %Y-%m-%d %H:%M:%S. Пример: /utime ts -> , где ts число секунд с 01 января 1970."
    )]
    Utime(String),

    #[command(
        parse_with = winner_parser,
        description = r#"Выбирает случайный id из списка. Пример: /winner 1 2 3 4 5"#
    )]
    Winner(String),
    // #[command(description = "Запрос в wikipedia. Пример: /wikit простое число.")]
    // Wikit { query: String },
}
