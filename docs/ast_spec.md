# Nack AST Specification

A Nack AST represents a Nack program that can be executed by the interpreter. This document describes the layout of a
valid AST.

There are two types of Nack AST nodes: **grouping nodes** and **token nodes**. Grouping nodes represent a logical chunk
of a Nack program, like a transform definition, an expression, a type definition, etc. Grouping nodes contain a single
string label that represents the type of the grouping node. Token nodes represent a part of the program that is defined
by a single Nack language token, like an identifier, a literal expression, or an operator.

All valid Nack ASTs must have a singular root node, which must be a `PROGRAM` node.

## Grouping Nodes

This section details all the valid grouping nodes an AST can have.

### `EXPRESSION`

`EXPRESSION` nodes represent a special expression subtree. An `EXPRESSION` node will always have exactly one child,
which can be a token node or a grouping node. That child may have its own children, representing their own expression
subtrees. Beyond the initial `EXPRESSION` root node of the subtree, an expression subtree is not required to have
another `EXPRESSION` node to be valid, but it is allowed and valid to do so.

#### Children

An `EXPRESSION` node must have exactly one child node of the following types:

- `intLiteral`
- `identifier`

### `PROGRAM`

A `PROGRAM` node is always the root node of the Nack AST. A `PROGRAM` node should not appear anywhere else in the tree.

#### Children

A `PROGRAM` node can have any number of children of the following node types in any order:

- `EXPRESSION`