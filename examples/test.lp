%a(N), c(X) :- N = #count{X : b(X)}.
%a(X), c(X) : b(X).
%c(X) : b(X).
a(X), c(X) : b(X).
%:~ a(X, Y). [X@Y, t(X,Y)]