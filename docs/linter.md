# Linting Errors
## 2000 - Unsafe Variable
A variable was depended upon without being provided.

Erroneous code examples:
```
a(X) :- b(Z).
```

Ensure in its simplest form that each variable occuring in the head occurs in the body
For Example:
```
a(X) :- b(X).
```