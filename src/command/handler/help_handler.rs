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
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::utils::command::BotCommands;
use teloxide::{respond, Bot};

pub async fn help_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Help = cmd else {
        return respond(());
    };

    bot.send_message(msg.chat.id, BotCommand::descriptions().to_string())
        .await?;

    respond(())
}
