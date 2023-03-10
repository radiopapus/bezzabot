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

#![allow(clippy::or_fun_call)]

mod command;

use crate::command::datetime_from_unix::unix_timestamp_to_datetime;
use crate::command::radix::radix;
use crate::command::switch_keyboard::SwitchKeyboard;
use crate::command::winner::winner;
use crate::command::BotCommand;
use base64::engine::general_purpose;
use base64::Engine;
use log::info;
use std::convert::Infallible;
use std::env;
use teloxide::prelude::{Message, ResponseResult};
use teloxide::requests::Requester;
use teloxide::types::Me;
use teloxide::update_listeners::webhooks::Options;
use teloxide::update_listeners::{webhooks, UpdateListener};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use BotCommand::{Help, Radix, Skb, Utime, Winner, B64D, B64E};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    info!("Starting bezzabot...");

    let bot = Bot::from_env();
    let listener = setup_webhook(bot.clone()).await;

    teloxide::repl_with_listener(bot, answer, listener).await;
}

async fn setup_webhook(bot: Bot) -> impl UpdateListener<Err = Infallible> + Send {
    info!("Setup webhook...");
    let port = env::var("PORT").expect("Set PORT, please.");
    let port: u16 = port
        .parse()
        .expect("Could not convert port from env var PORT value");

    let addr = ([0, 0, 0, 0], port).into();

    let host = env::var("HOST_URL").expect("Set HOST_URL, please.");
    let host = host.parse().expect("Incorrect Url format");

    webhooks::axum(bot.clone(), Options::new(addr, host))
        .await
        .expect("Could not setup webhook")
}

async fn answer(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    let bot_name = me.username();
    let text = msg.text().unwrap_or("help");

    let cmd = BotCommand::parse(text, bot_name).unwrap_or(Help);

    match cmd {
        B64E(input) => {
            let result = general_purpose::URL_SAFE_NO_PAD.encode(input);
            bot.send_message(msg.chat.id, result).await?
        }
        B64D(input) => {
            let result = general_purpose::URL_SAFE_NO_PAD
                .decode(input)
                .unwrap_or_default();

            bot.send_message(msg.chat.id, String::from_utf8_lossy(&result))
                .await?
        }

        Help => {
            bot.send_message(msg.chat.id, BotCommand::descriptions().to_string())
                .await?
        }

        Skb(text, layout, from_lang, to_lang) => {
            let skb = SwitchKeyboard {
                layout,
                from_lang,
                to_lang,
            };
            let result = skb.switch_layout(text);
            bot.send_message(msg.chat.id, result).await?
        }

        Radix(from, to, value) => {
            let result = radix(from, to, value.as_str());
            bot.send_message(msg.chat.id, result).await?
        }

        Utime { timestamp } => {
            let result = unix_timestamp_to_datetime(timestamp);
            bot.send_message(msg.chat.id, result).await?
        }
        Winner(input) => {
            let result = winner(input);
            bot.send_message(msg.chat.id, result).await?
        }
    };

    Ok(())
}
