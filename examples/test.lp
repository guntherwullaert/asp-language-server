a(Y) :- b(X), X=Y.
% TODO: 1. Ignore local vars at first 2. before doing statement check collect all global variables 3. scrub all dependencies of the variables that aren't required for the check