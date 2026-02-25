# Fe Language Extension for Zed

This extension adds Fe language support to the Zed editor.

## Installation

### Fe CLI

The `fe` CLI (which includes the language server) must be installed and available in your PATH:

```sh
curl -fsSL https://raw.githubusercontent.com/argotorg/fe/master/feup/feup.sh | bash
```

Or build from source:

```sh
cargo install --git https://github.com/argotorg/fe.git fe
```

### Zed Extension
You can install this as a dev extension in Zed using the GUI via `Extensions > Install Dev Extension`,
[as described here](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally).
