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

#![allow(clippy::from_str_radix_10)]
use crate::command::radix::Radix::{Bin, Dec, Hex};
use std::i128;
use std::str::FromStr;
use teloxide::utils::command::ParseError;

#[derive(PartialEq, PartialOrd, Debug, Eq, Hash, Clone)]
pub enum Radix {
    Bin,
    Dec,
    Hex,
}

pub type FromRadix = Radix;
pub type ToRadix = Radix;

impl FromStr for Radix {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Bin),
            "10" => Ok(Dec),
            "16" => Ok(Hex),
            _ => Err("not implemented"),
        }
    }
}

const INCORRECT_FORMAT_MESSAGE: &str = r#"Неверный формат. Поддерживаются с системы исчесления 2, 10, 16. 
Пример /radix 2 16 1111 - где 2 и 16 - это основания, а 1111 это число в двочиной системе отсчета, 
которое будет преобразовано в hex"#;

pub fn radix_parser<'a>(input: String) -> Result<(FromRadix, ToRadix, String), ParseError> {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.len() != 3 {
        return Err(ParseError::IncorrectFormat(INCORRECT_FORMAT_MESSAGE.into()));
    }

    let from_radix = Radix::from_str(args[0]);

    if let Err(e) = from_radix {
        return Err(ParseError::IncorrectFormat(e.into()));
    }

    let to_radix = Radix::from_str(args[1]);

    if let Err(e) = to_radix {
        return Err(ParseError::IncorrectFormat(e.into()));
    }

    Ok((from_radix.unwrap(), to_radix.unwrap(), args[2].to_string()))
}

pub fn radix(from_radix: FromRadix, to_radix: ToRadix, value: &str) -> String {
    let value = match from_radix {
        Bin => i128::from_str_radix(value, 2).unwrap_or_default(),
        Dec => i128::from_str_radix(value, 10).unwrap_or_default(),
        Hex => i128::from_str_radix(value, 16).unwrap_or_default(),
    };

    match to_radix {
        Bin => format!("0b{:b}", value),
        Dec => format!("{}", value),
        Hex => format!("0x{:0x}", value),
    }
}
