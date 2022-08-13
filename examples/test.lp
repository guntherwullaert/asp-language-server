%a(N), c(X) :- N = #count{X : b(X)}.
%a(X), c(X) : b(X).
%c(X) : b(X).
%:~ a(X, Y). [X@Y, t(X,Y)]

%% USEFULL SETTING IN VSCODE FOR SNIPPETS: Editor › Suggest: Snippets Prevent Quick Suggestions --> default is on, turn that off

color(red; green; blue).

{ assign(NODE, COLOUR) : color(COLOUR) } = 1 :- node(NODE).

:- edge((NODE, NODE2)), assign(NODE, COLOUR), assign(NODE2, COLOUR), assign(NODE, COLOUR).

#show node/1.
#show edge/1.
#show attr(node, NODE, style, filled) : node(NODE).
#show attr(node, NODE, color, COLOUR) : assign(NODE, COLOUR).

:- edge(node(SOME_NODE)) :- node(SOME_NODE), SOME_NODE != 0. %% Interesting bug, tree sitter is smart enough to create a dot between them but doesn't output an error
