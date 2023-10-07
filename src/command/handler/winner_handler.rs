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
use rand::Rng;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};

pub async fn winner_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Winner(input) = cmd else {
        return respond(());
    };

    let args: Vec<&str> = input.split_whitespace().collect();
    let s: Vec<String> = args.iter().map(ToString::to_string).collect();

    let result = s[rand::thread_rng().gen_range(0..s.len())].to_string();

    bot.send_message(msg.chat.id, result).await?;

    respond(())
}
