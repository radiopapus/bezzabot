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

use teloxide::utils::command::ParseError;
use teloxide::utils::command::ParseError::IncorrectFormat;

pub type FromRadix = u32;
pub type ToRadix = u32;

const INCORRECT_FORMAT_MESSAGE: &str = r#"Неверный формат. Поддерживаются с системы исчесления 2, 10, 16. 
Пример /radix 2 16 1111 - где 2 и 16 - это основания, а 1111 это число в двочиной системе отсчета, 
которое будет преобразовано bin в hex"#;

pub fn radix_parser<'a>(input: String) -> Result<(FromRadix, ToRadix, String), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() != 3 {
        return Err(IncorrectFormat(INCORRECT_FORMAT_MESSAGE.into()));
    }

    let (from_radix, to_radix, value) = (
        args[0]
            .parse::<u32>()
            .map_err(|e| IncorrectFormat(e.into()))?,
        args[1]
            .parse::<u32>()
            .map_err(|e| IncorrectFormat(e.into()))?,
        args[2],
    );

    Ok((from_radix, to_radix, value.into()))
}
