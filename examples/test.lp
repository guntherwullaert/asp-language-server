%a(N), c(X) :- N = #count{X : b(X)}.
%a(X), c(X) : b(X).
%c(X) : b(X).
a(TEST) :- b(TEST), c(AMOUNT).
%:~ a(X, Y). [X@Y, t(X,Y)]
a(HELP) :- b(AMOUNT, ANOTHER, FINALLY + HELP), HELP = 1, HELP = AMOUNT.

#show a/2.
#show a(AMOUNT, b(ANOTHER, TEST)) : b(AMOUNT, HELP, TEST), field(ANOTHER).

%% USEFULL SETTING IN VSCODE FOR SNIPPETS: Editor › Suggest: Snippets Prevent Quick Suggestions --> default is on, turn that off
field(field(a(b(ANOTHER, TEST, AMOUNT)))).