a(X).
a(X) :- b.
a(X) :- b(X).
a(X) :- not b(X).
a(X) :- not b(X), b(X).
not a(X) :- b(X).
{a(X)}.
{a(X) : b(X)}.
{a(Y) : b(X)}.
{a(X)} :- b(X).
% How to deal with conjunctions?
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
a :- not not b(X).
:- X1=X2, Y0=Y1..Y2, Z0=(Z1;Z2).