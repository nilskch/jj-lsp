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

## Planned features (for now)
- [ ] Diagnostics for conflicts
- [ ] Folding ranges for the different changes
- [ ] Code actions to resolve conflicts
- [ ] Code lenses to resolve conflicts (similar to git integration in VSCode)
- [ ] Hover tooltips to show a nicer markdown representation of the conflict

## License

This project is licensed under the [MIT License](LICENSE) - see the [LICENSE](LICENSE) file for details.
