# Nack Language Specification

## Expressions

Nack expressions are composed of one or more expression atoms combined through the use of operators.

### Expression Atoms

The following is a list of self-contained expressions that produce a single Nack value:

- Integer literals
  - Yields `Int`
- Boolean literals
  - Yields `Bool`

## Language Grammar

### Rules

```
PROGRAM -> PROGRAM_UNIT* eof

PROGRAM_UNIT -> EXPRESSION

EXPRESSION -> MULT_EXPR ((plusSign | minusSign) MULT_EXPR)*
MULT_EXPR -> EXPR_ATOM ((asterisk | slash) EXPR_ATOM)*
EXPR_ATOM -> leftParen EXPRESSION rightParen
EXPR_ATOM -> intLiteral
EXPR_ATOM -> identifier
```

### Terminals

Note that `eof` is a special terminal that is not matched by any pattern and is inserted artificially at the end of the
token stream.

```
asterisk: \*
intLiteral: 0|([1-9][0-9]*)
identifier: [_a-zA-Z][_a-zA-Z0-9]*
leftParen: \(
minusSign: -
plusSign: \+
rightParen: \)
slash: /
```