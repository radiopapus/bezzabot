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
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn unix_timestamp_to_datetime(
    timestamp: String, /*, tz: TimeZone*/
) -> Result<String, BezzabotError> {
    let timestamp_sec: i64 = timestamp.parse().unwrap_or(Utc::now().timestamp());
    let naive = NaiveDateTime::from_timestamp_opt(timestamp_sec, 0);
    match naive {
        Some(time) => {
            let dt: DateTime<Utc> = DateTime::from_utc(time, Utc);
            Ok(dt.format("%Y-%m-%d %H:%M:%S").to_string())
        }
        None => Err(BezzabotError::ParseError(format!(
            "Incorrect format for timestamp {}",
            timestamp
        ))),
    }
}
