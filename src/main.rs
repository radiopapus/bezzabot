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

use bezzabot::command::handler::decode_handler::decode_handler;
use bezzabot::command::handler::encode_handler::encode_handler;
use bezzabot::command::handler::help_handler::help_handler;
use bezzabot::command::handler::jp_handler::jp_handler;
use bezzabot::command::handler::qr_handler::qr_handler;
use bezzabot::command::handler::radix_handler::radix_handler;
use bezzabot::command::handler::switch_keyboard_handler::skb_handler;
use bezzabot::command::handler::tracking_handler::tracking_handler;
use bezzabot::command::handler::transform_handler::transform_handler;
use bezzabot::command::handler::unixtime_handler::unixtime_handler;
use bezzabot::command::handler::winner_handler::winner_handler;
use bezzabot::command::BotCommand;
use bezzabot::listener::setup_listener;
use log::info;
use teloxide::dispatching::{Dispatcher, HandlerExt, UpdateFilterExt};
use teloxide::dptree::{case, endpoint};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::prelude::{Message, Update};
use teloxide::requests::Requester;
use teloxide::{dptree, respond, Bot};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    info!("Starting bezzabot...");

    let bot = Bot::from_env();

    // настраиваем веб-сервер
    let listener = setup_listener(bot.clone()).await;

    // настраиваем обработчики команд
    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<BotCommand>()
                .branch(case![BotCommand::Decode(text, format)].endpoint(decode_handler))
                .branch(case![BotCommand::Encode(text, format)].endpoint(encode_handler))
                .branch(case![BotCommand::Help].endpoint(help_handler))
                .branch(case![BotCommand::Jp(text)].endpoint(jp_handler))
                .branch(case![BotCommand::Qr(text)].endpoint(qr_handler))
                .branch(case![BotCommand::Radix(from, to, value)].endpoint(radix_handler))
                .branch(
                    case![BotCommand::Skb(text, layout, from_lang, to_lang)].endpoint(skb_handler),
                )
                .branch(case![BotCommand::Tracking(barcode)].endpoint(tracking_handler))
                .branch(case![BotCommand::Utime(timestamp)].endpoint(unixtime_handler))
                .branch(case![BotCommand::Winner(input)].endpoint(winner_handler)),
        )
        // команда отличается от остальных, обрабатывает файл csv. Можно просто кинуть файл в тг бот разберется
        // что это transform.
        .branch(endpoint(transform_handler))
        .branch(endpoint(|msg: Message, bot: Bot| async move {
            info!("Error handler");
            bot.send_message(
                msg.chat.id,
                "Бот не понял команду. Проверьте команду и параметры или наберите /help",
            )
            .await?;
            respond(())
        }));

    // объединяем все и настраиваем диспетчер
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;

    info!("bezzabot started!");
}
