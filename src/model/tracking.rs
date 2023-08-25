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

use chrono::{DateTime, Utc};
use prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR;
use prettytable::{Cell, Row, Table};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackingHistoryItem {
    date: String,
    human_status: String,
    country_name: String,
    description: String,
    weight: Option<i64>,
    is_in_international_tracking: bool,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackingItem {
    pub destination_country_name: String,
    pub destination_city_name: String,
    pub title: String,
    pub tracking_history_item_list: Vec<TrackingHistoryItem>,
    pub barcode: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DetailedTracking {
    pub tracking_item: TrackingItem,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackingModel {
    pub detailed_trackings: Vec<DetailedTracking>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl TrackingModel {
    pub fn as_text_report(&self) -> String {
        let mut report = String::new();

        if !&self.detailed_trackings.is_empty() {
            let tracking_item = &self.detailed_trackings[0].tracking_item;
            let info = format!(
                "{title} в {destination}",
                title = &tracking_item.title,
                destination = tracking_item.destination_city_name
            );

            report.push_str(&info);
        }

        report.push('\n');

        let mut table = Table::new();

        table.set_format(*FORMAT_NO_BORDER_LINE_SEPARATOR);
        for item in &self.detailed_trackings {
            for history_item in &item.tracking_item.tracking_history_item_list {
                let dt = DateTime::parse_from_rfc3339(&history_item.date).unwrap();
                let formatted_date = dt.format("%d %b %Y").to_string();

                let cells = vec![
                    Cell::new(&formatted_date),
                    Cell::new(&history_item.human_status),
                ];
                table.add_row(Row::new(cells));
            }
        }

        report.push_str(&table.to_string());

        report
    }
}
