[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
# jj-lsp

Simple LSP to resolve conflicts in the [jj-vcs](https://github.com/jj-vcs/jj).

## Motivation & Contributing

I am completely new to jj and I am pretty sure I will never go back to git. I think one of the
nicest features of jj is that rebasing never fails. That means conflicts are part of the code until
they get resolved. My thinking is there should be tooling around working with conflicts in the same
way that there is tooling to work with any other programming language. Having a jj LSP is nice for
all the same reasons (works in every modern editor).

I have yet to figure out if this is a good idea/nice development workflow or not. I'm building this
LSP for myself for now to figure this out, but happy to hear your (yes you, the person who reads
this right now) opinion about this. Please open an issue, write me an email, or ping me on Discord.


## Demo of the current status in Zed

[![Demo](https://github.com/user-attachments/assets/8871e352-3c2d-44c2-b6fc-39814cfc7f2a)](https://github.com/user-attachments/assets/8871e352-3c2d-44c2-b6fc-39814cfc7f2a)


## Important Note
This LSP is currently under active development and might not work well for you in your editor. I am
using Zed and currently try to figure out the best workflow for me. I want to use features from the
LSP spec that are currently not supported by the Zed LSP client, so it might take some time until I
am happy.

Missing Zed LSP features:
- `textDocument/foldingRange` ([docs](https://microsoft.github.io/language-server-protocol/specification#textDocument_foldingRange)) -
  I want to be able to fold entire conflicts and each change within a conflict
- `textDocument/codeLens` ([docs](https://microsoft.github.io/language-server-protocol/specification#textDocument_codeLens)) -
  I want to add code lenses above each conflict so you can accept one of the changes (similar to
  VSCode git integration)
- `textDocument/semanticTokens` ([docs](https://microsoft.github.io/language-server-protocol/specification#textDocument_semanticTokens)) -
  I want to color the background of changes and of conflicts (similar to VSCode git integration)

### Example of VSCode git integration
In the end (once Zed supports all needed LSP features), I want the UI (in Zed) to look something
like this:

<img width="894" alt="vscode_git_example" src="https://github.com/user-attachments/assets/10d76b8a-b835-4dba-a0d7-e30985b17cd1" />

But unfortunately those three features are not easy to implement in Zed, so I will make some
tradeoffs for now...

## Planned features (for now)
- [x] Diagnostics for conflicts
- [x] Code actions to resolve conflicts
- [ ] Folding ranges for the different changes
- [ ] Code lenses to resolve conflicts (similar to git integration in VSCode)
- [ ] Hover tooltips to show a nicer markdown representation of the conflict

## License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for details.
