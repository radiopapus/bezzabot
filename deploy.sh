#!/bin/bash

set -o errexit -o noclobber -o nounset -o pipefail

BOT_NAME="bezzabot"
ABS_BEZZABOT_PATH="/srv/$BOT_NAME"
BEZZABOT_IP="192.168.1.83"
BEZZABOT_SSH_PORT=2828

echo "Backup previous version"
ssh -p $BEZZABOT_SSH_PORT radio@$BEZZABOT_IP "cp $ABS_BEZZABOT_PATH/$BOT_NAME $ABS_BEZZABOT_PATH/$BOT_NAME.backup"

echo "Deploy"
scp -P $BEZZABOT_SSH_PORT target/arm-unknown-linux-musleabihf/release/$BOT_NAME radio@$BEZZABOT_IP:$ABS_BEZZABOT_PATH

echo "Finished"