#!/bin/bash

# This is a temporary run script for local development.
# It automates the process of cleaning cargo build artifacts, syncing code to a remote server,
# and running the lightning-cli on the remote server.
#
# Local Structure:
#   - The script expects a directory structure where 'asn-models', 'open-api', and 'unifyair-core'
#     are subdirectories of the parent directory of the script's location.
#     For example, if this script is in /path/to/unifyair-core, then the parent directory is /path/to.
#     And /path/to/asn-models,  /path/to/open-api, /path/to/unifyair-core should exist.
#
# Remote Structure:
#   - The script assumes a 'unifyair' directory exists in the remote user's home directory.
#   - After syncing, the remote directory structure will mirror the local structure, i.e.,
#     $HOME/unifyair/asn-models, $HOME/unifyair/open-api, and $HOME/unifyair/unifyair-core.
#
# Requirements:
#   - rsync must be installed locally.
#   - ssh access to the remote server must be configured.
#   - cargo must be installed on the remote server.
#
# Usage:
#   ./run.sh <ssh server>
#
#   - <ssh server> is the ssh server to connect to.
#
# Example:
#   ./run.sh user@server.com
#


# Function to execute a command and handle errors
execute_command() {
  local command="$1"
  local error_message="$2"
  echo "$command"
  eval "$command"
  if [ $? -ne 0 ]; then
    echo "Error: $error_message"
    exit 1
  fi
}

# Function to clean cargo build artifacts
clean_cargo() {
  local directory="$1"
  cd "$LOCAL_PARENT_REPO/$directory"
  execute_command "cargo clean" "cargo clean failed in $directory."
}

# Function to sync a directory using rrsync
sync_directory() {
  local directory="$1"
  execute_command "rrsync \"$LOCAL_PARENT_REPO/$directory\" \"$REMOTE_SERVER:$REMOTE_PARENT_REPO\"" "rrsync failed for $directory."
}

if [ $# -ne 1 ]; then
  echo "Usage: $0 <ssh server>"
  exit 1
fi

REMOTE_SERVER="$1"

shopt -s expand_aliases
alias rrsync="rsync -r --info=progress2 --info=name0 --exclude=target/ --exclude=.git/ --exclude=.DS_Store"
LOCAL_PARENT_REPO="$(realpath "..")"

# Check if LOCAL_PARENT_REPO exists
if [ ! -d "$LOCAL_PARENT_REPO" ]; then
  echo "Error: LOCAL_PARENT_REPO directory not found: $LOCAL_PARENT_REPO"
  exit 1
fi

# Check if required directories exist within LOCAL_PARENT_REPO
required_dirs=("asn-models" "open-api" "unifyair-core")
for dir in "${required_dirs[@]}"; do
  if [ ! -d "$LOCAL_PARENT_REPO/$dir" ]; then
    echo "Error: $dir directory not found in $LOCAL_PARENT_REPO"
    exit 1
  fi
done

# Determine REMOTE_HOME using SSH
REMOTE_HOME=$(ssh -o StrictHostKeyChecking=no "$REMOTE_SERVER" "echo \$HOME")
if [ -z "$REMOTE_HOME" ]; then
  echo "Error: Failed to determine remote home directory."
  exit 1
fi

REMOTE_PARENT_REPO="$REMOTE_HOME/unifyair"

# Clean cargo build artifacts
echo "Cargo Clean Directories"
clean_cargo "asn-models"
clean_cargo "open-api"
clean_cargo "unifyair-core"

# Sync directories
echo "Syncing Directories"
sync_directory "asn-models"
sync_directory "open-api"
sync_directory "unifyair-core"

# Execute remote command
execute_command "ssh -t \"$REMOTE_SERVER\" \"cd $REMOTE_PARENT_REPO/unifyair-core; RUST_LOG=\\\"trace\\\" COLORBT_SHOW_HIDDEN=1 RUST_BACKTRACE=full $REMOTE_HOME/.cargo/bin/cargo run -p lightning-cli -- omnipath --config config/amfcfg.yaml\"" "Remote command failed."