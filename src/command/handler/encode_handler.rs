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
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};
use url::form_urlencoded::byte_serialize;

pub async fn encode_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Enc(text, format) = cmd else {
        return respond(());
    };

    let reply = match format {
        EncDecFormat::B64 => general_purpose::URL_SAFE_NO_PAD.encode(text),
        EncDecFormat::Url => byte_serialize(text.as_bytes()).collect(),
    };

    bot.send_message(msg.chat.id, reply).await?;

    respond(())
}
