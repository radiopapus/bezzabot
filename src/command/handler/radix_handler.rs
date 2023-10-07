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
use teloxide::{respond, Bot};

pub async fn radix_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Radix(from, to, value) = cmd else {
        return respond(());
    };

    let from_value = u32::from_str_radix(&value, from);

    if let Err(e) = from_value {
        bot.send_message(
            msg.chat.id,
            format![
                "Ошибка преобразования {e} числа {:?} из {:?} в {:?} систему счисления",
                value, from, to
            ],
        )
        .await?;
        return respond(());
    }

    let value = from_value.unwrap_or_default();

    let result = match to {
        1 => format!["Странная система счисления {}", 1],
        2 => format!("{:b}", value),
        10 => format!("{:?}", value),
        16 => format!("{:0x}", value),
        _ => "Пока только 2, 10, 16 системы счисления".to_string(),
    };

    bot.send_message(msg.chat.id, result).await?;

    respond(())
}
