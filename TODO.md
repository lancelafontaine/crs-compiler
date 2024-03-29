# GENERAL

- [ ] Add a `verbose` argument for fine-grained control of output
- [ ] Add a `log-output` argument for determining whether to write to files in a directory
- [ ] If the source file path is incorrect, `quicli` throws an error.
- [ ] Create a `log` directory. Logs are written to a file every time, make it timestamp-based
- [ ] Keep a log buffer in memory until the end of the program, then output it all at once.

# LEXICAL ANALYSIS
- [ ] Fix display (color) of line/character on lexical error
- [ ] Refactoring line/character reporting on error. The token struct needs to contain the location in the source code, as it's propagated through different compiler stages.
- [ ] Implement a panic mode error recovery mechanism
    - Dicard tokens until a common well-defined subroutine token is found. See slides.
- [ ] Replace the integration tests being run with a bash script by `assert_cli` and Rust integration tests
- [ ] Add the `,` token to the lexer for multiple inheritance, multiple parameters
- [ ] Make `Token.lexeme` an `Option` type? Or possibly refactor `TokenClass` to also have an `Option<lexeme>` instead of keeping two fields on the `Token` struct

# SYNTACTIC ANALYSIS

- [ ] Identify all error types and edge cases.
- [ ] Implement error reporting mechanism for the parser (see `skipErrors()` in the slides), including source location display. It must report all errors present in the source code.
- [ ] log to a file the derivation that proves that the source program can be derived from the starting symbol
    - Exactly as is shown in the right side of the window of the "Derivation" tab in the kfgEdit tool
- [ ] Unit tests
- [ ] Integration tests

# SYNTAX-DRIVEN TRANSLATION AND AST-GENERATION


- [ ] Throw an error if there are AST nodes other than the start node that are node pointed to.
- [ ] Throw an error for some nodes if it has more/less children than is expected for that particular node type.
- [ ] Implement error reporting mechanism for the syntax-driven translation and final AST, including source location display. It must report all errors present in the source code.
- [ ] Throw an error if there is more than one node index in the semantic stack at the end of AST generation.
- [ ] Unit tests
- [ ] Integration tests
- [ ] Option to export a GraphViz file from petgraph for the AST

# SEMANTIC ANALYSIS

- [ ] Option to export a GraphViz file from petgraph for the symbol tables.
- [ ] Errors should be reported in synchronized order, even if different phases are implemented and errors are found in different phases.
- [ ] Log the symbol table to a file

# CODE GENERATION

TBD

# NICETIES

- [ ] Run builds with Travis CI
- [ ] Add code coverage with CodeCov
- [ ] Add cargo examples
- [ ] Benchmark with criterion.rs
- [ ] Add badges to README



# IMPLEMENTATION NOTES

# Next Steps

SECOND PASS:
  - error on multiple declared identifiers in the same scope (also takes care of the 'only one main function' case)
  - variables must be defined before being used in statements
  - warnings for shadowed inherited members.

  - Type inference in expressions through attribute migration.
    - For now, let's assume we can't do use an operator with different token types.
    - Similarly, both operands of an assignment should be the same type.
  - Type checking for consistency:
      - Expressions
      - Assignments
      - Return statements
  - Ensure a function or class method is being called with the right number and type of parameters.
  - Make sure that use of any identifier has been defined. Otherwise, error with (undefined local variable, undefined free function, undefined member, undefined class)
  - Using an array variable should be done with the same number of dimensions as was declared
  - Ensure that expression in array indexing results in an integer type
  - Circular class dependencies through data members or inheritance.
  - The accessor operator can only be used on variables of a class type. The identifier after the accessor must be a member of that class. Give an (undefined member) error otherwise.
  - Ensure that the function declaration's return type is consistent with the value being returned.


### CODE GENERATION

- Add a column to the symbol table to store the unique label.
  - For each variable memory is allocated for, associate it with a unique tag
  - For each variable memory is allocated for, associate it with a memory size
  - Next time, when you want to access this variable (in-memory location) you can just get its address by using that predefined tag in your table.


For this code:
```
int a;
int b;
int c;
```

int is 32bits, 4 bytes
Generate:
```
% space for variable a
a   res 4
% space for variable b
b   res 4
% space for variable c
c   res 4
```

For this code:
```
int a[3][4];
int b[2];
int c[2][2][2];
```

Generate:
```
% space for variable a
a   res 4
% space for variable b
b   res 4
% space for variable c
c   res 4
```

For this code:
```
class A : B {
  int a1;
  int a2[2][2];
  C a3;
}
class B {
  int b1;
  int b2;
}
class C {
  int c1;
  int c2;
}
program {
  A a;
}
```
Generate:
```
% space for variable a
a res 36;
```

To load or change thje content of an integer variable:
```
lw r1,x(r0)
sw x(r0),r1
```
where x is the label of variable x, r1 is the register containing the value of
variable x and r0 is assumed to contain 0 (offset).

Accessing elements of an array of integers, using offsets:
```
x = a[2];
```
is
```
addi r1,r0,8
lw r2,a(r1)
sw x(r0),r2
```

