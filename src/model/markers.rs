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

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DavinciMarker {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Notes")]
    pub notes: String,
    #[serde(rename = "Source In")]
    pub start_at: String, // 00:00:02:27
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct YoutubeTimeTag {
    pub time: String,   // HH:mm:ss
    pub marker: String, // marker
}

impl YoutubeTimeTag {
    pub fn from_davinci(davinci: DavinciMarker) -> Self {
        let len = davinci.start_at.len();
        let time = davinci.start_at[0..len - 3].to_string();
        let marker = davinci.name.unwrap_or(davinci.notes.to_string());
        YoutubeTimeTag { time, marker }
    }
}
