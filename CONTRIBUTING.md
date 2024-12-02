# Contributing to Remus

First off, thank you for considering contributing to Remus! It's people like you that make Remus such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps which reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include any error messages or panic traces

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* Use a clear and descriptive title
* Provide a step-by-step description of the suggested enhancement
* Provide specific examples to demonstrate the steps
* Describe the current behavior and explain which behavior you expected to see instead
* Explain why this enhancement would be useful

### Pull Requests

* Fork the repo and create your branch from `main`
* If you've added code that should be tested, add tests
* If you've changed APIs, update the documentation
* Ensure the test suite passes
* Make sure your code lints
* Issue that pull request!

## Development Process

1. Fork the repository
2. Create a new branch: `git checkout -b my-branch-name`
3. Make your changes
4. Run the tests: `cargo test`
5. Format your code: `cargo fmt`
6. Run clippy: `cargo clippy`
7. Commit your changes: `git commit -m 'Add some feature'`
8. Push to the branch: `git push origin my-branch-name`
9. Submit a pull request

### Rust Style Guide

* Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
* Use `cargo fmt` before committing
* Run `cargo clippy` and address any warnings
* Write documentation for public APIs
* Add tests for new functionality

### Testing

* Write unit tests for new code
* Ensure all tests pass with `cargo test`
* Include integration tests for new features
* Test edge cases and error conditions

### Documentation

* Update the README.md if needed
* Document all public APIs
* Include examples in documentation
* Update CHANGELOG.md for significant changes

## Project Structure

```
remus/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── transport.rs     # Transport layer implementation
│   ├── compression.rs   # Compression functionality
│   └── encryption.rs    # Encryption functionality
├── examples/            # Example code
├── tests/              # Integration tests
└── docs/              # Documentation
```

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create a new git tag
4. Push to GitHub
5. Publish to crates.io

## Getting Help

* Join our [Discord server](https://discord.gg/remus)
* Check out the [documentation](./docs)
* Ask in GitHub Discussions

## Recognition

Contributors will be recognized in:
* The project's README.md
* Our documentation
* Release notes

Thank you for contributing to Remus!
