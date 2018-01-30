#!/usr/bin/env bash
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
NUM_TESTS=0
NUM_FAILING_TESTS=0

function run_test {
  cargo run -q -- "$SOURCE_FILE" | grep "$EXPECTED_OUTPUT" > /dev/null
  if [ $? == 0 ]; then
    echo -e "${GREEN}✔ ${SOURCE_FILE}${NC}";
  else
    echo -e "${RED}✘ ${SOURCE_FILE}${NC}";
    NUM_FAILING_TESTS=$((NUM_FAILING_TESTS+1))
  fi
  NUM_TESTS=$((NUM_TESTS+1))
}

SOURCE_FILE="src/lexer/test/source_files/keyword_if.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'if' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_if_with_whitespace.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'if' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_then.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'then' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_else.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'else' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_for.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'for' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_class.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'class' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_get.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'get' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_put.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'put' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_return.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'return' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_program.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'program' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_int.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'int' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_float.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'float' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/keyword_bool.txt"
EXPECTED_OUTPUT="Token { class: '< keyword >', lexeme: 'bool' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/integer_invalid_0123.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/float_invalid_01.23.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/float_invalid_12.340.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/float_invalid_12.34e01.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/float_valid_12345.6789e-123.txt"
EXPECTED_OUTPUT="Token { class: '< float >', lexeme: '12345.6789e-123' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/integer_valid_12345.txt"
EXPECTED_OUTPUT="Token { class: '< int >', lexeme: '12345' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_valid_abc.txt"
EXPECTED_OUTPUT="Token { class: '< identifier >', lexeme: 'abc' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_valid_abc1.txt"
EXPECTED_OUTPUT="Token { class: '< identifier >', lexeme: 'abc1' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_valid_abc_1.txt"
EXPECTED_OUTPUT="Token { class: '< identifier >', lexeme: 'abc_1' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_valid_abc1_.txt"
EXPECTED_OUTPUT="Token { class: '< identifier >', lexeme: 'abc1_' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_invalid__abc1.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_invalid_1abc.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/identifier_invalid__1abc.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/math_operator_div.txt"
EXPECTED_OUTPUT="Token { class: '< math_operator, '/' >', lexeme: '/' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/math_operator_add.txt"
EXPECTED_OUTPUT="Token { class: '< math_operator, '+' >', lexeme: '+' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/math_operator_sub.txt"
EXPECTED_OUTPUT="Token { class: '< math_operator, '-' >', lexeme: '-' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/math_operator_mult.txt"
EXPECTED_OUTPUT="Token { class: '< math_operator, '\*' >', lexeme: '\*' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/math_operator_mod.txt"
EXPECTED_OUTPUT="Token { class: '< math_operator, '%' >', lexeme: '%' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/open_multi_line_comment.txt"
EXPECTED_OUTPUT="Token { class: '< open_multi_line_comment >', lexeme: '/\*' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/single_line_comment.txt"
EXPECTED_OUTPUT="Token { class: '< single_line_comment >', lexeme: '//' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/newline.txt"
EXPECTED_OUTPUT="Token { class: '< newline >', lexeme: '' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/close_multi_line_comment.txt"
EXPECTED_OUTPUT="Token { class: '< close_multi_line_comment >', lexeme: '\*/' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/open_parens.txt"
EXPECTED_OUTPUT="Token { class: '< open_parens >', lexeme: '(' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/close_parens.txt"
EXPECTED_OUTPUT="Token { class: '< close_parens >', lexeme: ')' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/open_curly_brace.txt"
EXPECTED_OUTPUT="Token { class: '< open_curly_brace >', lexeme: '{' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/close_curly_brace.txt"
EXPECTED_OUTPUT="Token { class: '< close_curly_brace >', lexeme: '}' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/relational_operator_==.txt"
EXPECTED_OUTPUT="Token { class: '< relational_operator, '==' >', lexeme: '==' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/assignment_operator.txt"
EXPECTED_OUTPUT="Token { class: '< assignment_operator >', lexeme: '=' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/semicolon.txt"
EXPECTED_OUTPUT="Token { class: '< semicolon >', lexeme: ';' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/logical_operator_and_invalid.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/logical_operator_or_invalid.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
run_test

SOURCE_FILE="src/lexer/test/source_files/logical_operator_and_valid.txt"
EXPECTED_OUTPUT="Token { class: '< logical_operator, and >', lexeme: '&&' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/logical_operator_or_valid.txt"
EXPECTED_OUTPUT="Token { class: '< logical_operator, or >', lexeme: '||' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/logical_operator_not.txt"
EXPECTED_OUTPUT="Token { class: '< logical_operator, not >', lexeme: '!' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/relational_operator_>.txt"
EXPECTED_OUTPUT="Token { class: '< relational_operator, '>' >', lexeme: '>' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/relational_operator_>=.txt"
EXPECTED_OUTPUT="Token { class: '< relational_operator, '>=' >', lexeme: '>=' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/relational_operator_<>.txt"
EXPECTED_OUTPUT="Token { class: '< relational_operator, '<>' >', lexeme: '<>' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/relational_operator_<=.txt"
EXPECTED_OUTPUT="Token { class: '< relational_operator, '<=' >', lexeme: '<=' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/relational_operator_<.txt"
EXPECTED_OUTPUT="Token { class: '< relational_operator, '<' >', lexeme: '<' }"
run_test

SOURCE_FILE="src/lexer/test/source_files/unrecognized_character.txt"
EXPECTED_OUTPUT="Input character is unrecognized."
run_test

SOURCE_FILE="path/does/not/exist.txt"
EXPECTED_OUTPUT="The input file path provided does not exist on the filesystem."
run_test

echo "Number of tests: $NUM_TESTS"
echo "Number of failed tests: $NUM_FAILING_TESTS"
if [ "$NUM_FAILING_TESTS" == "0" ]; then
  exit 0
fi
exit 1
