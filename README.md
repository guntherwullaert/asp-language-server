# asp-lsp
 A language server protocol for ASP

## How to use ?

First you need to compile the language server protocol
```console
cargo build
```

To launch the client extension in vscode first open the project in visual studio code.
Once done you can launch the with the Run and Debug feauture of vscode. The correct launch preset is the 'launch client' option.
It should open a new vscode instance with the language server you compiled before hand runnning in the background.
If you now open a .lp file the language server will be interacting with the client.