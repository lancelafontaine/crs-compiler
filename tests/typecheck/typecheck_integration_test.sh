#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
NUM_TESTS=0
NUM_FAILING_TESTS=0

function run_test {
  cargo run -q -- "$SOURCE_FILE" > /dev/null
  cat "${SCRIPTPATH}/../../logs/${LOGFILE}"
  cat "${SCRIPTPATH}/../../logs/${LOGFILE}" | grep "$EXPECTED_OUTPUT" > /dev/null

  if [ $? == 0 ]; then
    echo -e "${GREEN}✔ ${SOURCE_FILE}${NC}";
  else
    echo -e "${RED}✘ ${SOURCE_FILE}${NC}";
    NUM_FAILING_TESTS=$((NUM_FAILING_TESTS+1))
  fi
  NUM_TESTS=$((NUM_TESTS+1))
}

SOURCE_FILE="${SCRIPTPATH}/source_files/doubly_declared_data_member.crs"
EXPECTED_OUTPUT='TYPECHECK: A data member of a class has been declared twice'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/method_declared_but_not_defined.crs"
EXPECTED_OUTPUT='TYPECHECK: A class method was defined without being declared, or vice-versa'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/method_defined_but_not_declared.crs"
EXPECTED_OUTPUT='TYPECHECK: A class method was defined without being declared, or vice-versa'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/method_defined_but_class_not_declared.crs"
EXPECTED_OUTPUT='TYPECHECK: A class method is defined for a class that does not exist.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/variable_used_but_class_not_declared.crs"
EXPECTED_OUTPUT='TYPECHECK: Attempting to instantiate a class that was not declared.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/variable_declared_twice.crs"
EXPECTED_OUTPUT='TYPECHECK: A variable, function or class was declared twice.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/function_defined_twice.crs"
EXPECTED_OUTPUT='TYPECHECK: A variable, function or class was declared twice.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/class_declared_twice.crs"
EXPECTED_OUTPUT='TYPECHECK: A variable, function or class was declared twice.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/circular_class_dependency_inheritance.crs"
EXPECTED_OUTPUT='TYPECHECK: There is a cyclic dependency between your class inheritance lists or object members.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/circular_class_dependency_data_member.crs"
EXPECTED_OUTPUT='TYPECHECK: There is a cyclic dependency between your class inheritance lists or object members.'
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/circular_class_dependency_both.crs"
EXPECTED_OUTPUT='TYPECHECK: There is a cyclic dependency between your class inheritance lists or object members.'
LOGFILE='0-error.log'
run_test

echo "Number of tests: $NUM_TESTS"
echo "Number of failed tests: $NUM_FAILING_TESTS"
if [ "$NUM_FAILING_TESTS" == "0" ]; then
  exit 0
fi
exit 1

