#!/usr/bin/env bash

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

set -e

. ./docker/scripts/entrypoint-helpers.sh

if [[ "${1}" = "build" ]]
then
    Echo_Message 'Building bezzabot'
    Run_Build_Bezzabot
elif [[ "${1}" = "run" ]]; then
    Echo_Message 'Run bezzabot'
    Run_Bezzabot
else
    exec "$@"
fi
