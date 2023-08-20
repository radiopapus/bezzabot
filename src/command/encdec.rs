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

use crate::command::encdec::EncDecFormat::{Url, B64};
use teloxide::utils::command::ParseError;

#[derive(Clone, Debug)]
pub enum EncDecFormat {
    B64,
    Url,
}

pub fn encdec_parser(input: String) -> Result<(String, EncDecFormat), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() <= 2 {
        return Err(ParseError::IncorrectFormat(
            "Should be /enc -f {format} string. f.i.  /enc -f b64 text".into(),
        ));
    }

    let format = match args[1] {
        "b64" => B64,
        "url" => Url,
        _ => {
            return Err(ParseError::IncorrectFormat(
                "Available formats are b64, url.".into(),
            ))
        }
    };

    let string: Vec<String> = args[2..].iter().map(|c| c.to_string()).collect();

    Ok((string.join(" "), format))
}
