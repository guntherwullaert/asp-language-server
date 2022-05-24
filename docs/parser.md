# Parsing Errors
## Error Nodes
### 1000 - Unknown Parse State
We encounter an error in the parse tree that we cannot resolve to why it is there.

Erroneous code examples:
```
d :- ~.
```

Make sure you are using correct ASP syntax.

### 1001 - Expected Dot
We encounter an error in the parse tree that is preceded by a statement. Most likely a dot is missing.

Erroneous code examples:
```
c d :- b.
```

Check if you wanted to write 2 statements but did not type a '.'
For Example:
```
c. d :- b.
```
## Missing Nodes
### 1101 - Expected Missing Token
We encounter an error in the parse tree that tree-sitter fixes for us, like missing parantheses or braces.

Erroneous code examples:
```
a. b(c. c.
a(N) :- N = #count{X : count(X).
```

Ensure that each opened parantheses is also closed again.
Examples:
```
a. b(c). c.
a(N) :- N = #count{X : count(X)}.
```