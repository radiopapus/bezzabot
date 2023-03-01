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

use rand::Rng;
use teloxide::utils::command::ParseError;

pub fn winner_parser(input: String) -> Result<(String,), ParseError> {
    match input.len() {
        0 => Ok(("Нужен список id через запятую".into(),)),
        _len => Ok((input,)),
    }
}

pub fn winner(input: String) -> String {
    let args: Vec<&str> = input.split_whitespace().collect();
    let s: Vec<String> = args.iter().map(ToString::to_string).collect();

    s[rand::thread_rng().gen_range(0..s.len())].to_string()
}
