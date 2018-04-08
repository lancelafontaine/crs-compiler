# `crs` Semantic Analysis

## Syntax-Directed Translation

In order to perform semantic analysis of our code as well as ultimately generate an abstract syntax tree (AST), the LL(1) context-free grammar derived during the parsing stage was transformed into an attribute grammar. This involved the addition of semantic actions to the right-hand sides of our grammar's productions, which ultimately represent the execution of a subroutine to accumulate and process semantic values or semantic attributes about the program being parsed. These values can have properties such as type, value, memory size or location, translated code, correctness, or more.

Semantic values are accumulated in semantic records until they can be processed by further semantic actions. However, *attribute migration* will be required given that these semantic rules and values will need to be available throughout the parsing step.

Semantic actions are associated with nodes within the parse tree:
- Semantic actions at the leaves of the tree typically gather semantic values, from either the token type of the node, or through the type recorded in a symbol table for a token already seen
- Semantic actions at intermediate nodes ultimately use and validate these semantic values and pass the information up the tree through attribute migration.

In table-driven parsers, semantic action symbols are inserted into the RHS of productions and pushed onto their own _semantic stack_. Executing a semantic action typically pops a semantic record from the semantic stack, does some processing, then pushes a semantic record back onto the stack.

## AST Generation

The method for generating an AST was modified slightly...

1. Undergo parsing until a semantic action is discovered as a PARSE symbol
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

Multiple semantic action types can end up using the same semantic action. This is to namespace specific types, since our way of generating the AST relies on pop the stack until a semantic action does not recognize that type anymore.

Semantic actions for leaf nodes are placed *before* the terminal parse symbol, in order to be able to observe the next token in the token queue before it is dequed. Semantic actions for intermediate nodes are placed after the non-terminal parse symbol, in order to ensure that all productions have already taken place, and that these types are all placed on the semantic stack before creating a family.

We're using the `petgraph` library to represent the AST. As opposed to referencing to individual nodes that we can keep on the stack as the graph grows, we're keeping track of nodes through unique indices that are assigned to nodes as they are added to a directed graph. This singleton mutex-locked graph object is mutably passed to semantic actions as it is being pushed onto the stack.
