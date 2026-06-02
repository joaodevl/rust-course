#!/bin/bash

CHAL_DESC_FILE="challenge.md"
SOLUTION_NAME="solution"

if [ $# -lt 1 ] || [ $# -gt 2 ]; then
    echo -e "nchal need the name of the challenge to run...\n\nEx: nchal \"<chal name>\" [bin|lib]\n"
    exit 1
fi

chal_name="$1"
proj_type="${2:-bin}"

if [[ "$proj_type" != "bin" && "$proj_type" != "lib" ]]; then 
	echo -e "Error: The second argument must be 'bin' or 'lib'.\n"
	exit 1
fi

echo -e "creating challenge folder for: ${chal_name} (${proj_type}) ...\n"

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

cargo_flags="--quiet"
if [ "$proj_type" == "lib" ]; then
	cargo_flags="--lib --quiet"
fi


mkdir -p "$TARGET_DIR" && touch "$TARGET_DIR/${CHAL_DESC_FILE}" && (cd "$TARGET_DIR" && cargo new "$SOLUTION_NAME" $cargo_flags)

if [ ! -d "${TARGET_DIR}/$SOLUTION_NAME" ] || [ ! -f "${TARGET_DIR}/$CHAL_DESC_FILE" ]; then
    echo -e "Fail to create files...\n"
    exit 1
fi

echo -e "${TARGET_DIR} is created with:\n$SOLUTION_NAME (rust $proj_type cargo generated)\n$CHAL_DESC_FILE"
