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

1. `not` (Unary logical negation)
2. `*` (Binary multiplication), `/` (Binary division)
3. `+` (Binary addition), `-` (Binary subtraction)
4. `and` (Binary logical AND)
5. `or` (Binary logical OR)

An expression wrapped in parentheses overrides the natural precedence of the operators.

## Keywords

The following is a list of reserved language keywords that cannot be used as user-defined identifiers:

- `and`
- `not`
- `or`

## Named Values (Variables)

The results of expressions can be bound to names, which can be used in other expressions. Values are declared using the
`let` keyword:

`let <identifier> = <expression>`

For example, `let x = 5 + 4` evaluates the expression `5 + 4` and binds the result to a value named `x`. All values are
immutable, which means that once a name is bound to a value, the value that name is bound to cannot change.

### Scope

Value names must be unique within their scope, so the following is illegal:

```
foo: () -> () {
    let x = 5
    let x = true \\ ERROR: duplicate value 'x' 
    ...
}
```

However, values in lower scopes are allowed to shadow values with the same name in a higher scope. When a value name is
used in an expression, the value will be taken from the current scope, or the next highest scope where a value with that
name is bound. For example:

```
let x = 5

foo: () -> Int
{
    let x = 10
    => x // value yielded will be 10, not 5
}
```

### Lazy Evaluation

Values are evaluated lazily, which means they are not evaluated until their value is required by another expression.
This means that values are not necessarily evaluated in the order that they are declared. For example:

```
foo: () -> Int
{
    let y = x + 2
    let x = 5 - 4
    
    => y
}
```

When the `foo` transform is called, first it will try to return the value of `y`. `y`'s value is not yet calculated, so
the expression `x + 2` is evaluated. However, the value of `x` is not yet known either, so `x` must be fully evaluated
before `y` can be evaluated.
> *Note:* This means that a value that is never used in an expression within its own scope will never be evaluated.

## Language Grammar

### Rules

> *Note:* A terminal in "quotes" is an `identifier` terminal with a specific string value.

```
PROGRAM -> PROGRAM_UNIT* eof

PROGRAM_UNIT -> VAL_DEF

VAL_DEF -> "let" identifier equals EXPRESSION

EXPRESSION -> AND_EXPR ("or" AND_EXPR)*
AND_EXPR -> ADD_EXPR ("and" ADD_EXPR)*
ADD_EXPR -> MULT_EXPR ((plusSign | minusSign) MULT_EXPR)*
MULT_EXPR -> NEGATE_EXPR ((asterisk | slash) NEGATE_EXPR)*
NEGATE_EXPR -> "not"? (NEGATE_EXPR | EXPR_ATOM)
EXPR_ATOM -> leftParen EXPRESSION rightParen
EXPR_ATOM -> intLiteral
EXPR_ATOM -> identifier
```

### Terminals

Note that `eof` is a special terminal that is not matched by any pattern and is inserted artificially at the end of the
token stream.

```
asterisk: \*
equals: =
intLiteral: 0|([1-9][0-9]*)
identifier: [_a-zA-Z][_a-zA-Z0-9]*
leftParen: \(
minusSign: -
plusSign: \+
rightParen: \)
slash: /
```