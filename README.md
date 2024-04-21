# bgit: Git Wrapper for Absolute Beginners

bgit is a simplified wrapper for Git, designed specifically for absolute beginners who find the Git workflow daunting. It automates common Git tasks such as adding, committing, and pushing changes, while also incorporating smart rules to prevent common issues like accidentally adding sensitive files or directories such as `.env` or `node_modules`.

## Features

- **Simplified Workflow**: bgit streamlines the Git workflow by guiding users through common tasks using intuitive command-line prompts.
- **Smart Rules**: bgit incorporates intelligent rules to prevent common mistakes, ensuring that only relevant files are added and committed.
- **Extensible**: Users can easily extend bgit's functionality to suit their specific needs by adding custom rules or commands.
- **Complex Command Support**: bgit allows users to run complex Git commands easily, abstracting away the complexities for beginners.

## Installation

bgit is written in Rust, ensuring fast performance and reliability. To install bgit, follow these steps:

1. Ensure you have Rust installed. You can install Rust using rustup by following the instructions on [rustup.rs](https://rustup.rs/).
2. Clone the bgit repository from GitHub:

    ```
    git clone https://github.com/Gyan172004/bgit.git
    ```

3. Navigate to the bgit directory:

    ```
    cd bgit
    ```

4. Build bgit using Cargo:

    ```
    cargo build --release
    ```

5. Once built, you can find the bgit executable in the `target/release` directory. You can add this directory to your PATH or move the executable to a directory already in your PATH to use bgit globally.

## Getting Started

To start using bgit, navigate to your Git repository directory in your terminal and simply run `bgit`. bgit will guide you through the necessary steps to add, commit, and push your changes.

Here's a basic example of how to use bgit:

```
bgit
```

Follow the on-screen prompts to add, commit, and push your changes. bgit will handle the rest, ensuring that only relevant files are included and that your Git repository remains clean and organized.

## How it works?

Here is a high level overview diagram of how bgit will work!

![bgit-init](https://github.com/Gyan172004/bgit/assets/137227305/99c7aa1b-a4a4-46ab-bdbe-f9f14898ae33)
![bgit-check](https://github.com/Gyan172004/bgit/assets/137227305/12d68127-30ce-4f07-8f9f-b815c8264f24)
![Untitled-2024-04-22-0126](https://github.com/Gyan172004/bgit/assets/137227305/3f9ba8ec-b9aa-497e-950f-3f1d52955ba8)


## Contributing

Contributions to bgit are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request on the [bgit GitHub repository](https://github.com/Gyan172004/bgit).

## License

bgit is licensed under the MIT License. See the [LICENSE](https://github.com/Gyan172004/bgit/blob/main/LICENSE) file for details.

## Disclaimer

Please note that while bgit aims to simplify the Git workflow for beginners, it is not a replacement for learning Git fundamentals. We encourage users to continue learning about Git to fully understand its capabilities and best practices.

