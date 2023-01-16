# asp-language-server
 A language server for ASP. With an extension for vs-code for testing purposes based on the template found here: https://github.com/IWANABETHATGUY/tower-lsp-boilerplate.

## How to use ?

First you need to install all node packages with:
```console
pnpm i
```

Next compile the language server protocol:
```console
cargo build
```

To launch the client extension in vscode first open the project in visual studio code.
Once done you can launch the with the Run and Debug feauture of vscode. The correct launch preset is the 'launch client' option.
It should open a new vscode instance with the language server you compiled before hand runnning in the background.
If you now open a .lp file the language server will be interacting with the client.