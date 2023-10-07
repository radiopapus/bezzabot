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
use serde_json::Value;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};

// Json pretty print
pub async fn jp_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Jp(json_string) = cmd else {
        return respond(());
    };

    // let example_json = r#"{"a": "b", "c" : [1,2,3,4], "d": {"d1": 123, "d2": 3}}"#;
    let value = serde_json::from_str(&json_string).unwrap_or_else(|e| Value::String(e.to_string()));
    let prettified = serde_json::to_string_pretty(&value).unwrap();

    bot.send_message(msg.chat.id, prettified).await?;

    respond(())
}
