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


We can simplify the non-terminals of the language by following this legend:

| Previous Non-Terminal Symbol | New Non-Terminal Symbol |
|:-----:|:-----:|
| _prog_ | _S_ |
| _classDeclRecursion_ | _A_ |
| _funcDefRecursion_ | _B_ |
| _funcBody_ | _C_ |
| _classDecl_ | _D_ |
| _optionalInheritance_ | _E_ |
| _varDeclRecursion_ | _F_ |
| _funcDeclRecursion_ | _G_ |
| _MultipleSuperClasses_ | _H_ |
| _funcDecl_ | _I_ |
| _type_ | _J_ |
| _fParams_ | _K_ |
| _funcHead_ | _L_ |
| _optionalNamespace_ | _M_ |
| _funcDef_ | _N_ |
| _statementRecursion_ | _O_ |
| _varDecl_ | _P_ |
| _arraySizeRecursion_ | _Q_ |
| _statement_ | _R_ |
| _assignStat_ | _T_ |
| _expr_ | _U_ |
| _statBlock_ | _V_ |
| _assignOp_ | _W_ |
| _relExpr_ | _X_ |
| _variable_ | _Y_ |
| _arithExpr_ | _Z_ |
| _relOp_ | _AA_ |
| _addOp_ | _AB_ |
| _term_ | _AC_ |
| _sign_ | _AD_ |
| _multOp_ | _AE_ |
| _factor_ | _AF_ |
| _functionCall_ | _AG_ |
| _negationOperator_ | _AH_ |
| _idnestRecursion_ | _AI_ |
| _indiceRecursion_ | _AJ_ |
| _aParams_ | _AK_ |
| _idnest_ | _AL_ |
| _indice_ | _AM_ |
| _arraySize_ | _AN_ |
| _fParamsTailRecursion_ | _AO_ |
| _aParamsTailRecursion_ | _AP_ |
| _fParamsTail_ | _AQ_ |
| _aParamsTail_ | _AR_ |


This results in the same BNF grammar but with single-letter non-terminal symbols.

| LHS | &rarr; | RHS |
|:---:|:----:|:----:|
| _S_ | &rarr; | _A_ _B_ `program` _C_ `;`|
| _A_ | &rarr; | _A_ _D_ \| &epsilon; |
| _B_ | &rarr; | _B_ _N_ \| &epsilon; |
| _C_ | &rarr; | `{` _F_ _O_ `}` |
| _D_ | &rarr; | `class` `id` _E_ `{` _F_ _G_ `}` `;` |
| _E_ | &rarr; | `:` `id` _H_ \| &epsilon;|
| _F_ | &rarr; | _F_ _P_ \| &epsilon; |
| _G_ | &rarr; | _G_ _I_ \| &epsilon; |
| _H_ | &rarr; | _H_ `,` `id` \| &epsilon; |
| _I_ | &rarr; | _J_ `id` `(` _K_ `)` `;` |
| _J_ | &rarr; | `int` \| `float` \| `id` |
| _K_ | &rarr; | _J_ `id` _Q_ _AO_ \| &epsilon; |
| _L_ | &rarr; | _J_ _M_ `id` `(` _K_ `)` |
| _M_ | &rarr; |  `id` `::` \| &epsilon;|
| _N_ | &rarr; | _L_ _C_ `;` |
| _O_ | &rarr; | _O_ _R_ \| &epsilon; |
| _P_ | &rarr; | _J_ `id` _Q_ `;` |
| _Q_ | &rarr; | _Q_ _AN_ \| &epsilon; |
| _R_ | &rarr; | _T_ \| `if` `(` _U_ `)` `then` _V_ `;` \| `for` `(` _J_ `id` _W_ _U_ `;` _X_ `;` _T_  `)` _V_ `;` \| <br/> `get` `(` _Y_ `)` `;` \| `put` `(` _U_ `)` `;` \| `return` `(` _U_ `)` `;` |
| _T_ | &rarr; | _Y_ _W_ _U_ |
| _U_ | &rarr; | _Z_ \| _X_ |
| _V_ | &rarr; | `{` _O_ `}` \| _R_ \| &epsilon; |
| _W_ | &rarr; | `=` |
| _X_ | &rarr; | _Z_ _AA_ _Z_ |
| _Y_ | &rarr; | _AI_ `id` _AJ_ |
| _Z_ | &rarr; | _Z_ _AB_ _AC_ \| _AC_ |
| _AA_ | &rarr; | `==` \| `<>` \| `<` \| `>` \| `<=` \| `>=` |
| _AB_ | &rarr; | `+` \| `-` \| `or` \| `¦¦` |
| _AC_ | &rarr; | _AC_ _AE_ _AF_ \| _AF_ |
| _AD_ | &rarr; | `+` \| `-` |
| _AE_ | &rarr; | `*` \| `/` \| `and` \| `&&` |
| _AF_ | &rarr; | _Y_ \| _AG_ \| `intNum` \| `floatNum` \| `(` _Z_ `)` \| _AH_ _AF_ \| _AD_ _AF_ |
| _AG_ | &rarr; | _AI_ `id` `(` _AK_ `)` |
| _AH_ | &rarr; | `not` \| `!` |
| _AI_ | &rarr; | _AI_ _AL_ \| &epsilon; |
| _AJ_ | &rarr; | _AJ_ _AM_ \| &epsilon; |
| _AK_ | &rarr; | _U_ _AP_ \| &epsilon; |
| _AL_ | &rarr; | `id` _AJ_ `.` \| `id` `(` _AK_ `)` `.` |
| _AM_ | &rarr; | `[` _Z_ `]` |
| _AN_ | &rarr; | `[` `intNum` `]` |
| _AO_ | &rarr; | _AO_ _AQ_ \| &epsilon; |
| _AP_ | &rarr; | _AP_ _AR_ \| &epsilon; |
| _AQ_ | &rarr; | `,` _J_ `id` _Q_ |
| _AR_ | &rarr; | `,` _U_ |

An [AtoCC](http://atocc.de)-compatible text format of the above grammar is shown below:

```
S -> A B 'program' C ';'
A -> A D | '
B -> B N | '
C -> '{' F O '}'
D -> 'class' 'id' E '{' F G '}' ';'
E -> ':' 'id' H | '
F -> F P | '
G -> G I | '
H -> H ',' 'id' | '
I -> J 'id' '(' K ')' ';'
J -> 'int' | 'float' | 'id'
K -> J 'id' Q AO | '
L -> J M 'id' '(' K ')'
M ->  'id' '::' | '
N -> L C ';'
O -> O R | '
P -> J 'id' Q ';'
Q -> Q AN | '
R -> T | 'if' '(' U ')' 'then' V ';' | 'for' '(' J 'id' W U ';' X ';' T  ')' V ';' | 'get' '(' Y ')' ';' | 'put' '(' U ')' ';' | 'return' '(' U ')' ';'
T -> Y W U
U -> Z | X
V -> '{' O '}' | R | '
W -> '='
X -> Z AA Z
Y -> AI 'id' AJ
Z -> Z AB AC | AC
AA -> '==' | '<>' | '<' | '>' | '<=' | '>='
AB -> '+' | '-' | 'or' | '||'
AC -> AC AE AF | AF
AD -> '+' | '-'
AE -> '*' | '/' | 'and' | '&&'
AF -> Y | AG | 'intNum' | 'floatNum' | '(' Z ')' | AH AF | AD AF
AG -> AI 'id' '(' AK ')'
AH -> 'not' | '!'
AI -> AI AL | '
AJ -> AJ AM | '
AK -> U AP | '
AL -> 'id' AJ '.' | 'id' '(' AK ')' '.'
AM -> '[' Z ']'
AN -> '[' 'intNum' ']'
AO -> AO AQ | '
AP -> AP AR | '
AQ -> ',' J 'id' Q
AR -> ',' U
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

Ultimately, too many changes were made to the grammar to describe every ambiguity, left factoring or left-recursion elimination. The resulting grammar is shown below.

```
S  -> A B program C ;
A  -> D A | EPSILON
B  -> N B | EPSILON
C  -> { F }
D  -> class id E { AG } ;
E  -> : id H | EPSILON
F  -> id AY | AX | P F
G  -> id Q ; F
H  -> , id H | EPSILON
I  -> AL I
J  -> float | id | int
K  -> J id Q AO | EPSILON
L  -> J M ( K )
M  -> id BA
N  -> L C ;
O  -> id T O | R O | EPSILON
P  -> float id Q ; | int id Q ;
Q  -> AN Q | EPSILON
R  -> for ( J id = U ; X ; id T ) V ; | get ( id Y ) ; | if ( U ) then V ; | put ( U ) ; | return ( U ) ;
T  -> Y = U
U  -> Z AZ
V  -> { O } | id T | R | EPSILON
W  -> float id AI | id id AI | int id AI | EPSILON
X  -> Z AA Z
Y  -> I AJ
Z  -> AC BE
AA -> < | <= | <> | == | > | >=
AB -> + | - | or | ||
AC -> AF BD
AD -> + | -
AE -> and | && | * | /
AF -> ( Z ) | floatNum | id BB | intNum | AH AF | AD AF
AG -> float id AT | id id AT | int id AT | EPSILON
AH -> ! | not
AI -> ( K ) ; W
AJ -> AM AJ | EPSILON
AK -> U AP | EPSILON
AL -> ( AK ) . | AJ .
AM -> [ Z ]
AN -> [ intNum ]
AO -> AQ AO | EPSILON
AP -> AR AP | EPSILON
AQ -> , J id Q
AR -> , U
AS -> Q ; AG
AT -> AI | AS
AU -> id AW
AV -> AX | AU
AW -> T AV
AX -> R AV | EPSILON
AY -> AW | G
AZ -> AA Z | EPSILON
BA -> :: id | EPSILON
BB -> I BC
BC -> ( AK ) | AJ
BD -> AE AF BD | EPSILON
BE -> AB AC BE | EPSILON
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
| `ε` | `ε` |
| `{` | `{` |
| `}` | `}` |
| `class` | `class` |
| `id` | `id` |
| `:` | `:` |
| `,` | `,` |
| `float` | `float` |
| `int` | `int` |
| `(` | `(` |
| `)` | `)` |
| `for` | `for` |
| `=` | `=` |
| `get` | `get` |
| `if` | `if` |
| `then` | `then` |
| `put` | `put` |
| `return` | `return` |
| `<` | `<` |
| `<=` | `<=` |
| `<>` | `<>` |
| `==` | `==` |
| `>` | `>` |
| `>=` | `>=` |
| `+` | `+` |
| `-` | `-` |
| `or` | `or` |
| `¦¦` | `¦¦` |
| `and` | `and` |
| `&&` | `&&` |
| `*` | `*` |
| `/` | `/` |
| `floatNum` | `floatNum` |
| `intNum` | `intNum` |
| `not` | `not` |
| `!` | `!` |
| `.` | `.` |
| `[` | `[` |
| `]` | `]` |
| `::` | `::` |
| _S_ | `program`, `ε`, `class`, `float`, `id`, `int` |
| _A_ | `ε`, `class` |
| _B_ | `ε`, `float`, `id`, `int` |
| _C_ | `{` |
| _D_ | `class` |
| _E_ | `:`, `ε` |
| _F_ | `id`, `ε`, `for`, `get`, `if`, `put`, `return`, `float`, `int` |
| _G_ | `id` |
| _H_ | `,`, `ε` |
| _I_ | `(`, `.`, `ε`, `[` |
| _J_ | `float`, `id`, `int` |
| _K_ | `ε`, `float`, `id`, `int` |
| _L_ | `float`, `id`, `int` |
| _M_ | `id` |
| _N_ | `float`, `id`, `int` |
| _O_ | `id`, `ε`, `for`, `get`, `if`, `put`, `return` |
| _P_ | `float`, `int` |
| _Q_ | `ε`, `[` |
| _R_ | `for`, `get`, `if`, `put`, `return` |
| _T_ | `(`, `.`, `ε`, `[` |
| _U_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _V_ | `{`, `id`, `ε`, `for`, `get`, `if`, `put`, `return` |
| _W_ | `float`, `id`, `int`, `ε` |
| _X_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _Y_ | `(`, `.`, `ε`, `[` |
| _Z_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AA_ | `<`, `<=`, `<>`, `==`, `>`, `>=` |
| _AB_ | `+`, `-`, `or`, `¦¦` |
| _AC_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AD_ | `+`, `-` |
| _AE_ | `and`, `&&`, `*`, `/` |
| _AF_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AG_ | `float`, `id`, `int`, `ε` |
| _AH_ | `not`, `!` |
| _AI_ | `(` |
| _AJ_ | `ε`, `[` |
| _AK_ | `ε`, `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AL_ | `(`, `.`, `ε`, `[` |
| _AM_ | `[` |
| _AN_ | `[` |
| _AO_ | `ε`, `,` |
| _AP_ | `ε`, `,` |
| _AQ_ | `,` |
| _AR_ | `,` |
| _AS_ | `;`, `ε`, `[` |
| _AT_ | `(`, `;`, `ε`, `[` |
| _AU_ | `id` |
| _AV_ | `ε`, `for`, `get`, `if`, `put`, `return`, `id` |
| _AW_ | `(`, `.`, `ε`, `[` |
| _AX_ | `ε`, `for`, `get`, `if`, `put`, `return` |
| _AY_ | `(`, `.`, `ε`, `[`, `id` |
| _AZ_ | `ε`, `<`, `<=`, `<>`, `==`, `>`, `>=` |
| _BA_ | `::`, `ε` |
| _BB_ | `(`, `.`, `ε`, `[` |
| _BC_ | `(`, `ε`, `[` |
| _BD_ | `ε`, `and`, `&&`, `*`, `/` |
| _BE_ | `ε`, `+`, `-`, `or`, `¦¦` |

### _FOLLOW_ Sets

| Non-Terminal Symbol | Follow Set |
|:----:|:-----:|
| _S_ | `$` |
| _A_ | `program`, `float`, `id`, `int` |
| _B_ | `program` |
| _C_ | `;` |
| _D_ | `class`, `program`, `float`, `id`, `int` |
| _E_ | `{` |
| _F_ | `}` |
| _G_ | `}` |
| _H_ | `{` |
| _I_ | `(`, `[`, `=`, `)`, `and`, `&&`, `*`, `/`, `+`, `-`, `or`, `¦¦`, `]`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _J_ | `id` |
| _K_ | `)` |
| _L_ | `{` |
| _M_ | `(` |
| _N_ | `float`, `id`, `int`, `program` |
| _O_ | `}` |
| _P_ | `id`, `for`, `get`, `if`, `put`, `return`, `float`, `int`, `}` |
| _Q_ | `;`, `,`, `)` |
| _R_ | `for`, `get`, `if`, `put`, `return`, `id`, `;`, `}` |
| _T_ | `for`, `get`, `if`, `put`, `return`, `id`, `)`, `;`, `}` |
| _U_ |  `,`, `;`, `)`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _V_ | `;` |
| _W_ | `}` |
| _X_ | `;` |
| _Y_ | `=`, `)` |
| _Z_ | `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _AA_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AB_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AC_ | `+`, `-`, `or`, `¦¦`, `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _AD_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AE_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AF_ | `and`, `&&`, `*`, `/`, `+`, `-`, `or`, `¦¦`, `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _AG_ | `}` |
| _AH_ | `(`, `floatNum`, `id`, `intNum`, `not`, `!`, `+`, `-` |
| _AI_ | `}` |
| _AJ_ | `.`, `=`, `)`, `and`, `&&`, `*`, `/`, `+`, `-`, `or`, `¦¦`,  `]`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _AK_ | `)` |
| _AL_ | `(`, `.`, `[` |
| _AM_ | `[`, `.`, `=`, `)`, `and`, `&&`, `*`, `/`, `+`, `-`, `or`, `¦¦`, `]`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _AN_ | `[`, `;`, `,`, `)` |
| _AO_ | `)` |
| _AP_ | `)` |
| _AQ_ | `,`, `)` |
| _AR_ | `,`, `)` |
| _AS_ | `}` |
| _AT_ | `}` |
| _AU_ | `}` |
| _AV_ | `}` |
| _AW_ | `}` |
| _AX_ | `}` |
| _AY_ | `}` |
| _AZ_ | `,`, `;`, `)`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _BA_ | `(` |
| _BB_ | `and`, `&&`, `*`, `/`, `+`, `-`, `or`, `¦¦`, `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _BC_ | `and`, `&&`, `*`, `/`, `+`, `-`, `or`, `¦¦`, `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _BD_ | `+`, `-`, `or`, `¦¦`, `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |
| _BE_ | `]`, `)`, `<`, `<=`, `<>`, `==`, `>`, `>=`, `;`, `,`, `for`, `get`, `if`, `put`, `return`, `id`, `}` |

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

