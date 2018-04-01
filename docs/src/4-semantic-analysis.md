# `crs` Semantic Analysis

## Syntax-Directed Translation

In order to perform semantic analysis of our code as well as ultimately generate an abstract syntax tree (AST), the LL(1) context-free grammar derived during the parsing stage was transformed into an attribute grammar. This involved the addition of semantic actions to the right-hand sides of our grammar's productions, which ultimately represent the execution of a subroutine to accumulate and process semantic values or semantic attributes about the program being parsed. These values can have properties such as type, value, memory size or location, translated code, correctness, or more.

Semantic values are accumulated in semantic records until they can be processed by further semantic actions. However, *attribute migration* will be required given that these semantic rules and values will need to be available throughout the parsing step.

Semantic actions are associated with nodes within the parse tree:
- Semantic actions at the leaves of the tree typically gather semantic values, from either the token type of the node, or through the type recorded in a symbol table for a token already seen
- Semantic actions at intermediate nodes ultimately use and validate these semantic values and pass the information up the tree through attribute migration.

In table-driven parsers, semantic action symbols are inserted into the RHS of productions and pushed onto their own _semantic stack_. Executing a semantic action typically pops a semantic record from the semantic stack, does some processing, then pushes a semantic record back onto the stack.

## Attribute Grammar

The LL(1) CFG obtained during parsing is now shown belong but with additional attributes in the right-hand sides of productions.


| Prediction Number | Production LHS Non-Terminal | &rarr; | Production RHS Expression |
|:--------:|:-----------|:-----------:|:-----------|:-----------:|
| 1 | _Program_ | &rarr; | _ClassDeclarationRecursion_ _FunctionDefinitionRecursion_ `program` _FunctionBody_ `;` |
| 2 | _AdditiveOperator_ | &rarr; | `+` |
| 3 | _AdditiveOperator_ | &rarr; | `-` |
| 4 | _AdditiveOperator_ | &rarr; | `or` |
| 5 | _AdditiveOperator_ | &rarr; | `¦¦` |
| 6 | _ArithmeticExpression_ | &rarr; | _Term_ _ArithmeticExpressionTail_ |
| 7 | _ArithmeticExpressionTail_ | &rarr; | _AdditiveOperator_ _Term_ _ArithmeticExpressionTail_ |
| 8 | _ArithmeticExpressionTail_ | &rarr; | &epsilon; |
| 9 | _ArithmeticOrRelationalExpression_ | &rarr; | _RelationalOperator_ _ArithmeticExpression_ |
| 10 | _ArithmeticOrRelationalExpression_ | &rarr; | &epsilon; |
| 11 | _ArraySize_ | &rarr; | `[` `intNum` `]` |
| 12 | _ArraySizeRecursion_ | &rarr; | _ArraySize_ _ArraySizeRecursion_ |
| 13 | _ArraySizeRecursion_ | &rarr; | &epsilon; |
| 14 | _AssignmentStatement_ | &rarr; | _Variable_ `=` _Expression_ |
| 15 | _ClassDeclaration_ | &rarr; | `class` `id` _OptionalInheritanceList_ `{` _VariableThenFunctionDeclarationRecursion_ `}` `;` |
| 16 | _ClassDeclarationRecursion_ | &rarr; | _ClassDeclaration_ _ClassDeclarationRecursion_ |
| 17 | _ClassDeclarationRecursion_ | &rarr; | &epsilon; |
| 18 | _Expression_ | &rarr; | _ArithmeticExpression_ _ArithmeticOrRelationalExpression_ |
| 19 | _Factor_ | &rarr; | `(` ArithmeticExpression `)` |
| 20 | _Factor_ | &rarr; | _FunctionCallOrVariable_ |
| 21 | _Factor_ | &rarr; | _NegationOperator_ _Factor_ |
| 22 | _Factor_ | &rarr; | _NumberSign_ _Factor_ |
| 23 | _Factor_ | &rarr; | `floatNum` |
| 24 | _Factor_ | &rarr; | `intNum` |
| 25 | _FunctionArguments_ | &rarr; | _Expression_ _FunctionArgumentsTailRecursion_ |
| 26 | _FunctionArguments_ | &rarr; | &epsilon; |
| 27 | _FunctionArgumentsTail_ | &rarr; | `,` _Expression_ |
| 28 | _FunctionArgumentsTailRecursion_ | &rarr; | _FunctionArgumentsTail_ _FunctionArgumentsTailRecursion_ |
| 29 | _FunctionArgumentsTailRecursion_ | &rarr; | &epsilon; |
| 30 | _FunctionBody_ | &rarr; | `{` _VariableDeclarationRecursionThenStatementRecursionA_ `}` |
| 31 | _FunctionCallOrVariable_ | &rarr; | `id` _FunctionCallOrVariableTail_ |
| 32 | _FunctionCallOrVariableTail_ | &rarr; | _FunctionCallParensOrIndexing_ _FunctionCallOrVariableTailRecursion_ |
| 33 | _FunctionCallOrVariableTailRecursion_ | &rarr; | `.` `id` _FunctionCallOrVariableTail_ |
| 34 | _FunctionCallOrVariableTailRecursion_ | &rarr; | &epsilon; |
| 35 | _FunctionCallParensOrIndexing_ | &rarr; | `(` _FunctionArguments_ `)` |
| 36 | _FunctionCallParensOrIndexing_ | &rarr; | _IndexingRecursion_ |
| 37 | _FunctionDeclarationRecursionStart_ | &rarr; | _Type_ `id` _FunctionDeclarationRecursionTail_ |
| 38 | _FunctionDeclarationRecursionStart_ | &rarr; | &epsilon; |
| 39 | _FunctionDeclarationRecursionTail_ | &rarr; | `(` _FunctionParameters_ `)` `;` _FunctionDeclarationRecursionStart_ |
| 40 | _FunctionDefinition_ | &rarr; | _FunctionHeader_ _FunctionBody_ `;` |
| 41 | _FunctionDefinitionRecursion_ | &rarr; | _FunctionDefinition_ _FunctionDefinitionRecursion_ |
| 42 | _FunctionDefinitionRecursion_ | &rarr; | &epsilon; |
| 43 | _FunctionHeader_ | &rarr; | _Type_ _OptionalNamespacing_ `(` _FunctionParameters_ `)` |
| 44 | _FunctionParameters_ | &rarr; | _Type_ `id` _ArraySizeRecursion_ _FunctionParametersTailRecursion_ |
| 45 | _FunctionParameters_ | &rarr; | &epsilon; |
| 46 | _FunctionParametersTail_ | &rarr; | `,` _Type_ `id` _ArraySizeRecursion_ |
| 47 | _FunctionParametersTailRecursion_ | &rarr; | _FunctionParametersTail_ _FunctionParametersTailRecursion_ |
| 48 | _FunctionParametersTailRecursion_ | &rarr; | &epsilon; |
| 49 | _IdListRecursion_ | &rarr; | `,` `id` _IdListRecursion_ |
| 50 | _IdListRecursion_ | &rarr; | &epsilon; |
| 51 | _Indexing_ | &rarr; | `[` _ArithmeticExpression_ `]` |
| 52 | _IndexingRecursion_ | &rarr; | _Indexing_ _IndexingRecursion_ |
| 53 | _IndexingRecursion_ | &rarr; | &epsilon; |
| 54 | _MultiplicativeOperator_ | &rarr; | `&&` |
| 55 | _MultiplicativeOperator_ | &rarr; | `*` |
| 56 | _MultiplicativeOperator_ | &rarr; | `/` |
| 57 | _MultiplicativeOperator_ | &rarr; | `and` |
| 58 | _NegationOperator_ | &rarr; | `!` |
| 59 | _NegationOperator_ | &rarr; | `not` |
| 60 | _NumberSign_ | &rarr; | `+` |
| 61 | _NumberSign_ | &rarr; | `-` |
| 62 | _NumberType_ | &rarr; | `float` |
| 63 | _NumberType_ | &rarr; | `int` |
| 64 | _OptionalInheritanceList_ | &rarr; | `:` `id` _IdListRecursion_ |
| 65 | _OptionalInheritanceList_ | &rarr; | &epsilon; |
| 66 | _OptionalNamespacing_ | &rarr; | `id` _OptionalNamespacingTail_ |
| 67 | _OptionalNamespacingTail_ | &rarr; | `::` `id` |
| 68 | _OptionalNamespacingTail_ | &rarr; | &epsilon; |
| 69 | _RelationalExpression_ | &rarr; | _ArithmeticExpression_ _RelationalOperator_ _ArithmeticExpression_ |
| 70 | _RelationalOperator_ | &rarr; | `<>` |
| 71 | _RelationalOperator_ | &rarr; | `<` |
| 72 | _RelationalOperator_ | &rarr; | `<=` |
| 73 | _RelationalOperator_ | &rarr; | `==` |
| 74 | _RelationalOperator_ | &rarr; | `>` |
| 75 | _RelationalOperator_ | &rarr; | `>=` |
| 76 | _Statement_ | &rarr; | _AssignmentStatement_ `;` |
| 77 | _Statement_ | &rarr; | _StatementWithoutAssignment_ |
| 78 | _StatementBlock_ | &rarr; | _Statement_ |
| 79 | _StatementBlock_ | &rarr; | &epsilon; |
| 80 | _StatementBlock_ | &rarr; | `{` _StatementRecursion_ `}` |
| 81 | _StatementRecursion_ | &rarr; | _Statement_ _StatementRecursion_ |
| 82 | _StatementRecursion_ | &rarr; | &epsilon; |
| 83 | _StatementWithoutAssignment_ | &rarr; | `for` `(` _Type_ `id` `=` _Expression_ `;` _RelationalExpression_ `;` _AssignmentStatement_ `)` _StatementBlock_ `;` |
| 84 | _StatementWithoutAssignment_ | &rarr; | `get` `(` _Variable_ `)` `;` |
| 85 | _StatementWithoutAssignment_ | &rarr; | `if` `(` _Expression_ `)` `then` _StatementBlock_ `else` _StatementBlock_ `;` |
| 86 | _StatementWithoutAssignment_ | &rarr; | `put` `(` _Expression_ `)` `;` |
| 87 | _StatementWithoutAssignment_ | &rarr; | `return` `(` _Expression_ `)` `;` |
| 88 | _Term_ | &rarr; | _Factor_ _TermRecursion_ |
| 89 | _TermRecursion_ | &rarr; | _MultiplicativeOperator_ _Factor_ _TermRecursion_ |
| 90 | _TermRecursion_ | &rarr; | &epsilon; |
| 91 | _Type_ | &rarr; | _NumberType_ |
| 92 | _Type_ | &rarr; | `id` SemanticActionTypeId |
| 93 | _Variable_ | &rarr; | `id` _VariableTail_ |
| 94 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | _NumberType_ `id` _ArraySizeRecursion_ `;` _VariableDeclarationRecursionThenStatementRecursionA_ |
| 95 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | &epsilon; |
| 96 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | _StatementWithoutAssignment_ _StatementRecursion_ |
| 97 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | `id` _VariableDeclarationRecursionThenStatementRecursionB_ |
| 98 | _VariableDeclarationRecursionThenStatementRecursionB_ | &rarr; | _VariableTail_ `=` _Expression_ `;` _StatementRecursion_ |
| 99 | _VariableDeclarationRecursionThenStatementRecursionB_ | &rarr; | `id` _ArraySizeRecursion_ `;` _VariableDeclarationRecursionThenStatementRecursionA_ |
| 100 | _VariableTail_ | &rarr; | `(` _FunctionArguments_ `)` `.` `id` _VariableTail_ |
| 101 | _VariableTail_ | &rarr; | _IndexingRecursion_ _VariableTailTail_ |
| 102 | _VariableTailTail_ | &rarr; | `.` `id` _VariableTail_ |
| 103 | _VariableTailTail_ | &rarr; | &epsilon; |
| 104 | _VariableThenFunctionDeclarationRecursion_ | &rarr; | _Type_ `id` _VariableThenFunctionDeclarationRecursionTail_ |
| 105 | _VariableThenFunctionDeclarationRecursion_ | &rarr; | &epsilon; |
| 106 | _VariableThenFunctionDeclarationRecursionTail_ | &rarr; | _ArraySizeRecursion_ `;` _VariableThenFunctionDeclarationRecursion_ |
| 107 | _VariableThenFunctionDeclarationRecursionTail_ | &rarr; | _FunctionDeclarationRecursionTail_ |
| 108 | POP ERROR | POP ERROR | POP ERROR |
| 109 | SCAN ERROR | SCAN ERROR | SCAN ERROR |

The following semantic actions have been added to the CFG to form an attributed grammar:
| Semantic Action Name | Semantic Action Function |
|:--------:|:-----------:|
| _SemanticActionTypeId_ | `makeNode(id)` |





## AST Generation

The method for generating an AST was modified slightly...

1. Undergo parsing until a semantic action is discovered as a PARSE symbol.
2. Pop the semantic action off the parsing stack
3. Execute the function related to that semantic action
  - `makeNode` will:
      - define the AST node type
      - add the token to the node
      - create that node in the graph
      - store the node (or a reference to it) by pushing on the semantic stack
  - `makeFamily` will
      - define the AST node type
      - add the token to the node (if applicable)
      - create that node in the graph
      - pop X number of children nodes or their reference off the stack
      - Add edges from the new parent node to the children nodes in the graph
      - store the parent node (or a reference to it) by pushing on the semantic stack


We're using the `petgraph` library, so it's unsure if whether we'll have:
- Reference to individual nodes that we can keep on the stack as the graph grows
- One global graph object which is mutably passed to semantic actions and we push node indices on the stack







 --------------

Every row in the predict table should have its own semantic action/rule. Every nonterminal symbol in the production can have 0 or more semantic values.

When he says reduction, he means: you have the entire of the RHS of a production on your stack, I think you can pop these off and then execute your semantic action

Every terminal should have a makeNode and makeFamily associated with it in the semantic action. Every non-terminal should have a node-pointer attribute on ti to keep track of the node and pass it up the tree

Vector of closures to apply all at once at a node? Or actually just generate AST on the first pass, then go through it again for things like type checking?

S(ynthesized) attributes: values of children are being composed or given to the parent

Inherited attribute: value of children dervied from parent or sibling

An S-attributed SDT only contains synthesized attributes. All semantic actions are at the very right side of a production, even after he terminals/nonterminals (postfix sdt). And the attributes are evaluated during bottom-up parsing

An L-attributed SDT uses both inherited and synthesized attributes, but every inheritance must be done from the left side. Each inherited attribute is restricted to values from either the parent or the left siblings only. Semantic actions can be placed at any single point within the RHS production. Lastly, instead of waiting for reductions, we can apply them right away, during depth-first traversal.


