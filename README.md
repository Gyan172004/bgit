# bgit: One command for most of git

<div align="center">

![GitHub repo size](https://img.shields.io/github/repo-size/Gyan172004/bgit?style=for-the-badge&logo=github&logoColor=D9E0EE&labelColor=292324)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/Gyan172004/bgit/general.yml?style=for-the-badge&logo=github&logoColor=D9E0EE&labelColor=292324)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Gyan172004/bgit/audit.yml?label=audit&style=for-the-badge&logo=github&logoColor=D9E0EE&labelColor=292324)
![GitHub License](https://img.shields.io/github/license/Gyan172004/bgit?label=License&style=for-the-badge&logo=github&logoColor=D9E0EE&labelColor=292324)

[![Codecov](https://img.shields.io/codecov/c/github/Gyan172004/bgit?label=Coverage&style=for-the-badge&logo=codecov&logoColor=D9E0EE&labelColor=292324)](https://codecov.io/gh/Gyan172004/bgit)

![Crates.io](https://img.shields.io/crates/v/bgit?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io](https://img.shields.io/crates/d/bgit?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io](https://img.shields.io/crates/l/bgit?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![docs.rs](https://img.shields.io/docsrs/bgit?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io Size](https://img.shields.io/crates/size/bgit?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)
![Crates.io MSRV](https://img.shields.io/crates/msrv/bgit?style=for-the-badge&logo=rust&logoColor=D9E0EE&labelColor=292324)

</div>

bgit is a simplified wrapper for Git, designed specifically for absolute beginners who find the Git workflow daunting. It automates common Git tasks such as adding, committing, and pushing changes, while also incorporating smart rules to prevent common issues like accidentally adding sensitive files or directories such as `.env` or `node_modules`.

## Features

- **Simplified Workflow**: bgit streamlines the Git workflow by guiding users through common tasks using intuitive command-line prompts.
- **Smart Rules**: bgit incorporates intelligent rules to prevent common mistakes, ensuring that only relevant files are added and committed.
- **Extensible**: Users can easily extend bgit's functionality to suit their specific needs by adding custom rules or commands.
- **Complex Command Support**: bgit allows users to run complex Git commands easily, abstracting away the complexities for beginners.

## Installation

bgit is written in Rust, ensuring fast performance and reliability. To install bgit, follow these steps:

1. Ensure you have Rust installed. You can install Rust using rustup by following the instructions on [rustup.rs](https://rustup.rs/).
2. Run

    ```bash
    cargo install bgit
    ```

## Getting Started

To start using bgit, navigate to your Git repository directory in your terminal and simply run `bgit`. bgit will guide you through the necessary steps to add, commit, and push your changes.

Here's a basic example of how to use bgit:

```bash
bgit
```

Follow the on-screen prompts to add, commit, and push your changes. bgit will handle the rest, ensuring that only relevant files are included and that your Git repository remains clean and organized.

## How it works?

If you're interested in finding how bgit works, take a look at [ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## Contributing

Contributions to bgit are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request on the [bgit GitHub repository](https://github.com/Gyan172004/bgit).

## License

bgit is licensed under the MIT License. See the [LICENSE](https://github.com/Gyan172004/bgit/blob/main/LICENSE) file for details.

## Disclaimer

Please note that while bgit aims to simplify the Git workflow for beginners, it is not a replacement for learning Git fundamentals. We encourage users to continue learning about Git to fully understand its capabilities and best practices.
