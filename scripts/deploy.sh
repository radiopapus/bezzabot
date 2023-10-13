#!/bin/bash

set -o errexit -o noclobber -o nounset -o pipefail

BOT_NAME="bezzabot"
ABS_BEZZABOT_PATH="/srv/$BOT_NAME"
RELEASE_BIN="/home/user/github/bezzabot/target/arm-unknown-linux-musleabihf/release/$BOT_NAME"
BEZZABOT_IP="192.168.1.83"
BEZZABOT_SSH_USER="radio"
BEZZABOT_SSH_PORT=2828
BEZZABOT_SSH_HOST=${BEZZABOT_SSH_USER}@${BEZZABOT_IP}

#read -rp "Version: " version
#read -rp "Short description: " description
#
#echo "Making release"
#ghr --replace -n="v${version}" -b="$description" "$version" $RELEASE_BIN

echo "Backup previous version"
ssh -p ${BEZZABOT_SSH_PORT} ${BEZZABOT_SSH_HOST} "cp ${ABS_BEZZABOT_PATH}/${BOT_NAME} ${ABS_BEZZABOT_PATH}/${BOT_NAME}.backup"

echo "Stop bezzabot service"
ssh -p ${BEZZABOT_SSH_PORT} $BEZZABOT_SSH_HOST "systemctl --user stop ${BOT_NAME}.service"

echo "Deploy"
scp -P $BEZZABOT_SSH_PORT $RELEASE_BIN ${BEZZABOT_SSH_HOST}:${ABS_BEZZABOT_PATH}

echo "Start bezzabot service"
ssh -p $BEZZABOT_SSH_PORT $BEZZABOT_SSH_HOST "systemctl --user start ${BOT_NAME}.service"

echo "Check bezzabot service"
ssh -p $BEZZABOT_SSH_PORT $BEZZABOT_SSH_HOST "systemctl --user status ${BOT_NAME}.service"

echo "Finished"
