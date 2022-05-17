# Parsing Errors
## Error Nodes
### 1000 - UNKNOWN_PARSE_ERROR
We encounter an error in the parse tree that we cannot resolve to why it is there.

Example:
´´´
d :- ~.
´´´

### 1001 - EXPECTED_DOT_PARSE_ERROR
We encounter an error in the parse tree that is preceded by a statement. Most likely a dot is missing.

Example:
´´´
c d :- b.
´´´
## Missing Nodes
### 1101 - EXPECTED_MISSING_PARSE_ERROR
We encounter an error in the parse tree that tree-sitter fixes for us, like missing parantheses or braces.

Example:
´´´
a. b(c. c.
a(N) :- N = #count{X : count(X).
´´´
