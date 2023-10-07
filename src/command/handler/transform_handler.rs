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

use crate::error::BezzabotError;
use crate::model::markers::{DavinciMarker, YoutubeTimeTag};
use csv::ReaderBuilder;
use teloxide::net::Download;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::{MediaKind, MessageKind};
use teloxide::{respond, Bot};

pub struct DavinciYoutubeTransformer;

impl<'a> DavinciYoutubeTransformer {
    pub async fn transform(&self, bytes: &[u8]) -> Result<String, BezzabotError> {
        let mut reader = ReaderBuilder::new().from_reader(bytes);
        let mut lines = vec![];

        for davinci_result in reader.deserialize::<DavinciMarker>() {
            let marker = davinci_result.map_err(BezzabotError::CsvError)?;
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

pub async fn transform_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let MessageKind::Common(mc) = msg.kind else {
        return respond(());
    };

    let MediaKind::Document(md) = mc.media_kind else {
        return respond(());
    };

    let caption = md.caption.unwrap_or(String::from("davinci -> yt"));
    let (from, to) = caption.split_once(',').unwrap_or(("davinci", "yt"));

    let mut dst: Vec<u8> = vec![];

    let transformer = DavinciYoutubeTransformer;

    if from == "davinci" && to == "yt" {
        let file = bot.get_file(md.document.file.id).await?;
        bot.download_file(&file.path, &mut dst).await?;
    }

    let result = transformer.transform(&dst).await;

    let transformed = result.unwrap_or_else(|e| e.to_string());

    bot.send_message(msg.chat.id, transformed).await?;

    respond(())
}
