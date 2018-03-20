# `crs` Syntatic Analysis

## Context-Free Language Grammar Analysis

### EBNF Grammar

The specification for the syntactic analysis of the language is shown with the productions below in EBNF syntax.

| LHS | &rarr; | RHS |
|:---:|:----:|:----:|
| _prog_ | &rarr; | {_classDecl_} {_funcDef_} `program` _funcBody_ `;`|
| _classDecl_ | &rarr; | `class` `id` [`:` `id` {`,` `id`}] `{` {_varDecl_} {_funcDecl_} `}` `;` |
| _funcDecl_ | &rarr; | _type_ `id` `(` _fParams_ `)` `;` |
| _funcHead_ | &rarr; | _type_ [ `id` `::` ] `id` `(` _fParams_ `)` |
| _funcDef_ | &rarr; | _funcHead_ _funcBody_ `;` |
| _funcBody_ | &rarr; | `{` {_varDecl_} {_statement_} `}` |
| _varDecl_ | &rarr; | _type_ `id` {_arraySize_} `;` |
| _statement_ | &rarr; | _assignStat_ \| <br/> `if` `(` _expr_ `)` `then` _statBlock_ `;` \| <br/> `for` `(` _type_ `id` _assignOp_ _expr_ `;` _relExpr_ `;` _assignStat_  `)` _statBlock_ `;` \| <br/> `get` `(` _variable_ `)` `;` \| <br/> `put` `(` _expr_ `)` `;` \| <br/> `return` `(` _expr_ `)` `;` |
| _assignStat_ | &rarr; | _variable_ _assignOp_ _expr_ |
| _statBlock_ | &rarr; | `{` {_statement_} `}` \| _statement_ \| &epsilon; |
| _expr_ | &rarr; | _arithExpr_ \| _relExpr_ |
| _relExpr_ | &rarr; | _arithExpr_ _relOp_ _arithExpr_ |
| _arithExpr_ | &rarr; | _arithExpr_ _addOp_ _term_ \| _term_ |
| _sign_ | &rarr; | `+` \| `-` |
| _term_ | &rarr; | _term_ _multOp_ _factor_ \| _factor_ |
| _factor_ | &rarr; | _variable_ \| <br/> _functionCall_ \| <br/> `intNum` \| <br/> `floatNum` \| <br/> `(` _arithExpr_ `)` \| <br/> (`not` \| `!`) _factor_ \| <br/> _sign_ _factor_ |
| _variable_ | &rarr; | {_idnest_} `id` {_indice_} |
| _functionCall_ | &rarr; | {_idnest_} `id` `(` _aParams_ `)` |
| _idnest_ | &rarr; | `id` {_indice_} `.` \| `id` `(` _aParams_ `)` `.` |
| _indice_ | &rarr; | `[` _arithExpr_ `]` |
| _arraySize_ | &rarr; | `[` `intNum` `]` |
| _type_ | &rarr; | `int` \| `float` \| `id` |
| _fParams_ | &rarr; | _type_ `id` {_arraySize_} {_fParamsTail_} \| &epsilon; |
| _aParams_ | &rarr; | _expr_ {_aParamsTail_} \| &epsilon; |
| _fParamsTail_ | &rarr; | `,` _type_ `id` {_arraySize_} |
| _aParamsTail_ | &rarr; | `,` _expr_ |
| _assignOp_ | &rarr; | `=` |
| _relOp_ | &rarr; | `==` \| `<>` \| `<` \| `>` \| `<=` \| `>=` |
| _addOp_ | &rarr; | `+` \| `-` \| `or` \| `¦¦` |
| _multOp_ | &rarr; | `*` \| `/` \| `and` \| `&&` |

### BNF Grammar

An EBNF grammar can be converted to a BNF grammar by applying the following rules:

- For every instance of {_X_}, extract it to a new nonterminal _Y_ and add the production _Y_ &rarr; _Y_ _X_ | &epsilon;.
- For every instance of [_X_], extract it to a new nonterminal _Y_ and add the production _Y_ &rarr; _X_ | &epsilon;.
- For every instance of (_X_), extract it to a new nonterminal _Y_ and add the production _Y_ &rarr; _X_.

The syntactic language specification in EBNF format was converted to a BNF grammar shown below.

| LHS | &rarr; | RHS |
|:---:|:----:|:----:|
| _prog_ | &rarr; | _classDeclRecursion_ _funcDefRecursion_ `program` _funcBody_ `;`|
| _classDeclRecursion_ | &rarr; | _classDeclRecursion_ _classDecl_ \| &epsilon; |
| _classDecl_ | &rarr; | `class` `id` _optionalInheritance_ `{` _varDeclRecursion_ _funcDeclRecursion_ `}` `;` |
| _optionalInheritance_ | &rarr; | `:` `id` _MultipleSuperClasses_ \| &epsilon;|
| _MultipleSuperClasses_ | &rarr; | _MultipleSuperClasses_ `,` `id` \| &epsilon; |
| _funcDeclRecursion_ | &rarr; | _funcDeclRecursion_ _funcDecl_ \| &epsilon; |
| _funcDecl_ | &rarr; | _type_ `id` `(` _fParams_ `)` `;` |
| _funcHead_ | &rarr; | _type_ _optionalNamespace_ `id` `(` _fParams_ `)` |
| _optionalNamespace_ | &rarr; |  `id` `::` \| &epsilon;|
| _funcDefRecursion_ | &rarr; | _funcDefRecursion_ _funcDef_ \| &epsilon; |
| _funcDef_ | &rarr; | _funcHead_ _funcBody_ `;` |
| _funcBody_ | &rarr; | `{` _varDeclRecursion_ _statementRecursion_ `}` |
| _varDeclRecursion_ | &rarr; | _varDeclRecursion_ _varDecl_ \| &epsilon; |
| _varDecl_ | &rarr; | _type_ `id` _arraySizeRecursion_ `;` |
| _statementRecursion_ | &rarr; | _statementRecursion_ _statement_ \| &epsilon; |
| _statement_ | &rarr; | _assignStat_ \| <br/> `if` `(` _expr_ `)` `then` _statBlock_ `;` \| <br/> `for` `(` _type_ `id` _assignOp_ _expr_ `;` _relExpr_ `;` _assignStat_  `)` _statBlock_ `;` \| <br/> `get` `(` _variable_ `)` `;` \| <br/> `put` `(` _expr_ `)` `;` \| <br/> `return` `(` _expr_ `)` `;` |
| _assignStat_ | &rarr; | _variable_ _assignOp_ _expr_ |
| _statBlock_ | &rarr; | `{` _statementRecursion_ `}` \| _statement_ \| &epsilon; |
| _expr_ | &rarr; | _arithExpr_ \| _relExpr_ |
| _relExpr_ | &rarr; | _arithExpr_ _relOp_ _arithExpr_ |
| _arithExpr_ | &rarr; | _arithExpr_ _addOp_ _term_ \| _term_ |
| _sign_ | &rarr; | `+` \| `-` |
| _term_ | &rarr; | _term_ _multOp_ _factor_ \| _factor_ |
| _factor_ | &rarr; | _variable_ \| <br/> _functionCall_ \| <br/> `intNum` \| <br/> `floatNum` \| <br/> `(` _arithExpr_ `)` \| <br/> _negationOperator_ _factor_ \| <br/> _sign_ _factor_ |
| _negationOperator_ | &rarr; | `not` \| `!` |
| _variable_ | &rarr; | _idnestRecursion_ `id` _indiceRecursion_ |
| _functionCall_ | &rarr; | _idnestRecursion_ `id` `(` _aParams_ `)` |
| _idnestRecursion_ | &rarr; | _idnestRecursion_ _idnest_ \| &epsilon; |
| _idnest_ | &rarr; | `id` _indiceRecursion_ `.` \| `id` `(` _aParams_ `)` `.` |
| _indiceRecursion_ | &rarr; | _indiceRecursion_ _indice_ \| &epsilon; |
| _indice_ | &rarr; | `[` _arithExpr_ `]` |
| _arraySizeRecursion_ | &rarr; | _arraySizeRecursion_ _arraySize_ \| &epsilon; |
| _arraySize_ | &rarr; | `[` `intNum` `]` |
| _type_ | &rarr; | `int` \| `float` \| `id` |
| _fParams_ | &rarr; | _type_ `id` _arraySizeRecursion_ _fParamsTailRecursion_ \| &epsilon; |
| _aParams_ | &rarr; | _expr_ _aParamsTailRecursion_ \| &epsilon; |
| _fParamsTailRecursion_ | &rarr; | _fParamsTailRecursion_ _fParamsTail_ \| &epsilon; |
| _fParamsTail_ | &rarr; | `,` _type_ `id` _arraySizeRecursion_ |
| _aParamsTailRecursion_ | &rarr; | _aParamsTailRecursion_ _aParamsTail_ \| &epsilon; |
| _aParamsTail_ | &rarr; | `,` _expr_ |
| _assignOp_ | &rarr; | `=` |
| _relOp_ | &rarr; | `==` \| `<>` \| `<` \| `>` \| `<=` \| `>=` |
| _addOp_ | &rarr; | `+` \| `-` \| `or` \| `¦¦` |
| _multOp_ | &rarr; | `*` \| `/` \| `and` \| `&&` |

An [AtoCC](http://atocc.de)-compatible text format of the above grammar (with renamed variables) is shown below:

```
prog -> classDeclRecursion funcDefRecursion 'program' funcBody ';'
classDeclRecursion -> classDeclRecursion classDecl | EPSILON
classDecl -> 'class' 'id' optionalInheritance '{' varDeclRecursion funcDeclRecursion '}' ';'
optionalInheritance -> ':' 'id' multipleSuperClasses | EPSILON
multipleSuperClasses -> multipleSuperClasses ',' 'id' | EPSILON
funcDeclRecursion -> funcDeclRecursion funcDecl | EPSILON
funcDecl -> type 'id' '(' fParams ')' ';'
funcHead -> type optionalNamespace 'id' '(' fParams ')'
optionalNamespace ->  'id' '::' | EPSILON
funcDefRecursion -> funcDefRecursion funcDef | EPSILON
funcDef -> funcHead funcBody ';'
funcBody -> '{' varDeclRecursion statementRecursion '}'
varDeclRecursion -> varDeclRecursion varDecl | EPSILON
varDecl -> type 'id' arraySizeRecursion ';'
statementRecursion -> statementRecursion statement | EPSILON
statement -> assignStat | 'if' '(' expr ')' 'then' statBlock ';' | 'for' '(' type 'id' assignOp expr ';' relExpr ';' assignStat  ')' statBlock ';' | 'get' '(' variable ')' ';' | 'put' '(' expr ')' ';' | 'return' '(' expr ')' ';'
assignStat -> variable assignOp expr
statBlock -> '{' statementRecursion '}' | statement | EPSILON
expr -> arithExpr | relExpr
relExpr -> arithExpr relOp arithExpr
arithExpr -> arithExpr addOp term | term
sign -> '+' | '-'
term -> term multOp factor | factor
factor -> variable | functionCall | 'intNum' | 'floatNum' | '(' arithExpr ')' | negationOperator factor | sign factor
negationOperator -> 'not' | '!'
variable -> idnestRecursion 'id' indiceRecursion
functionCall -> idnestRecursion 'id' '(' aParams ')'
idnestRecursion -> idnestRecursion idnest | EPSILON
idnest -> 'id' indiceRecursion '.' | 'id' '(' aParams ')' '.'
indiceRecursion -> indiceRecursion indice | EPSILON
indice -> '[' arithExpr ']'
arraySizeRecursion -> arraySizeRecursion arraySize | EPSILON
arraySize -> '[' 'intNum' ']'
type -> 'int' | 'float' | 'id'
fParams -> type 'id' arraySizeRecursion fParamsTailRecursion | EPSILON
aParams -> expr aParamsTailRecursion | EPSILON
fParamsTailRecursion -> fParamsTailRecursion fParamsTail | EPSILON
fParamsTail -> ',' type 'id' arraySizeRecursion
aParamsTailRecursion -> aParamsTailRecursion aParamsTail | EPSILON
aParamsTail -> ',' expr
assignOp -> '='
relOp -> '==' | '<>' | '<' | '>' | '<=' | '>='
addOp -> '+' | '-' | 'or' | '¦¦'
multOp -> '*' | '/' | 'and' | '&&'
```

### Left-Factored, Right-Recursive and LL(1) Grammar

Left factoring is a technique used in predictive top-down parsers avoid the need for backtracking or lookaheads during parsing, such as is done in recursive descent. It involves the removal of any common left factor (terminal or non-terminal) that appears in a production with an or clause (|), or effectively two productions of the same non-terminal. Performing left factoring means that at a given non-terminal, there is a clear deterministic choice of which production to proceed towards.

Left recursion is avoided in recursive descent and predictive parsing strategies due to the possibility of an infinite loop, resulting in the compiler never terminating and with no progress.

Left-recursive productions can be replaced with right-recursive productions by applying the following rules:

For any production with the general form...

> _S_ &rarr; _S_&alpha;<sub>1</sub> | ... | _S_&alpha;<sub>n</sub> | &beta;<sub>1</sub> | ... | &beta;<sub>m</sub>

... replace with two productions:
> _S_ &rarr; &beta;<sub>1</sub> _T_ | ... | &beta;<sub>m</sub> _T_ <br/>
> _T_ &rarr; &alpha;<sub>1</sub> _T_ | ... | &alpha;<sub>1</sub> _T_ | &epsilon;

This technique needs to be applied and the condition must hold for all non-terminal substitutions (one derivation step of a production).

Ambiguity in context-free grammars means that it can result in multiple possible derivations or parse trees. This is undesirable as we want our compiler to reliably generate the same parse tree given the same input program.

Using an LL(1) grammar for a syntactic analysis is attractive given that LL(1) grammars are known to be unambigiuous. Converting a grammar to LL(1) is an effective technique for dealing with ambiguities in general, giving that determining if an arbitrary grammar is ambiguous is an undecidable problem.

An attempt to use the following [left-factoring online tool](https://cyberzhg.github.io/toolbox/left_fact), [left-recusion elimination online tool](https://cyberzhg.github.io/toolbox/left_rec) and [CFG-to-LL(k) online tool](https://cyberzhg.github.io/toolbox/cfg2ll) was done. However, these tools were error-prone, and superior results were obtained by manipulating the grammar by hand while verifying with the [AtoCC](http://atocc.de) kfGEdit tool along the way.

Ultimately, too many changes were made to the grammar to describe every ambiguity, left factoring or left-recursion elimination. The resulting grammar is shown below in an AtoCC-compatible format (with renamed non-terminal symbols to more accurately represent their role).

```
Program                                             -> ClassDeclarationRecursion FunctionDefinitionRecursion program FunctionBody ;
AdditiveOperator                                    -> +
AdditiveOperator                                    -> -
AdditiveOperator                                    -> or
AdditiveOperator                                    -> ¦¦
ArithmeticExpression                                -> Term ArithmeticExpressionTail
ArithmeticExpressionTail                            -> AdditiveOperator Term ArithmeticExpressionTail
ArithmeticExpressionTail                            -> EPSILON
ArithmeticOrRelationalExpression                    -> RelationalOperator ArithmeticExpression
ArithmeticOrRelationalExpression                    -> EPSILON
ArraySize                                           -> [ intNum ]
ArraySizeRecursion                                  -> ArraySize ArraySizeRecursion
ArraySizeRecursion                                  -> EPSILON
AssignmentStatement                                 -> Variable = Expression
ClassDeclaration                                    -> class id OptionalInheritanceList { VariableThenFunctionDeclarationRecursion } ;
ClassDeclarationRecursion                           -> ClassDeclaration ClassDeclarationRecursion
ClassDeclarationRecursion                           -> EPSILON
Expression                                          -> ArithmeticExpression ArithmeticOrRelationalExpression
Factor                                              -> ( ArithmeticExpression )
Factor                                              -> FunctionCallOrVariable
Factor                                              -> NegationOperator Factor
Factor                                              -> NumberSign Factor
Factor                                              -> floatNum
Factor                                              -> intNum
FunctionArguments                                   -> Expression FunctionArgumentsTailRecursion
FunctionArguments                                   -> EPSILON
FunctionArgumentsTail                               -> , Expression
FunctionArgumentsTailRecursion                      -> FunctionArgumentsTail FunctionArgumentsTailRecursion
FunctionArgumentsTailRecursion                      -> EPSILON
FunctionBody                                        -> { VariableDeclarationRecursionThenStatementRecursionA }
FunctionCallOrVariable                              -> id FunctionCallOrVariableTail
FunctionCallOrVariableTail                          -> FunctionCallParensOrIndexing FunctionCallOrVariableTailRecursion
FunctionCallOrVariableTailRecursion                 -> . id FunctionCallOrVariableTail
FunctionCallOrVariableTailRecursion                 -> EPSILON
FunctionCallParensOrIndexing                        -> ( FunctionArguments )
FunctionCallParensOrIndexing                        -> IndexingRecursion
FunctionDeclarationRecursionStart                   -> Type id FunctionDeclarationRecursionTail
FunctionDeclarationRecursionStart                   -> EPSILON
FunctionDeclarationRecursionTail                    -> ( FunctionParameters ) ; FunctionDeclarationRecursionStart
FunctionDefinition                                  -> FunctionHeader FunctionBody ;
FunctionDefinitionRecursion                         -> FunctionDefinition FunctionDefinitionRecursion
FunctionDefinitionRecursion                         -> EPSILON
FunctionHeader                                      -> Type OptionalNamespacing ( FunctionParameters )
FunctionParameters                                  -> Type id ArraySizeRecursion FunctionParametersTailRecursion
FunctionParameters                                  -> EPSILON
FunctionParametersTail                              -> , Type id ArraySizeRecursion
FunctionParametersTailRecursion                     -> FunctionParametersTail FunctionParametersTailRecursion
FunctionParametersTailRecursion                     -> EPSILON
IdListRecursion                                     -> , id IdListRecursion
IdListRecursion                                     -> EPSILON
Indexing                                            -> [ ArithmeticExpression ]
IndexingRecursion                                   -> Indexing IndexingRecursion
IndexingRecursion                                   -> EPSILON
MultiplicativeOperator                              -> &&
MultiplicativeOperator                              -> *
MultiplicativeOperator                              -> /
MultiplicativeOperator                              -> and
NegationOperator                                    -> !
NegationOperator                                    -> not
NumberSign                                          -> +
NumberSign                                          -> -
NumberType                                          -> float
NumberType                                          -> int
OptionalInheritanceList                             -> : id IdListRecursion
OptionalInheritanceList                             -> EPSILON
OptionalNamespacing                                 -> id OptionalNamespacingTail
OptionalNamespacingTail                             -> :: id
OptionalNamespacingTail                             -> EPSILON
RelationalExpression                                -> ArithmeticExpression RelationalOperator ArithmeticExpression
RelationalOperator                                  -> !=
RelationalOperator                                  -> <
RelationalOperator                                  -> <=
RelationalOperator                                  -> ==
RelationalOperator                                  -> >
RelationalOperator                                  -> >=
Statement                                           -> AssignmentStatement ;
Statement                                           -> StatementWithoutAssignment
StatementBlock                                      -> Statement
StatementBlock                                      -> EPSILON
StatementBlock                                      -> { StatementRecursion }
StatementRecursion                                  -> Statement StatementRecursion
StatementRecursion                                  -> EPSILON
StatementWithoutAssignment                          -> for ( Type id = Expression ; RelationalExpression ; AssignmentStatement ) StatementBlock ;
StatementWithoutAssignment                          -> get ( Variable ) ;
StatementWithoutAssignment                          -> if ( Expression ) then StatementBlock else StatementBlock ;
StatementWithoutAssignment                          -> put ( Expression ) ;
StatementWithoutAssignment                          -> return ( Expression ) ;
Term                                                -> Factor TermRecursion
TermRecursion                                       -> MultiplicativeOperator Factor TermRecursion
TermRecursion                                       -> EPSILON
Type                                                -> NumberType
Type                                                -> id
Variable                                            -> id VariableTail
VariableDeclarationRecursionThenStatementRecursionA -> NumberType id ArraySizeRecursion ; VariableDeclarationRecursionThenStatementRecursionA
VariableDeclarationRecursionThenStatementRecursionA -> EPSILON
VariableDeclarationRecursionThenStatementRecursionA -> StatementWithoutAssignment StatementRecursion
VariableDeclarationRecursionThenStatementRecursionA -> id VariableDeclarationRecursionThenStatementRecursionB
VariableDeclarationRecursionThenStatementRecursionB -> VariableTail = Expression ; StatementRecursion
VariableDeclarationRecursionThenStatementRecursionB -> id ArraySizeRecursion ; VariableDeclarationRecursionThenStatementRecursionA
VariableTail                                        -> ( FunctionArguments ) . id VariableTail
VariableTail                                        -> IndexingRecursion VariableTailTail
VariableTailTail                                    -> . id VariableTail
VariableTailTail                                    -> EPSILON
VariableThenFunctionDeclarationRecursion            -> Type id VariableThenFunctionDeclarationRecursionTail
VariableThenFunctionDeclarationRecursion            -> EPSILON
VariableThenFunctionDeclarationRecursionTail        -> ArraySizeRecursion ; VariableThenFunctionDeclarationRecursion
VariableThenFunctionDeclarationRecursionTail        -> FunctionDeclarationRecursionTail
```

The [AtoCC](http://atocc.de) kfG Edit tool confirms that the above grammar is LL(1).

![AtoCC confirmation prompt indicating that the gramar satifies both LL(1) properties.](assets/images/atocc-ll1-confirmation.png)

## LL(1) Parse Table

A parsing table must be constructed from the above grammar in order to represent the grammar during predictive parsing. The information below was generated automatically using [an online tool](http://hackingoff.com/compilers/predict-first-follow-set).

The fact that there was at most one production in each table cell further reinforces the fact that the grammar is LL(1).

### _FIRST_ Sets
| Non-Terminal Symbol | First Set |
|:------:|:-----:|
| `program` | `program` |
| `;` | `;` |
| `+` | `+` |
| `-` | `-` |
| `or` | `or` |
| `¦¦` | `¦¦` |
| &epsilon; | &epsilon; |
| `[` | `[` |
| `intNum` | `intNum` |
| `]` | `]` |
| `=` | `=` |
| `class` | `class` |
| `id` | `id` |
| `{` | `}` |
| `}` | `}` |
| `(` | `)` |
| `)` | `)` |
| `floatNum` | `floatNum` |
| `,` | `,` |
| `.` | `.` |
| `&&` | `&&` |
| `*` | `*` |
| `/` | `/` |
| `and` | `and` |
| `!` | `!` |
| `not` | `not` |
| `float` | `float` |
| `int` | `int` |
| `:` | `:` |
| `::` | `::` |
| `!=` | `!=` |
| `<` | `<` |
| `<=` | `<=` |
| `==` | `==` |
| `>` | `>` |
| `>=` | `>=` |
| `for` | `for` |
| `get` | `get` |
| `if` | `if` |
| `then` | `then` |
| `else` | `else` |
| `put` | `put` |
| `return` | `return` |
| _Program_ | `program`, &epsilon;, `class`, `id`, `float`, `int` |
| _AdditiveOperator_ | `+`, `-`, `or`, `¦¦` |
| _ArithmeticExpressionTail_ | &epsilon;, `+`, `-`, `or`, `¦¦` |
| _ArithmeticOrRelationalExpression_ | &epsilon;, `!=`, `<`, `<=`, `==`, `>`, `>=` |
| _ArraySize_ | `[` |
| _ArraySizeRecursion_ | &epsilon;, `[` |
| _ClassDeclaration_ | `class` |
| _ClassDeclarationRecursion_ | &epsilon;, `class` |
| _Factor_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _FunctionArguments_ | &epsilon;, `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _FunctionArgumentsTail_ | `,` |
| _FunctionArgumentsTailRecursion_ | &epsilon;, `,` |
| _FunctionBody_ | `{` |
| _FunctionCallOrVariable_ | `id` |
| _FunctionCallOrVariableTailRecursion_ | `.`, &epsilon; |
| _FunctionCallParensOrIndexing_ | `(`, &epsilon;, `[` |
| _FunctionDeclarationRecursionStart_ | &epsilon;, `id`, `float`, `int` |
| _FunctionDeclarationRecursionTail_ | `(` |
| _FunctionDefinitionRecursion_ | &epsilon;, `id`, `float`, `int` |
| _FunctionParameters_ | &epsilon;, `id`, `float`, `int` |
| _FunctionParametersTail_ | `,` |
| _FunctionParametersTailRecursion_ | &epsilon;, `,` |
| _IdListRecursion_ | `,`, &epsilon; |
| _Indexing_ | `[` |
| _IndexingRecursion_ | &epsilon;, `[` |
| _MultiplicativeOperator_ | `&&`, `*`, `/`, `and` |
| _NegationOperator_ | `!`, `not` |
| _NumberSign_ | `+`, `-` |
| _NumberType_ | `float`, `int` |
| _OptionalInheritanceList_ | `:`, &epsilon; |
| _OptionalNamespacing_ | `id` |
| _OptionalNamespacingTail_ | `::`, &epsilon; |
| _RelationalOperator_ | `!=`, `<`, `<=`, `==`, `>`, `>=` |
| _StatementBlock_ | &epsilon;, `{`, `id`, `for`, `get`, `if`, `put`, `return` |
| _StatementRecursion_ | &epsilon;, `id`, `for`, `get`, `if`, `put`, `return` |
| _StatementWithoutAssignment_ | `for`, `get`, `if`, `put`, `return` |
| _TermRecursion_ | &epsilon;, `&&`, `*`, `/`, `and` |
| _Type_ | `id`, `float`, `int` |
| _Variable_ | `id` |
| _VariableDeclarationRecursionThenStatementRecursionA_ | &epsilon;, `id`, `float`, `int`, `for`, `get`, `if`, `put`, `return` |
| _VariableDeclarationRecursionThenStatementRecursionB_	| `=`, `id`, `(`, &epsilon;, `[`, `.` |
| _VariableTail_ | `(`, &epsilon;, `[`, `.` |
| _VariableTailTail_ | `.`, &epsilon; |
| _VariableThenFunctionDeclarationRecursion_ | &epsilon;, `id`, `float`, `int` |
| _VariableThenFunctionDeclarationRecursionTail_ | `;`, &epsilon;, `[`, `(` |
| _AssignmentStatement_ | `id` |
| _FunctionHeader_ | `id`, `float`, `int` |
| _Statement_ | `id`, `for`, `get`, `if`, `put`, `return` |
| _FunctionCallOrVariableTail_ | `(`, &epsilon;, `[`, `.` |
| _FunctionDefinition_ | `id`, `float`, `int` |
| _Term_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _ArithmeticExpression_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _Expression_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _RelationalExpression_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |

### _FOLLOW_ Sets

| Non-Terminal Symbol | Follow Set |
|:----:|:-----:|
| _Program_ | `$` |
| _AdditiveOperator_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _ArithmeticExpression_ | `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _ArithmeticExpressionTail_ | `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _ArithmeticOrRelationalExpression_ | `;`, `)`, `,` |
| _ArraySize_ | `[`, `;`, `,`, `)` |
| _ArraySizeRecursion_ `;`, `,`, `)` |
| _AssignmentStatement_ | `)`, `;` |
| _ClassDeclaration_ | `class`, `program`, `id`, `float`, `int` |
| _ClassDeclarationRecursion_ | `program`, `id`, `float`, `int` |
| _Expression_ | `;`, `)`, `,` |
| _Factor_ | `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _FunctionArguments_ | `)` |
| _FunctionArgumentsTail_ | `,`, `)` |
| _FunctionArgumentsTailRecursion_ | `)` |
| _FunctionBody_ | `;` |
| _FunctionCallOrVariable_ | `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _FunctionCallOrVariableTail_ | `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _FunctionCallOrVariableTailRecursion_ | `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _FunctionCallParensOrIndexing_ | `.`, `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _FunctionDeclarationRecursionStart_ | `}` |
| _FunctionDeclarationRecursionTail_ | `}` |
| _FunctionDefinition_ | `id`, `float`, `int`, `program` |
| _FunctionDefinitionRecursion_ | `program` |
| _FunctionHeader_ | `{` |
| _FunctionParameters_ | `)` |
| _FunctionParametersTail_ | `,`, `)` |
| _FunctionParametersTailRecursion_ | `)` |
| _IdListRecursion_ | `{` |
| _Indexing_ | `[`, `.`, `=`, `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `)`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `;`, `,` |
| _IndexingRecursion_ | `.`, `=`, `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `)`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `;`, `,` |
| _MultiplicativeOperator_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _NegationOperator_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _NumberSign_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _NumberType_ | `id` |
| _OptionalInheritanceList_ | `{` |
| _OptionalNamespacing_ | `(` |
| _OptionalNamespacingTail_ | `(` |
| _RelationalExpression_ | `;` |
| _RelationalOperator_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| _Statement_ | `id`, `for`, `get`, `if`, `put`, `return`, `}`, `;`, `else` |
| _StatementBlock_ | `;`, `else` |
| _StatementRecursion_ | `}` |
| _StatementWithoutAssignment_ | `id`, `for`, `get`, `if`, `put`, `return`, `}`, `;`, `else` |
| _Term_ | `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _TermRecursion_ | `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| _Type_ | `id` |
| _Variable_ | `)`, `=` |
| _VariableDeclarationRecursionThenStatementRecursionA_ | `}` |
| _VariableDeclarationRecursionThenStatementRecursionB_ | `}` |
| _VariableTail_ | `=`, `)` |
| _VariableTailTail_ | `=`, `)` |
| _VariableThenFunctionDeclarationRecursion_ | `}` |
| _VariableThenFunctionDeclarationRecursionTail_ | `}` |

### Prediction Table

| Prediction Number | Production LHS Non-Terminal | &rarr; | Production RHS Expression | Predict Set |
|:--------:|:-----------|:-----------:|:-----------|:-----------:|
| 1 | _Program_ | &rarr; | _ClassDeclarationRecursion_ _FunctionDefinitionRecursion_ `program` _FunctionBody_ `;` | `class`, `id`, `float`, `int`, `program` |
| 2 | _AdditiveOperator_ | &rarr; | `+` | `+` |
| 3 | _AdditiveOperator_ | &rarr; | `-` | `-` |
| 4 | _AdditiveOperator_ | &rarr; | `or` | `or` |
| 5 | _AdditiveOperator_ | &rarr; | `¦¦` | `¦¦` |
| 6 | _ArithmeticExpression_ | &rarr; | _Term_ _ArithmeticExpressionTail_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| 7 | _ArithmeticExpressionTail_ | &rarr; | _AdditiveOperator_ _Term_ _ArithmeticExpressionTail_ | `+`, `-`, `or`, `¦¦` |
| 8 | _ArithmeticExpressionTail_ | &rarr; | &epsilon; | `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| 9 | _ArithmeticOrRelationalExpression_ | &rarr; | _RelationalOperator_ _ArithmeticExpression_ | `!=`, `<`, `=`, `==`, `>`, `>=` |
| 10 | _ArithmeticOrRelationalExpression_ | &rarr; | &epsilon; | `;`, `)`, `,` |
| 11 | _ArraySize_ | &rarr; | `[` `intNum` `]` | `[` |
| 12 | _ArraySizeRecursion_ | &rarr; | _ArraySize_ _ArraySizeRecursion_ | `[` |
| 13 | _ArraySizeRecursion_ | &rarr; | &epsilon; | `;`, `,`, `)` |
| 14 | _AssignmentStatement_ | &rarr; | _Variable_ `=` _Expression_ | `id` |
| 15 | _ClassDeclaration_ | &rarr; | `class` `id` _OptionalInheritanceList_ `{` _VariableThenFunctionDeclarationRecursion_ `}` `;` | `class` |
| 16 | _ClassDeclarationRecursion_ | &rarr; | _ClassDeclaration_ _ClassDeclarationRecursion_ | `class` |
| 17 | _ClassDeclarationRecursion_ | &rarr; | &epsilon; | `program`, `id`, `float`, `int` |
| 18 | _Expression_ | &rarr; | _ArithmeticExpression_ _ArithmeticOrRelationalExpression_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| 19 | _Factor_ | &rarr; | `(` `ArithmeticExpression` `)` | `(` |
| 20 | _Factor_ | &rarr; | _FunctionCallOrVariable_ | `id` |
| 21 | _Factor_ | &rarr; | _NegationOperator_ _Factor_ | `!`, `not` |
| 22 | _Factor_ | &rarr; | _NumberSign_ _Factor_ | `+`, `-` |
| 23 | _Factor_ | &rarr; | `floatNum` | `floatNum` |
| 24 | _Factor_ | &rarr; | `intNum` | `intNum` |
| 25 | _FunctionArguments_ | &rarr; | _Expression_ _FunctionArgumentsTailRecursion_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| 26 | _FunctionArguments_ | &rarr; | &epsilon; | `)` |
| 27 | _FunctionArgumentsTail_ | &rarr; | `,` _Expression_ | `,` |
| 28 | _FunctionArgumentsTailRecursion_ | &rarr; | _FunctionArgumentsTail_ _FunctionArgumentsTailRecursion_ | `,` |
| 29 | _FunctionArgumentsTailRecursion_ | &rarr; | &epsilon; | `)` |
| 30 | _FunctionBody_ | &rarr; | `{` _VariableDeclarationRecursionThenStatementRecursionA_ `}` | `{` |
| 31 | _FunctionCallOrVariable_ | &rarr; | `id` _FunctionCallOrVariableTail_ | `id` |
| 32 | _FunctionCallOrVariableTail_ | &rarr; | _FunctionCallParensOrIndexing_ _FunctionCallOrVariableTailRecursion_ | `(`, WEID_EMPTINESS, `.` |
| 33 | _FunctionCallOrVariableTailRecursion_ | &rarr; | `.` `id` _FunctionCallOrVariableTail_ | `.` |
| 34 | _FunctionCallOrVariableTailRecursion_ | &rarr; | &epsilon; | `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| 35 | _FunctionCallParensOrIndexing_ | &rarr; | `(` _FunctionArguments_ `)` | `(` |
| 36 | _FunctionCallParensOrIndexing_ | &rarr; | _IndexingRecursion_ | `[`|
| 37 | _FunctionDeclarationRecursionStart_ | &rarr; | _Type_ `id` _FunctionDeclarationRecursionTail_ | `id`, `float`, `int` |
| 38 | _FunctionDeclarationRecursionStart_ | &rarr; | &epsilon; | `}` |
| 39 | _FunctionDeclarationRecursionTail_ | &rarr; | `(` _FunctionParameters_ `)` `;` _FunctionDeclarationRecursionStart_ | `(` |
| 40 | _FunctionDefinition_ | &rarr; | _FunctionHeader_ _FunctionBody_ `;` | `id`, `float`, `int` |
| 41 | _FunctionDefinitionRecursion_ | &rarr; | _FunctionDefinition_ _FunctionDefinitionRecursion_ | `id`, `float`, `int` |
| 42 | _FunctionDefinitionRecursion_ | &rarr; | &epsilon; | `program` |
| 43 | _FunctionHeader_ | &rarr; | _Type_ _OptionalNamespacing_ `(` _FunctionParameters_ `)` | `id`, `float`, `int` |
| 44 | _FunctionParameters_ | &rarr; | _Type_ `id` _ArraySizeRecursion_ _FunctionParametersTailRecursion_ | `id`, `float`, `int` |
| 45 | _FunctionParameters_ | &rarr; | &epsilon; | `)` |
| 46 | _FunctionParametersTail_ | &rarr; | `,` _Type_ `id` _ArraySizeRecursion_ | `,` |
| 47 | _FunctionParametersTailRecursion_ | &rarr; | _FunctionParametersTail_ _FunctionParametersTailRecursion_ | `,` |
| 48 | _FunctionParametersTailRecursion_ | &rarr; | &epsilon; | `)` |
| 49 | _IdListRecursion_ | &rarr; | `,` `id` _IdListRecursion_ | `,` |
| 50 | _IdListRecursion_ | &rarr; | &epsilon; | `{` |
| 51 | _Indexing_ | &rarr; | `[` _ArithmeticExpression_ `]` | `[` |
| 52 | _IndexingRecursion_ | &rarr; | _Indexing_ _IndexingRecursion_ | `[` |
| 53 | _IndexingRecursion_ | &rarr; | &epsilon; | `.`, `=`, `&&`, `*`, `/`, `and`, `+`, `-`, `or`, `¦¦`, `)`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `;`, `,` |
| 54 | _MultiplicativeOperator_ | &rarr; | `&&` | `&&` |
| 55 | _MultiplicativeOperator_ | &rarr; | `*` | `*` |
| 56 | _MultiplicativeOperator_ | &rarr; | `/` | `/` |
| 57 | _MultiplicativeOperator_ | &rarr; | `and` | `and` |
| 58 | _NegationOperator_ | &rarr; | `!` | `!` |
| 59 | _NegationOperator_ | &rarr; | `not` | `not` |
| 60 | _NumberSign_ | &rarr; | `+` | `+` |
| 61 | _NumberSign_ | &rarr; | `-` | `-` |
| 62 | _NumberType_ | &rarr; | `float` | `float` |
| 63 | _NumberType_ | &rarr; | `int` | `int` |
| 64 | _OptionalInheritanceList_ | &rarr; | `:` `id` _IdListRecursion_ | `:` |
| 65 | _OptionalInheritanceList_ | &rarr; | &epsilon; | `{` |
| 66 | _OptionalNamespacing_ | &rarr; | `id` _OptionalNamespacingTail_ | `id` |
| 67 | _OptionalNamespacingTail_ | &rarr; | `::` `id` | `::` |
| 68 | _OptionalNamespacingTail_ | &rarr; | &epsilon; | `(` |
| 69 | _RelationalExpression_ | &rarr; | _ArithmeticExpression_ _RelationalOperator_ _ArithmeticExpression_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| 70 | _RelationalOperator_ | &rarr; | `!=` | `!=` |
| 71 | _RelationalOperator_ | &rarr; | `<` | `<` |
| 72 | _RelationalOperator_ | &rarr; | `<=` | `<=` |
| 73 | _RelationalOperator_ | &rarr; | `==` | `==` |
| 74 | _RelationalOperator_ | &rarr; | `>` | `>` |
| 75 | _RelationalOperator_ | &rarr; | `>=` | `>=` |
| 76 | _Statement_ | &rarr; | _AssignmentStatement_ `;` | `id` |
| 77 | _Statement_ | &rarr; | _StatementWithoutAssignment_ | `for`, `get`, `if`, `put`, `return` |
| 78 | _StatementBlock_ | &rarr; | _Statement_ | `id`, `for`, `get`, `if`, `put`, `return` |
| 79 | _StatementBlock_ | &rarr; | &epsilon; | `;`, `else` |
| 80 | _StatementBlock_ | &rarr; | `{` _StatementRecursion_ `}` | `{` |
| 81 | _StatementRecursion_ | &rarr; | _Statement_ _StatementRecursion_ | `id`, `for`, `get`, `if`, `put`, `return` |
| 82 | _StatementRecursion_ | &rarr; | &epsilon; | `}` |
| 83 | _StatementWithoutAssignment_ | &rarr; | `for` `(` _Type_ `id` `=` _Expression_ `;` _RelationalExpression_ `;` _AssignmentStatement_ `)` _StatementBlock_ `;` | `for` |
| 84 | _StatementWithoutAssignment_ | &rarr; | `get` `(` _Variable_ `)` `;` | `get` |
| 85 | _StatementWithoutAssignment_ | &rarr; | `if` `(` _Expression_ `)` `then` _StatementBlock_ `else` _StatementBlock_ `;` | `if` |
| 86 | _StatementWithoutAssignment_ | &rarr; | `put` `(` _Expression_ `)` `;` | `put` |
| 87 | _StatementWithoutAssignment_ | &rarr; | `return` `(` _Expression_ `)` `;` | `return` |
| 88 | _Term_ | &rarr; | _Factor_ _TermRecursion_ | `(`, `floatNum`, `intNum`, `id`, `+`, `-`, `!`, `not` |
| 89 | _TermRecursion_ | &rarr; | _MultiplicativeOperator_ _Factor_ _TermRecursion_ | `&&`, `*`, `/`, `and` |
| 90 | _TermRecursion_ | &rarr; | &epsilon; | `+`, `-`, `or`, `¦¦`, `!=`, `<`, `<=`, `==`, `>`, `>=`, `]`, `)`, `;`, `,` |
| 91 | _Type_ | &rarr; | _NumberType_ | `float`, `int` |
| 92 | _Type_ | &rarr; | `id` | `id` |
| 93 | _Variable_ | &rarr; | `id` _VariableTail_ | `id` |
| 94 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | _NumberType_ `id` _ArraySizeRecursion_ `;` _VariableDeclarationRecursionThenStatementRecursionA_ | `float`, `int` |
| 95 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | &epsilon; | `}` |
| 96 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | _StatementWithoutAssignment_ _StatementRecursion_ | `for`, `get`, `if`, `put`, `return` |
| 97 | _VariableDeclarationRecursionThenStatementRecursionA_ | &rarr; | `id` _VariableDeclarationRecursionThenStatementRecursionB_ | `id` |
| 98 | _VariableDeclarationRecursionThenStatementRecursionB_ | &rarr; | _VariableTail_ `=` _Expression_ `;` _StatementRecursion_ | `(`, `[`, `.`, `=` |
| 99 | _VariableDeclarationRecursionThenStatementRecursionB_ | &rarr; | `id` _ArraySizeRecursion_ `;` _VariableDeclarationRecursionThenStatementRecursionA_ | `id` |
| 100 | _VariableTail_ | &rarr; | `(` _FunctionArguments_ `)` `.` `id` _VariableTail_ | `(` |
| 101 | _VariableTail_ | &rarr; | _IndexingRecursion_ _VariableTailTail_ | `[`, `.` |
| 102 | _VariableTailTail_ | &rarr; | `.` `id` _VariableTail_ | `.` |
| 103 | _VariableTailTail_ | &rarr; | &epsilon; | `=`, `)` |
| 104 | _VariableThenFunctionDeclarationRecursion_ | &rarr; | _Type_ `id` _VariableThenFunctionDeclarationRecursionTail_ | `id`, `float`, `int` |
| 105 | _VariableThenFunctionDeclarationRecursion_ | &rarr; | &epsilon; | `}` |
| 106 | _VariableThenFunctionDeclarationRecursionTail_ | &rarr; | _ArraySizeRecursion_ `;` _VariableThenFunctionDeclarationRecursion_ | `[`, `;` |
| 107 | _VariableThenFunctionDeclarationRecursionTail_ | &rarr; | _FunctionDeclarationRecursionTail_ | `(` |
| 108 | POP ERROR | POP ERROR | POP ERROR | POP ERROR |
| 109 | SCAN ERROR | SCAN ERROR | SCAN ERROR | SCAN ERROR |

### LL(1) Parsing Table

The columns of the parsing table below represent the list of possible next input tokens.

The rows represent the current leftmost nonterminal in the parse tree.

Every table cell maps the indicates how the current non-terminal should be expanded based on the next input token. The numbers correspond to the rows of entries in the above prediction table, where the production right-hand side expression would be used to expand the current non-terminal.

|  | `program` | `;` | `+` | `-` | `or` | `¦¦` | `[` | `intNum` | `]` | `=` | `class` | `id` | `{` | `}` | `(` | `)` | `floatNum` | `,` | `.` | `&&` | `*` | `/` | `and` | `!` | `not` | `float` | `int` | `:` | `::` | `!=` | `<` | `<=` | `==` | `>` | `>=` | `for` | `get` | `if` | `then` | `else` | `put` | `return` | `$` |
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| `Program` | 1 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 1 | 1 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 1 | 1 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 |
| `AdditiveOperator` | 109 | 109 | 2 | 3 | 4 | 5 | 109 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ArithmeticExpression` | 109 | 108 | 6 | 6 | 109 | 109 | 109 | 6 | 108 | 109 | 109 | 6 | 109 | 109 | 6 | 108 | 6 | 108 | 109 | 109 | 109 | 109 | 109 | 6 | 6 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ArithmeticExpressionTail` | 109 | 8 | 7 | 7 | 7 | 7 | 109 | 109 | 8 | 109 | 109 | 109 | 109 | 109 | 109 | 8 | 109 | 8 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 8 | 8 | 8 | 8 | 8 | 8 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ArithmeticOrRelationalExpression` | 109 | 10 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 10 | 109 | 10 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 9 | 9 | 9 | 9 | 9 | 9 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ArraySize` | 109 | 108 | 109 | 109 | 109 | 109 | 11 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ArraySizeRecursion` | 109 | 13 | 109 | 109 | 109 | 109 | 12 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 13 | 109 | 13 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `AssignmentStatement` | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 14 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ClassDeclaration` | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 15 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `ClassDeclarationRecursion` | 17 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 16 | 17 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 17 | 17 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `Expression` | 109 | 108 | 18 | 18 | 109 | 109 | 109 | 18 | 109 | 109 | 109 | 18 | 109 | 109 | 18 | 108 | 18 | 108 | 109 | 109 | 109 | 109 | 109 | 18 | 18 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `Factor` | 109 | 108 | 22 | 22 | 108 | 108 | 109 | 24 | 108 | 109 | 109 | 20 | 109 | 109 | 19 | 108 | 23 | 108 | 109 | 108 | 108 | 108 | 108 | 21 | 21 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionArguments` | 109 | 109 | 25 | 25 | 109 | 109 | 109 | 25 | 109 | 109 | 109 | 25 | 109 | 109 | 25 | 26 | 25 | 109 | 109 | 109 | 109 | 109 | 109 | 25 | 25 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionArgumentsTail` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 27 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionArgumentsTailRecursion` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 29 | 109 | 28 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionBody` | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 30 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionCallOrVariable` | 109 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 108 | 109 | 109 | 31 | 109 | 109 | 109 | 108 | 109 | 108 | 109 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionCallOrVariableTail` | 109 | 108 | 108 | 108 | 108 | 108 | 32 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 32 | 108 | 109 | 108 | 32 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionCallOrVariableTailRecursion` | 109 | 34 | 34 | 34 | 34 | 34 | 109 | 109 | 34 | 109 | 109 | 109 | 109 | 109 | 109 | 34 | 109 | 34 | 33 | 34 | 34 | 34 | 34 | 109 | 109 | 109 | 109 | 109 | 109 | 34 | 34 | 34 | 34 | 34 | 34 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionCallParensOrIndexing` | 109 | 108 | 108 | 108 | 108 | 108 | 36 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 35 | 108 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionDeclarationRecursionStart` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 37 | 109 | 38 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 37 | 37 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionDeclarationRecursionTail` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 39 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionDefinition` | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 40 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 40 | 40 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionDefinitionRecursion` | 42 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 41 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 41 | 41 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionHeader` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 43 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 43 | 43 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionParameters` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 44 | 109 | 109 | 109 | 45 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 44 | 44 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionParametersTail` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 46 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `FunctionParametersTailRecursion` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 48 | 109 | 47 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `IdListRecursion` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 50 | 109 | 109 | 109 | 109 | 49 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `Indexing` | 109 | 108 | 108 | 108 | 108 | 108 | 51 | 109 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `IndexingRecursion` | 109 | 53 | 53 | 53 | 53 | 53 | 52 | 109 | 53 | 53 | 109 | 109 | 109 | 109 | 109 | 53 | 109 | 53 | 53 | 53 | 53 | 53 | 53 | 109 | 109 | 109 | 109 | 109 | 109 | 53 | 53 | 53 | 53 | 53 | 53 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `MultiplicativeOperator` | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 54 | 55 | 56 | 57 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `NegationOperator` | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 58 | 59 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `NumberSign` | 109 | 109 | 60 | 61 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `NumberType` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 62 | 63 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `OptionalInheritanceList` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 65 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 64 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `OptionalNamespacing` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 66 | 109 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `OptionalNamespacingTail` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 68 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 67 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `RelationalExpression` | 109 | 108 | 69 | 69 | 109 | 109 | 109 | 69 | 109 | 109 | 109 | 69 | 109 | 109 | 69 | 109 | 69 | 109 | 109 | 109 | 109 | 109 | 109 | 69 | 69 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `RelationalOperator` | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 108 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 108 | 109 | 109 | 109 | 109 | 70 | 71 | 72 | 73 | 74 | 75 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `Statement` | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 76 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 77 | 77 | 77 | 109 | 108 | 77 | 77 | 109 |
| `StatementBlock` | 109 | 79 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 78 | 80 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 78 | 78 | 78 | 109 | 79 | 78 | 78 | 109 |
| `StatementRecursion` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 81 | 109 | 82 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 81 | 81 | 81 | 109 | 109 | 81 | 81 | 109 |
| `StatementWithoutAssignment` | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 83 | 84 | 85 | 109 | 108 | 86 | 87 | 109 |
| `Term` | 109 | 108 | 88 | 88 | 108 | 108 | 109 | 88 | 108 | 109 | 109 | 88 | 109 | 109 | 88 | 108 | 88 | 108 | 109 | 109 | 109 | 109 | 109 | 88 | 88 | 109 | 109 | 109 | 109 | 108 | 108 | 108 | 108 | 108 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `TermRecursion` | 109 | 90 | 90 | 90 | 90 | 90 | 109 | 109 | 90 | 109 | 109 | 109 | 109 | 109 | 109 | 90 | 109 | 90 | 109 | 89 | 89 | 89 | 89 | 109 | 109 | 109 | 109 | 109 | 109 | 90 | 90 | 90 | 90 | 90 | 90 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `Type` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 92 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 91 | 91 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `Variable` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 109 | 93 | 109 | 109 | 109 | 108 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `VariableDeclarationRecursionThenStatementRecursionA` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 97 | 109 | 95 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 94 | 94 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 96 | 96 | 96 | 109 | 109 | 96 | 96 | 109 |
| `VariableDeclarationRecursionThenStatementRecursionB` | 109 | 109 | 109 | 109 | 109 | 109 | 98 | 109 | 109 | 98 | 109 | 99 | 109 | 108 | 98 | 109 | 109 | 109 | 98 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `VariableTail` | 109 | 109 | 109 | 109 | 109 | 109 | 101 | 109 | 109 | 108 | 109 | 109 | 109 | 109 | 100 | 108 | 109 | 109 | 101 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `VariableTailTail` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 103 | 109 | 109 | 109 | 109 | 109 | 103 | 109 | 109 | 102 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `VariableThenFunctionDeclarationRecursion` | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 104 | 109 | 105 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 104 | 104 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |
| `VariableThenFunctionDeclarationRecursionTail` | 109 | 106 | 109 | 109 | 109 | 109 | 106 | 109 | 109 | 109 | 109 | 109 | 109 | 108 | 107 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 | 109 |


## List of Terminal Symbols (Tokens)

- Identifier (`id`)
- Keyword (`program`)
- Keyword (`class`)
- Keyword (`if`)
- Keyword (`then`)
- Keyword (`for`)
- Keyword (`get`)
- Keyword (`put`)
- Keyword (`return`)
- Keyword (`float`)
- Keyword (`int`)
- Semicolon (`;`)
- OpenCurlyBrace (`{`)
- CloseCurlyBrace (`}`)
- InheritanceOperator (`:`)
- Comma (`,`)
- OpenParens (`(`)
- CloseParens (`)`)
- ScopeResolutionOperator (`::`)
- AssignmentOperator (`=`)
- RelationalOperator
- MathOperator (`+`)
- MathOperator (`-`)
- MathOperator (`*`)
- MathOperator (`/`)
- BinaryLogicalOperator (`or`)
- BinaryLogicalOperator (`||`)
- BinaryLogicalOperator (`and`)
- BinaryLogicalOperator (`&&`)
- UnaryLogicalOperator (`not`)
- UnaryLogicalOperator (`!`)
- Integer
- Float
- OpenSquareBracket (`[`)
- CloseSquareBracket (`]`)
- AccessorOperator (`.`)

