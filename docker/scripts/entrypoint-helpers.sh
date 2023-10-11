#!/bin/bash

#
# ______          _ _
# | ___ \        | (_)
# | |_/ /__ _  __| |_  ___  _ __   __ _ _ __  _   _ ___
# |    // _` |/ _` | |/ _ \| '_ \ / _` | '_ \| | | / __|
# | |\ \ (_| | (_| | | (_) | |_) | (_| | |_) | |_| \__ \
# \_| \_\__,_|\__,_|_|\___/| .__/ \__,_| .__/ \__,_|___/
#                          | |         | |
#                          |_|         |_|
#
# twitch: twitch.tv/radiopapus
# github: https://github.com/radiopapus
# telegram: https://t.me/radiopapus
#
# Отказ от ответственности - Использовать только в образовательных целях. Распространяется "как есть".
#
# Disclaimer - Only for educational purposes.
#
# 2023.
#
#
#

##############################################################
# End of VARIABLES declarations
##############################################################

##############################################################
# FUNCTIONS declarations
##############################################################

abort()
{
    echo $1 >&2
    exit $2
}

function Echo_Message()
{
    printf '%*s\n' "80" '' | tr ' ' -
    echo "> $1"
    printf '%*s\n' "80" '' | tr ' ' -
}

function Run_Build_Bezzabot()
{
    exec cargo build --release --target="$RUST_TARGET"
}

function Run_Bezzabot()
{
    exec cargo run --target="$RUST_TARGET"
}

##############################################################
# End of FUNCTIONS declarations
##############################################################
