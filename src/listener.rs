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

use log::info;
use std::convert::Infallible;
use std::env;
use teloxide::update_listeners::webhooks::Options;
use teloxide::update_listeners::{webhooks, UpdateListener};
use teloxide::Bot;

pub async fn setup_listener(bot: Bot) -> impl UpdateListener<Err = Infallible> + Send {
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
