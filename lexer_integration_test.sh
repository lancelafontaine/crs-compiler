#!/usr/bin/env bash
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
NUM_FAILING_TESTS=0

function run_test {
  cargo run -q -- "$SOURCE_FILE" | grep "$EXPECTED_OUTPUT" &> /dev/null
  if [ $? == 0 ]; then
    echo -e "${GREEN}✔ ${SOURCE_FILE}${NC}";
  else
    echo -e "${RED}✔ ${SOURCE_FILE}${NC}";
    NUM_FAILING_TESTS=$((NUM_FAILING_TESTS+1))
  fi
}

# Integration test: 'if'
SOURCE_FILE="src/lexer/test/source_files/if.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'if' }"
run_test

# Integration test: 'if_with_whitespace'
SOURCE_FILE="src/lexer/test/source_files/if_with_whitespace.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'if' }"
run_test


echo "Number of failed tests: $NUM_FAILING_TESTS"
