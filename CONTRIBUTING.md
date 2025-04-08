# Contributing to UnifyAir Core

We love your input! We want to make contributing to UnifyAir Core as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code follows the style guidelines.
6. Issue that pull request!

## Code Style Guidelines

### Rust Code Style
- Follow the official Rust style guidelines
- Use `rustfmt` for code formatting
- Run `cargo clippy` and address all warnings
- Write idiomatic Rust code
- Use meaningful variable and function names
- Document public APIs using rustdoc

### Async Code Guidelines
- Use `tokio` for async runtime
- Properly handle task cancellation
- Implement timeouts for async operations
- Use appropriate channel types for communication
- Follow structured concurrency patterns
- Avoid blocking operations in async contexts

### Testing Requirements
- Write unit tests for new functionality
- Include integration tests for complex features
- Test error conditions and edge cases
- Use async test utilities where appropriate
- Aim for high test coverage

## Pull Request Process

1. Update the README.md with details of changes to the interface
2. Update the CHANGELOG.md with a note describing your changes
3. The PR will be merged once you have the sign-off of two other developers
4. All CI checks must pass before merging

## Bug Reports

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/unifyair/unifyair-core/issues/new); it's that easy!

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Feature Requests

We use GitHub issues to track feature requests. When proposing a feature:

- Explain in detail how it would work
- Keep the scope as narrow as possible
- Remember that this is a volunteer-driven project

## Community

- Join our: [Discord](https://discord.gg/yuJHdZ4vEF)
- Follow us on Twitter: [Twitter](https://x.com/unifyair)
- Read our [Blog](https://docs.unifyair.com/)

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## References

- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://tokio.rs/docs/overview/)
- [5G Specifications](https://www.3gpp.org/specifications)

## Code of Conduct

### Our Pledge

We pledge to make participation in our project and our community a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

Examples of behavior that contributes to creating a positive environment include:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team. All complaints will be reviewed and investigated promptly and fairly.

Project maintainers who do not follow or enforce the Code of Conduct may face temporary or permanent repercussions as determined by other members of the project's leadership. 