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
use crate::error::BezzabotError;
use chrono::{NaiveDateTime, TimeZone, Utc};
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};

pub fn unix_timestamp_to_datetime(timestamp: &str) -> Option<NaiveDateTime> {
    let timestamp_sec: i64 = timestamp.parse().unwrap_or(Utc::now().timestamp());
    NaiveDateTime::from_timestamp_opt(timestamp_sec, 0)
}

pub async fn unixtime_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Utime(timestamp) = cmd else {
        return respond(());
    };

    let err = BezzabotError::ParseError(format!("Неверный timestamp {}", timestamp));

    let result = unix_timestamp_to_datetime(&timestamp)
        .ok_or(err)
        .map(|time| {
            Utc::from_utc_datetime(&Utc, &time)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        })
        .unwrap_or_else(|err| err.to_string());

    bot.send_message(msg.chat.id, result).await?;

    respond(())
}
