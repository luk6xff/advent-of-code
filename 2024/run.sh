#!/bin/bash


SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_WSL_PATH=${SCRIPT_DIR}

INSTALL_DEPENDENCIES_CMD="sudo apt-get update -qq && \
                          curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"

BUILD_CMD="cd src && cargo build"
CLEAN_BUILD_CMD="cargo clean && ${BUILD_CMD}"
UNITTESTS_CMD="cargo test"

# Parse command-line options
while getopts ":wiuUaAh" opt; do
  case $opt in
    w)
      RUN_WITH_WSL=true
      # Determine the Windows path to the current directory
      CURRENT_DIRECTORY=$(pwd)
      PROJECT_WSL_PATH="/mnt/${CURRENT_DIRECTORY:1:1}${CURRENT_DIRECTORY:2}"
      ;;
    i)
      # Install dependencies
      COMMAND="${INSTALL_DEPENDENCIES_CMD}"
      ;;
    u)
      # Clean and Run unit tests
      COMMAND="cd ${PROJECT_WSL_PATH} && ${CLEAN_BUILD_CMD} && ${UNITTESTS_CMD}"
      ;;
    U)
      # Just Run unit tests without cleaning
      COMMAND="cd ${PROJECT_WSL_PATH} && ${BUILD_CMD} && ${UNITTESTS_CMD}"
      ;;
    a)
      # Clean and Run all
      COMMAND="cd ${PROJECT_WSL_PATH} && ${CLEAN_BUILD_CMD} && ${UNITTESTS_CMD}"
      ;;
    A)
      # Just Run all without cleanning
      COMMAND="cd ${PROJECT_WSL_PATH} && ${BUILD_CMD} && ${UNITTESTS_CMD}"
      ;;
    h)
      # Display help
      echo "Usage: $0 [options]"
      echo "Options:"
      echo "-w    Run with WSL"
      echo "-i    Install dependencies"
      echo "-u    Clean and run unit tests"
      echo "-U    Run unit tests without cleaning"
      echo "-a    Clean and run all"
      echo "-A    Run all without cleaning"
      echo "-h    Display this help message"
      exit 0
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done


echo "PROJECT_WSL_PATH: $PROJECT_WSL_PATH"

# Check if WSL execution is requested
ret_code=0
if [ "$RUN_WITH_WSL" = true ]; then
    # Launch WSL and navigate to the project directory
    wsl.exe -e bash -c "$COMMAND"
    ret_code=$?
else
    eval "$COMMAND"
    ret_code=$?
    if [ $ret_code -ne 0 ]; then
        echo "Command: $COMMAND failed with exit code $?"
    fi
fi
exit $ret_code
