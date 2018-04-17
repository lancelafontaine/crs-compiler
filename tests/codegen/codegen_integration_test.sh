#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
NUM_TESTS=0
NUM_FAILING_TESTS=0

function run_test {
  cargo run -q -- "$SOURCE_FILE" > /dev/null
  cat "${SCRIPTPATH}/../../output.asm" | grep "$EXPECTED_OUTPUT" > /dev/null
  if [ $? == 0 ]; then
    echo -e "${GREEN}✔ ${SOURCE_FILE}${NC}";
  else
    echo -e "${RED}✘ ${SOURCE_FILE}${NC}";
    NUM_FAILING_TESTS=$((NUM_FAILING_TESTS+1))
  fi
  NUM_TESTS=$((NUM_TESTS+1))
}

SOURCE_FILE="${SCRIPTPATH}/source_files/arrays_of_integers.crs"
EXPECTED_OUTPUT="res 480"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/arrays_of_objects.crs"
EXPECTED_OUTPUT="res 10160640"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/arrays_of_objects_with_inheritance.crs"
EXPECTED_OUTPUT="res 10161648"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/arrays_of_objects_with_inheritance_and_object_members.crs"
EXPECTED_OUTPUT="res 10162656"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/arrays_of_objects_with_object_members.crs"
EXPECTED_OUTPUT="res 1008"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/integer.crs"
EXPECTED_OUTPUT="res 4"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/object.crs"
EXPECTED_OUTPUT="res 16"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/object_with_inheritance.crs"
EXPECTED_OUTPUT="res 8"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/object_with_inheritance_and_object_member.crs"
EXPECTED_OUTPUT="res 8"
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/object_with_object_member.crs"
EXPECTED_OUTPUT="res 4"
run_test

echo "Number of tests: $NUM_TESTS"
echo "Number of failed tests: $NUM_FAILING_TESTS"
if [ "$NUM_FAILING_TESTS" == "0" ]; then
  exit 0
fi
exit 1
