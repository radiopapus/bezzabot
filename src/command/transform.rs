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

use crate::command::BezzabotError;
use crate::model::markers::{DavinciMarker, YoutubeTimeTag};
use csv::ReaderBuilder;
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

pub enum TransformData<'a> {
    Bytes(&'a [u8]),
}

pub struct DavinciYoutubeTransformer;

impl<'a> DavinciYoutubeTransformer {
    pub async fn transform(&self, data: TransformData<'a>) -> Result<String, BezzabotError> {
        match data {
            TransformData::Bytes(bytes) => {
                let mut reader = ReaderBuilder::new().from_reader(bytes);
                let mut lines = vec![];

                for davinci in reader.deserialize::<DavinciMarker>() {
                    let marker = davinci.map_err(BezzabotError::CsvError)?;
                    lines.push(YoutubeTimeTag::from_davinci(marker))
                }

                let result = lines
                    .iter()
                    .map(|yt| format!("{} - {}", yt.time, yt.marker))
                    .collect::<Vec<String>>()
                    .join("\n");
                Ok(result)
            }
        }
    }
}
