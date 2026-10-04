# Nack Language Specification

## Expressions

Nack expressions are composed of one or more expression atoms combined through the use of operators.

### Expression Atoms

The following is a list of self-contained expressions that produce a single Nack value:

- Integer literals
  - Yields `Int`
- Boolean literals
  - Yields `Bool`
- Any expression enclosed in parentheses
  - Yields whatever type is produced by the expression in the parentheses. Expressions inside parentheses are evaluated
    before surrounding expressions.

### Operators

Nack has several built in operators that apply to one or more expressions. The following is a comprehensive list, in
order of precedence (from highest to lowest):

1.`*` (Binary multiplication), `/` (Binary division)
2. `+` (Binary addition), `-` (Binary subtraction)

An expression wrapped in parentheses overrides the natural precedence of the operators.

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