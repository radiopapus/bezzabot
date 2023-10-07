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

pub type Barcode = String;

const BARCODE_SIZE_RU: usize = 14;
const BARCODE_SIZE_GLOBAL: usize = 13;

pub fn tracking_parser(input: String) -> Result<(Barcode,), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() != 1 {
        return Err(ParseError::IncorrectFormat(
            "Неверный номер трекинга. Номер это число из 14 знаков.".into(),
        ));
    }

    let barcode: Barcode = args[0].chars().filter(|c| !c.is_whitespace()).collect();

    let valid_barcode = (barcode.len() == BARCODE_SIZE_RU || barcode.len() == BARCODE_SIZE_GLOBAL)
        && barcode.chars().all(|c| c.is_alphanumeric());

    if !valid_barcode {
        return Err(ParseError::IncorrectFormat(
            "Неверный номер трекинга. Должен быть 14 значным числом.".into(),
        ));
    }

    Ok((barcode,))
}
