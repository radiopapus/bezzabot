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

use crate::command::parser::encdec_parser::EncDecFormat;
use crate::command::BotCommand;
use base64::engine::general_purpose;
use base64::Engine;
use std::borrow::Cow;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};
use url::form_urlencoded::parse;

pub async fn decode_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Decode(text, format) = cmd else {
        return respond(());
    };

    let result = match format {
        EncDecFormat::B64 => general_purpose::URL_SAFE_NO_PAD.decode(text),
        EncDecFormat::Url => {
            let decoded: String = parse(text.as_bytes())
                .map(|(key, val)| [key, val].concat())
                .collect();
            Ok(Vec::from(decoded))
        }
    };

    let text = match &result {
        Ok(bytes) => String::from_utf8_lossy(bytes),
        Err(err) => Cow::from(err.to_string()),
    };

    bot.send_message(msg.chat.id, text).await?;

    respond(())
}
