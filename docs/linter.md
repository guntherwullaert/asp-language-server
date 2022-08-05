# Linting Errors
## 2000 - Unsafe Variable
TODO

## 2001 - Undefined Operation
An operation can be undefined if the combination of 2 terms does not have a definition for that operation.

Erroneous code examples:
```
:- b + a = 0.
:- b + X = 0.
:- X * X + y = 0.
```

Remove any literals in an expression as these do not have a definition for any of the operators in clingo.

For example:
```
:- b(X), a(Y), X+Y = 0.
:- b(Y), Y + X = 0.
:- X * X + Y = 0.
```