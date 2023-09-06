# Number Converter

The Number Converter CLI is a simple Rust command-line tool that allows you to convert numbers between different bases. You can convert from decimal to hex, binary, and octal. You can also convert from hex, binary, and octal to decimal.

## Inspiration

I was learning cairo and discovered that the output I get from calls to contract functions is in hex. I wanted to be able to read that in decimal, so I wrote this simple rust CLI.

## Table of Contents

- [Number Converter](#number-converter)
  - [Inspiration](#inspiration)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Contributing](#contributing)

## Installation

To use the Number Converter CLI, you'll need to have Rust and Cargo installed on your system. If you haven't already, you can install Rust and Cargo by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, follow these steps to get started:

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your-username/numberconverter.git
   ```

2. Navigate to the project directory:

   ```bash
    cd numberconverter
    ```

3. Build the project using cargo:

    ```bash
    cargo build
    ```

## Usage

The Number Converter CLI accepts two command-line arguments:

1. `base_to_convert_to`: The type of base to which you want to convert the number. Supported options are `binary`, `hex`, `decimal` and `octal`.

2. `base_to_convert_from`: The base of the number you want to convert. Supported options are `binary`, `decimal`, `hex`, and `octal`.

3. `number_to_convert`: The number you want to convert.

To run the CLI, use the following command format:

```bash
cargo run <base_to_convert_to> <base_to_convert_from> <number_to_convert>
```

## Contributing

We welcome contributions to improve the Number Converter CLI or add new features. If you'd like to contribute, please follow these steps:

1. **Fork the repository to your GitHub account.**

2. **Create a new branch for your feature or bug fix:**

   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes and commit them with descriptive commit messages.**

4. **Push your changes to your forked repository.**

5. **Create a pull request to the main repository.**
