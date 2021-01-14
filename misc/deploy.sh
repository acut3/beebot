#!/bin/bash
#
# Usage:
#   deploy.sh [user@]host
#
# Deploys beebot to the specified target.
#
# Two different users must be able to ssh onto the target host:
#
#   - the user specified in the target definition (or the current user if not
#   specified). This user must be able to use sudo.
#
#   - the "beebot" user
#
#  For security reasons those users should NOT be the same. The "beebot" user
#  should not be able to use sudo!
#

# The user as which the beebot service runs
BEEBOT_USER="beebot"

TARGET=$1
[[ "$TARGET" ]] || {
    echo "Usage: `basename $0` [user@]host" >&2
    exit 1
}

# cd to the top of the current repo
REPO_ROOT=`git rev-parse --show-toplevel` || exit 1
cd $REPO_ROOT

# Build a release binary
cargo build --release || exit 1

# Extract host from target
TARGET_HOST=${1##*@}

# rm before scp since the text will usually be in use, preventing overwrites
ssh "$BEEBOT_USER@$TARGET_HOST" rm bin/beebot || exit 1
scp -C "target/release/beebot" "$BEEBOT_USER@$TARGET_HOST:bin/" || exit 1

# Restart service. This is done as whatever user the target specifies (or the
# current username if a user is not specified), and assumes this user can ssh
# and run sudo commands.
ssh "$TARGET" sudo systemctl restart beebot.service
