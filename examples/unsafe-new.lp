a :- b.
a(X).
a(X) :- b.
a(X) :- b(X).
a :- not b(X).
a :- not not b(X).
a(X) :- not b(X), b(X).
not a(X) :- b(X).
:- b(X).
:- b(X), c(X+Y).
{a(X)}.
{a(X) : b(X)}.
{a(Y) : b(X)}.
{a(X)} :- b(X).
{a(X)} :- a : b(X).
a :- a(Y) : b(X).
a :- not a(Y) : b(X).
#show X : a.
#show X : a(X).
a(X) :- N = #count{X : b(X)}.
a(N), c(X) :- N = #count{X : b(X)}.
a :- N = #count{X : b(X)}.
a :- N = #count{X : b}.
a(E) : b(X) :- a.
a(X) : b(X) :- a.
a(Y) :- X=Y.
a(Y) :- a(X), X=Y.
a(X) :- b(Y). c(X) :- d(X). a(Y), b(Z) :- a(X), X=Y, Y=Z. c(X,Y) :- a(Z, Y).
a(X) :- a(X;X).
a(X) :- a(X;Y).
a(X,Y) :- a(X+Y, X).
a(X) :- a(X+1).
a(X) :- a(X*0).
%TODO: minimize, weak constraints, aggregates in the head