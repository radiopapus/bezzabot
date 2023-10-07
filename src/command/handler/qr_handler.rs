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

use crate::command::BotCommand;
use qrcode_generator::QrCodeEcc;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::InputFile;
use teloxide::{respond, Bot};

pub async fn qr_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Qr(text) = cmd else {
        return respond(());
    };

    let result: Vec<u8> = qrcode_generator::to_png_to_vec(text, QrCodeEcc::Low, 256).unwrap();

    let inner = InputFile::memory(result);

    bot.send_photo(msg.chat.id, inner).await?;

    respond(())
}
