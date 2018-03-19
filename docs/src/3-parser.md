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
Program                         -> ClassDeclarationRecursion FunctionDefinitionRecursion program FunctionBody ; 
ClassDeclarationRecursion       -> ClassDeclaration ClassDeclarationRecursion 
                                 | EPSILON 
ClassDeclaration                -> class id OptionalInheritance { VariableThenFunctionDeclarationRecursion ; 
OptionalInheritance             -> : id MultipleSuperClasses 
                                 | EPSILON 
MultipleSuperClasses            -> , id MultipleSuperClasses 
                                 | EPSILON 
FunctionDeclarationRecursion    -> Type id ( FunctionParameters ) ; FunctionDeclarationRecursion 
                                 | }
FunctionHead                    -> Type OptionalNamespace ( FunctionParameters ) 
OptionalNamespace               -> id OptionalNamespaceTail 
OptionalNamespaceTail           -> :: id | EPSILON 
FunctionDefinitionRecursion     -> FunctionDefinition FunctionDefinitionRecursion 
                                 | EPSILON 
FunctionDefinition              -> FunctionHead FunctionBody ; 
FunctionBody                    -> { VariableDeclarationThenStatementRecursion 
VariableDeclarationThenStatementRecursion    -> Type id ArraySizeRecursion ; VariableDeclarationThenStatementRecursion | StatementRecursion
VariableThenFunctionDeclarationRecursion    -> Type id ArraySizeRecursion ; VariableThenFunctionDeclarationRecursion | FunctionDeclarationRecursion
StatementRecursion              -> Statement StatementRecursion 
                                 | }
Statement                       -> for ( Type id AssignmentOperator Expression ; RelationalExpression ; AssignmentStatement ) StatementBlock ; 
                                 | if ( Expression ) then StatementBlock ; 
                                 | get ( Variable ) ; 
                                 | put ( Expression ) ; 
                                 | return ( Expression ) ; 
                                 | AssignmentStatement 
AssignmentStatement             -> Variable AssignmentOperator Expression 
StatementBlock                  -> { StatementRecursion
                                 | Statement 
                                 | EPSILON 
Expression                      -> ArithmeticExpression 
                                 | RelationalExpression 
RelationalExpression            -> ArithmeticExpression RelationalOperator ArithmeticExpression 
ArithmeticExpression            -> Term ArithmeticExpressionRecursion 
ArithmeticExpressionRecursion   -> AdditiveOperator Term ArithmeticExpressionRecursion 
                                 | EPSILON 
NumberSign                      -> + | - 
Term                            -> Factor TermRecursion 
TermRecursion                   -> MultiplicativeOperator Factor TermRecursion 
                                 | EPSILON 
Factor                          -> ( ArithmeticExpression ) 
                                 | floatNum 
                                 | intNum 
                                 | FunctionCall 
                                 | Variable 
                                 | NegationOperator Factor 
                                 | NumberSign Factor 
NegationOperator                -> ! | not 
Variable                        -> NestedIdRecursion IndexingRecursion 
FunctionCall                    -> NestedIdRecursion ( FunctionArguments ) 
NestedIdRecursion               -> id NestedId NestedIdRecursion id | id
NestedId                        -> ( FunctionArguments ) . 
                                 | IndexingRecursion . 
IndexingRecursion               -> Indexing IndexingRecursion 
                                 | EPSILON 
Indexing                        -> [ ArithmeticExpression ] 
ArraySizeRecursion              -> ArraySize ArraySizeRecursion 
                                 | EPSILON 
ArraySize                       -> [ intNum ] 
Type                            -> float | id | int 
FunctionParameters              -> Type id ArraySizeRecursion FunctionParametersTailRecursion 
                                 | EPSILON 
FunctionArguments               -> Expression FunctionArgumentsTailRecusion 
                                 | EPSILON 
FunctionParametersTailRecursion -> FunctionParametersTail FunctionParametersTailRecursion 
                                 | EPSILON 
FunctionParametersTail          -> , Type id ArraySizeRecursion 
FunctionArgumentsTailRecusion   -> FunctionArgumentsTail FunctionArgumentsTailRecusion 
                                 | EPSILON 
FunctionArgumentsTail           -> , Expression 
AssignmentOperator              -> = 
RelationalOperator              -> < | <= | <> | == | > | >= 
AdditiveOperator                -> + | - | or | ¦¦ 
MultiplicativeOperator          -> && | * | / | and 
```



