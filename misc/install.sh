#!/bin/bash

BEEBOT_USER="beebot"
BEEBOT_HOME="/home/$BEEBOT_USER"

# Create "beebot" user and group
useradd --system --create-home beebot

# Create beebot.service (no overwrite)
cp -n misc/beebot.service /etc/systemd/system/

# Create directories in beebot's home
mkdir -p -m 755 "$BEEBOT_HOME/.config/beebot"
mkdir -p "$BEEBOT_HOME/bin"

# Copy files to beebot's home (no overwrite)
(
    umask 077
    cp -n misc/config.yaml "$BEEBOT_HOME/.config/beebot/"
)
cp -n target/release/beebot "$BEEBOT_HOME/bin/"

# Make beebot the owner of everything in his home
chown -R "$BEEBOT_USER:$BEEBOT_USER" "$BEEBOT_HOME"
