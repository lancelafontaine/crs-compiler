# `crs` Semantic Analysis

## Semantic Analysis

### Symbol Tables
Symbol tables are a tool used during compilation for performing semantic checks on your program. These involve verifications that can only be expressed in a context-sentive grammar, such as type inteference, type checking, and much more

This compiler generates symbol tables by performing semantic actions when traversing the AST previously generated.

The implementation of a symbol table was done with a graph data structure. Some assumptions about the system are listed below:

- Every node can have either a `Table` or a `Record` type.
- A `Table` node can only have `Record` nodes as children.
- A `Record` node can only have `Table` nodes as children.

Imposing these restrictions on our graph results in some nice properties.
- Visiting all records in a table is done by visiting all of a `Table` node's immediate children (with a tree depth of 1).
- Viewing all smallers scopes (nested symbol tables) can be done by ignoring all nodes that are leaves at 1 tree depth and visiting all tables that are at 2 tree depths.
- Visiting a parent scope (symbol table) is done at any point by traversing 2 nodes up the tree from any `Table` node.
- If a `Table` node is a leaf node, this necessarily represents an empty scope.

### AST tree traversal with the Visitor pattern

The Visitor is a useful design pattern during AST tree traversal, in order to group semantic actions into one component during a specific phase of semantic analysis.

A more funcitonal approach to the Visitor pattern was used in this program, but the core premise remains the same.

A "visitor" in the form of a function callback was passed to the DFS tree-traversal algorith. This callback was executed at every node. Once the callback was executed, it executed specific visit functions based on the node type.
