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

# sshrasp is an alias for ssh connection.
sshrasp "sudo -S dd if=/dev/mmcblk0 bs=8M status=progress | gzip -1 -" | sudo dd of=bezzabot_image.gz bs=8M