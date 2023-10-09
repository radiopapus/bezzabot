#!/bin/bash

set -o errexit -o noclobber -o nounset -o pipefail

BOT_NAME="bezzabot"
ABS_BEZZABOT_PATH="/srv/$BOT_NAME"
BEZZABOT_IP="192.168.1.83"
BEZZABOT_SSH_PORT=2828

ssh -p $BEZZABOT_SSH_PORT radio@$BEZZABOT_IP "sudo dd if=/dev/mmcblk0 bs=100M status=progress | gzip -1 -" | sudo dd of=bezzabot_image.gz bs=100M
