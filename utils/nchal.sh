#!/bin/bash

CHAL_DESC_FILE="challenge.md"
SOLUTION_NAME="solution"

if [ $# -ne 1 ]; then
    echo -e "nchal need the name of the challenge to run...\n\nEx: nchal \"<chal name>\"\n"
    exit 1
fi

chal_name="$1"

echo -e "creating challenge folder for: ${chal_name} ...\n"

fmt_chal_name="${chal_name,,}"
fmt_chal_name="${fmt_chal_name// /_}"

echo -e "getting current challenge number...\n"

chal_n=$(ls -1d [0-9]* 2>/dev/null | cut -d '_' -f1 | sort -n | tail -n1)
chal_n=$(( ${chal_n:-0} + 1 ))

fmt_chal_name="${chal_n}_${fmt_chal_name}"
TARGET_DIR="${fmt_chal_name}"

if [ -d "$TARGET_DIR" ]; then 
    echo -e "${TARGET_DIR} already exists...\n"
    exit 1
fi

mkdir -p "$TARGET_DIR" && touch "$TARGET_DIR/${CHAL_DESC_FILE}" && (cd "$TARGET_DIR" && cargo new "$SOLUTION_NAME" --quiet)

if [ ! -d "${TARGET_DIR}/$SOLUTION_NAME" ] || [ ! -f "${TARGET_DIR}/$CHAL_DESC_FILE" ]; then
    echo -e "Fail to create files...\n"
    exit 1
fi

echo -e "${TARGET_DIR} is created with:\n$SOLUTION_NAME (rust proj cargo generated)\n$CHAL_DESC_FILE"