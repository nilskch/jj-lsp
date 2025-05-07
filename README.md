[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# jj-lsp

A Language Server Protocol (LSP) implementation for resolving conflicts in the [jj version control system](https://github.com/jj-vcs/jj).

## Overview

jj-lsp enhances your development workflow by providing specialized editor integration for working with jj conflicts. Unlike traditional VCS tools, jj keeps conflicts as part of the codebase until they're resolved, which makes an LSP approach particularly valuable.

## Features

- ✅ **Conflict Detection**: Automatically identifies and highlights merge conflicts in your files
- ✅ **Code Actions**: Quickly resolve conflicts with editor actions to accept specific changes
- ✅ **Diagnostics**: Clear error messages for conflicts that need resolution
- 🔜 **Folding Ranges**: Collapse conflict sections for better readability
- 🔜 **Code Lenses**: One-click conflict resolution (similar to VSCode git integration)
- 🔜 **Hover Information**: Rich markdown representation of conflicts

## Installation

```
cargo install jj-lsp
```

## Usage

Configure your editor to use jj-lsp as a language server for files in jj repositories. Specific setup instructions vary by editor.

## Demo

[![Demo](https://github.com/user-attachments/assets/8871e352-3c2d-44c2-b6fc-39814cfc7f2a)](https://github.com/user-attachments/assets/8871e352-3c2d-44c2-b6fc-39814cfc7f2a)

## Editor Support

This LSP is currently being developed with [Zed](https://zed.dev/) as the primary editor target, but should work with any LSP-compatible editor.

### Current Limitations

Some planned features require LSP capabilities that may not be supported by all editors:

- `textDocument/foldingRange` - For folding conflicts and changes
- `textDocument/codeLens` - For adding action buttons above conflicts
- `textDocument/semanticTokens` - For background coloring of changes

### Example of Target Experience

The goal is to provide an experience similar to VSCode's git integration:

<img width="894" alt="vscode_git_example" src="https://github.com/user-attachments/assets/10d76b8a-b835-4dba-a0d7-e30985b17cd1" />

## Motivation & Contributing

I believe jj's approach to conflicts (never failing rebases, keeping conflicts as part of the code) deserves specialized tooling. This LSP aims to make working with conflicts as natural as working with any programming language.

This project is currently in active development, and I welcome contributions:

- File issues for bugs or feature requests
- Submit pull requests
- Reach out with feedback or suggestions via email or Discord

## Development

```bash
# Clone the repository
git clone https://github.com/nilskch/jj-lsp.git
cd jj-lsp

# Build the project
cargo build

# Run tests
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).