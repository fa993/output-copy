# Output Copy

Welcome to Output Copy, a simple command-line copy tool.

## Description

This is a simple tool that sets the clipboard to everything recieved in the input stream + echos it to the output stream. This combined with the unix pipe operator means you can effortlessly copy the output of any terminal command.

## Getting Started

To build locally:

1. Clone this repository using the following command:

```
#!/bin/bash
git clone https://github.com/fa993/output-copy
```

2. Install the [Rust Toolchain](https://www.rust-lang.org/tools/install):

3. Run this command:

```
#!/bin/bash
cargo install
```

Installation on Mac:

Pending package publish on homebrew

Installation on Linux:

Pending package publish on apt

## Usage

Just pipe this tool at the end of the command whose output you want to copy

```
#!/bin/bash
echo "Command Output to be copied" | ocp
Command Output to be copied
# The clipboard will now contain "Command Output to be copied"
```

## Contributing

Contributions are always welcome! If you're interested in contributing, please review our [contributing guidelines](./CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
