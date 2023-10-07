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
use crate::command::BotCommand;
use crate::error::BezzabotError;
use crate::model::tracking::TrackingModel;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::{respond, Bot};

pub struct PostRuService {
    tracking_endpoint: String,
}

impl Default for PostRuService {
    fn default() -> Self {
        Self {
            tracking_endpoint: String::from("https://www.pochta.ru/api/tracking/api/v1/trackings/"),
        }
    }
}

impl PostRuService {
    pub async fn fetch_by_barcode(&self, barcode: String) -> Result<TrackingModel, BezzabotError> {
        let params = format!("by-barcodes?language=ru&track-numbers={barcode}");

        let by_barcode_endpoint = format!("{}{}", self.tracking_endpoint, params);

        let json = reqwest::get(by_barcode_endpoint)
            .await
            .map_err(BezzabotError::FetchError)?
            .text()
            .await
            .map_err(BezzabotError::FetchError)?;

        serde_json::from_str(&json).map_err(BezzabotError::SerdeError)
    }
}

pub async fn tracking_handler(bot: Bot, msg: Message, cmd: BotCommand) -> ResponseResult<()> {
    let BotCommand::Tracking(barcode) = cmd else {
        return respond(());
    };

    let post = PostRuService::default();

    let tracking_result = post.fetch_by_barcode(barcode).await;

    let result = match tracking_result {
        Ok(model) => model.as_text_report(),
        Err(err) => err.to_string(),
    };

    bot.send_message(msg.chat.id, result).await?;

    respond(())
}
