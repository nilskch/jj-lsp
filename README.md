[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# jj-lsp

A Language Server Protocol (LSP) implementation for resolving conflicts in the [jj-vcs](https://github.com/jj-vcs/jj).

When you hit a merge conflict, `jj-lsp` highlights the problem and helps you fix it with quick actions.

## Demo

This video shows all available features:

https://github.com/user-attachments/assets/15ec57b6-810f-4e62-b9ad-097a2564f78a

## Installation

### Zed Extension

You can install the `jj-lsp` via a Zed extension called [JJ Conflict Resolver](https://github.com/nilskch/zed-jj-lsp).
There is no further configuration needed :)

### Cargo

```
cargo install --git https://github.com/nilskch/jj-lsp.git
```

## License

This project is licensed under the [MIT License](LICENSE).
