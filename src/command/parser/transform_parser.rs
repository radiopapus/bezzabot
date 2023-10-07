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

type TransformFrom = String;
type TransformTo = String;

const INCORRECT_FORMAT_MESSAGE: &str =
    r#"Неверный формат. Поддерживаются преобразования davinci -> yt"#;

pub fn transform_parser(input: String) -> Result<(TransformFrom, TransformTo), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() != 2 {
        return Err(ParseError::IncorrectFormat(INCORRECT_FORMAT_MESSAGE.into()));
    }

    let (from, to) = (args[0], args[1]);

    if from != "davinci" && to != "yt" {
        return Err(ParseError::IncorrectFormat(
            "Доступные форматы davinci -> yt".into(),
        ));
    }

    Ok((from.into(), to.into()))
}
