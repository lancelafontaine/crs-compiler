#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
NUM_TESTS=0
NUM_FAILING_TESTS=0

function run_test {
  cargo run -q -- "$SOURCE_FILE" > /dev/null
  cat "${SCRIPTPATH}/../../logs/${LOGFILE}" | grep "$EXPECTED_OUTPUT" > /dev/null

  if [ $? == 0 ]; then
    echo -e "${GREEN}✔ ${SOURCE_FILE}${NC}";
  else
    echo -e "${RED}✘ ${SOURCE_FILE}${NC}";
    NUM_FAILING_TESTS=$((NUM_FAILING_TESTS+1))
  fi
  NUM_TESTS=$((NUM_TESTS+1))
}

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_if.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"if\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_if_with_whitespace.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"if\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_then.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"then\\" }'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_else.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"else\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_for.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"for\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_class.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"class\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_get.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"get\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_put.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"put\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_return.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"return\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_program.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"program\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_int.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"int\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_float.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"float\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/keyword_bool.txt"
EXPECTED_OUTPUT='Token { class: Keyword, lexeme: \\"bool\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/integer_invalid_with_letters.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/integer_invalid_with_(.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/integer_invalid_with_[.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/integer_invalid_0123.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_invalid_01.23.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_invalid_12.340.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_invalid_12.34e01.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_invalid_with_(.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_invalid_with_[.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_invalid_with_letters.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/float_valid_12345.6789e-123.txt"
EXPECTED_OUTPUT='Token { class: Float, lexeme: \\"12345.6789e-123\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/integer_valid_12345.txt"
EXPECTED_OUTPUT='Token { class: Integer, lexeme: \\"12345\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_valid_abc.txt"
EXPECTED_OUTPUT='Token { class: Identifier, lexeme: \\"abc\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_valid_abc1.txt"
EXPECTED_OUTPUT='Token { class: Identifier, lexeme: \\"abc1\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_valid_abc_1.txt"
EXPECTED_OUTPUT='Token { class: Identifier, lexeme: \\"abc_1\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_valid_abc1_.txt"
EXPECTED_OUTPUT='Token { class: Identifier, lexeme: \\"abc1_\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_invalid__abc1.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_invalid_1abc.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/identifier_invalid__1abc.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/math_operator_div.txt"
EXPECTED_OUTPUT='Token { class: MathOperator, lexeme: \\"/\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/math_operator_add.txt"
EXPECTED_OUTPUT='Token { class: MathOperator, lexeme: \\"+\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/math_operator_sub.txt"
EXPECTED_OUTPUT='Token { class: MathOperator, lexeme: \\"-\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/math_operator_mult.txt"
EXPECTED_OUTPUT='Token { class: MathOperator, lexeme: \\"\*\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/math_operator_mod.txt"
EXPECTED_OUTPUT='Token { class: MathOperator, lexeme: \\"%\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/open_parens.txt"
EXPECTED_OUTPUT='Token { class: OpenParens, lexeme: \\"(\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/close_parens.txt"
EXPECTED_OUTPUT='Token { class: CloseParens, lexeme: \\")\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/open_curly_brace.txt"
EXPECTED_OUTPUT='Token { class: OpenCurlyBrace, lexeme: \\"{\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/close_curly_brace.txt"
EXPECTED_OUTPUT='Token { class: CloseCurlyBrace, lexeme: \\"}\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/relational_operator_==.txt"
EXPECTED_OUTPUT='Token { class: RelationalOperator, lexeme: \\"==\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/assignment_operator.txt"
EXPECTED_OUTPUT='Token { class: AssignmentOperator, lexeme: \\"=\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/semicolon.txt"
EXPECTED_OUTPUT='Token { class: Semicolon, lexeme: \\";\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_&_invalid.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_|_invalid.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_&&_valid.txt"
EXPECTED_OUTPUT='Token { class: BinaryLogicalOperator, lexeme: \\"&&\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_keyword_and.txt"
EXPECTED_OUTPUT='Token { class: BinaryLogicalOperator, lexeme: \\"and\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_||_valid.txt"
EXPECTED_OUTPUT='Token { class: BinaryLogicalOperator, lexeme: \\"||\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_keyword_or.txt"
EXPECTED_OUTPUT='Token { class: BinaryLogicalOperator, lexeme: \\"or\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_!.txt"
EXPECTED_OUTPUT='Token { class: UnaryLogicalOperator, lexeme: \\"!\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/logical_operator_keyword_not.txt"
EXPECTED_OUTPUT='Token { class: UnaryLogicalOperator, lexeme: \\"not\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/relational_operator_>.txt"
EXPECTED_OUTPUT='Token { class: RelationalOperator, lexeme: \\">\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/relational_operator_>=.txt"
EXPECTED_OUTPUT='Token { class: RelationalOperator, lexeme: \\">=\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/relational_operator_<>.txt"
EXPECTED_OUTPUT='Token { class: RelationalOperator, lexeme: \\"<>\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/relational_operator_<=.txt"
EXPECTED_OUTPUT='Token { class: RelationalOperator, lexeme: \\"<=\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/relational_operator_<.txt"
EXPECTED_OUTPUT='Token { class: RelationalOperator, lexeme: \\"<\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/unrecognized_character.txt"
EXPECTED_OUTPUT="Input character is unrecognized."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/accessor_operator_valid.txt"
EXPECTED_OUTPUT='Token { class: AccessorOperator, lexeme: \\".\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/accessor_operator_valid_after_identifier.txt"
EXPECTED_OUTPUT='Token { class: AccessorOperator, lexeme: \\".\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/accessor_operator_valid_after_array.txt"
EXPECTED_OUTPUT='Token { class: AccessorOperator, lexeme: \\".\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/open_square_bracket.txt"
EXPECTED_OUTPUT='Token { class: OpenSquareBracket, lexeme: \\"\[\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/close_square_bracket.txt"
EXPECTED_OUTPUT='Token { class: CloseSquareBracket, lexeme: \\"]\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_valid.txt"
EXPECTED_OUTPUT='Token { class: InheritanceOperator, lexeme: \\":\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_valid_class_with_spaces.txt"
EXPECTED_OUTPUT='Token { class: InheritanceOperator, lexeme: \\":\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_valid_class_without_spaces.txt"
EXPECTED_OUTPUT='Token { class: InheritanceOperator, lexeme: \\":\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_valid_with_comment.txt"
EXPECTED_OUTPUT='Token { class: InheritanceOperator, lexeme: \\":\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_valid_with_newline.txt"
EXPECTED_OUTPUT='Token { class: InheritanceOperator, lexeme: \\":\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_open_parens.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_close_parens.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_open_curly_brace.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_close_curly_brace.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_open_square_bracket.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_close_square_bracket.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_+.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_-.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_*.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_%.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_semicolon.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_=.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_<.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_>.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_&.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_|.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_!.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_!.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/inheritance_operator_invalid_with_accessor_operator.txt"
EXPECTED_OUTPUT="Invalid syntax: error state reached. Resetting FSM to initial state."
LOGFILE='0-error.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/scope_resolution_operator_valid.txt"
EXPECTED_OUTPUT='Token { class: ScopeResolutionOperator, lexeme: \\"::\\" }'
LOGFILE='1-token.log'
run_test

SOURCE_FILE="${SCRIPTPATH}/source_files/scope_resolution_operator_valid_with_namespace.txt"
EXPECTED_OUTPUT='Token { class: ScopeResolutionOperator, lexeme: \\"::\\" }'
LOGFILE='1-token.log'
run_test

echo "Number of tests: $NUM_TESTS"
echo "Number of failed tests: $NUM_FAILING_TESTS"
if [ "$NUM_FAILING_TESTS" == "0" ]; then
  exit 0
fi
exit 1
