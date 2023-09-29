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

use base64::engine::general_purpose;
use base64::Engine;
use bezzabot::command::datetime_from_unix::unix_timestamp_to_datetime;
use bezzabot::command::encdec::EncDecFormat::{Url, B64};
use bezzabot::command::radix::radix;
use bezzabot::command::switch_keyboard::SwitchKeyboard;
use bezzabot::command::tracking::post_ru::PostRu;
use bezzabot::command::winner::winner;
use bezzabot::command::BotCommand;
use log::info;
use qrcode_generator::QrCodeEcc;
use serde_json::Value;
use std::borrow::Cow;
use std::convert::Infallible;
use std::env;
use teloxide::prelude::{Message, ResponseResult};
use teloxide::requests::Requester;
use teloxide::types::{InputFile, Me};
use teloxide::update_listeners::webhooks::Options;
use teloxide::update_listeners::{webhooks, UpdateListener};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;
use url::form_urlencoded::{byte_serialize, parse};
use BotCommand::{Decode, Encode, Help, Jp, Qr, Radix, Skb, Tracking, Utime, Winner};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    info!("Starting bezzabot...");

    let bot = Bot::from_env();
    let listener = setup_listener(bot.clone()).await;

    teloxide::repl_with_listener(bot.clone(), answer, listener).await;
    info!("bezzabot started!");
}

async fn setup_listener(bot: Bot) -> impl UpdateListener<Err = Infallible> + Send {
    info!("Setup webhook...");
    let port = env::var("PORT").expect("Set PORT, please.");
    let port: u16 = port
        .parse()
        .expect("Could not convert port from env var PORT value");

    let addr = ([0, 0, 0, 0], port).into();

    let host_env = env::var("HOST_URL").expect("Set HOST_URL, please.");
    let host = host_env.parse().expect("Incorrect Url format");

    let listener = webhooks::axum(bot, Options::new(addr, host))
        .await
        .expect("Could not setup webhook");

    info!("Listener run on {host_env} and address {addr}");

    listener
}

async fn answer(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    let bot_name = me.username();
    let text = msg.text().unwrap_or("help");

    // todo на текущий момент нет обработки ошибок и отображаем команду Help. Надо бы это исправить и возвращать текст с
    // описание ошибки

    let result = BotCommand::parse(text, bot_name);

    if let Err(e) = result {
        bot.send_message(msg.chat.id, e.to_string()).await?;
        return Ok(());
    }

    let cmd = result.unwrap();

    match cmd {
        Encode(text, format) => {
            let result = match format {
                B64 => general_purpose::URL_SAFE_NO_PAD.encode(text),
                Url => byte_serialize(text.as_bytes()).collect(),
            };
            bot.send_message(msg.chat.id, result).await?
        }

        Decode(text, format) => {
            let result = match format {
                B64 => general_purpose::URL_SAFE_NO_PAD.decode(text),
                Url => {
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

            bot.send_message(msg.chat.id, text).await?
        }

        Tracking(barcode) => {
            let post = PostRu::default();

            let tracking_result = post.fetch_by_barcode(barcode).await;

            if let Err(err) = tracking_result {
                bot.send_message(msg.chat.id, err.to_string()).await?;
                return Ok(());
            }

            let tracking_model = tracking_result.unwrap();

            bot.send_message(msg.chat.id, tracking_model.as_text_report())
                .await?
        }

        Jp(json_string) => {
            // let example_json = r#"{"a": "b", "c" : [1,2,3,4], "d": {"d1": 123, "d2": 3}}"#;

            let value: Value = match serde_json::from_str(&json_string) {
                Ok(v) => v,
                Err(err) => Value::String(err.to_string()),
            };

            let prettified = serde_json::to_string_pretty(&value).unwrap();

            bot.send_message(msg.chat.id, prettified).await?
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
            let result = match unix_timestamp_to_datetime(timestamp) {
                Ok(v) => v,
                Err(err) => err.to_string(),
            };
            bot.send_message(msg.chat.id, result).await?
        }

        Qr { text } => {
            let result: Vec<u8> =
                qrcode_generator::to_png_to_vec(text, QrCodeEcc::Low, 256).unwrap();

            let inner = InputFile::memory(result);

            bot.send_photo(msg.chat.id, inner).await?
        }

        Winner(input) => {
            let result = winner(input);
            bot.send_message(msg.chat.id, result).await?
        }
    };

    Ok(())
}
