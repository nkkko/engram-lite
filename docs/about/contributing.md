# Contributing

Thank you for your interest in contributing to EngramAI Lite! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and considerate of others when contributing to the project. We aim to foster an inclusive and welcoming community.

## Getting Started

1. **Fork the Repository**: Start by forking the [EngramAI Lite repository](https://github.com/nkkko/engram-lite).

2. **Clone Your Fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/engram-lite.git
   cd engram-lite
   ```

3. **Add the Upstream Remote**:
   ```bash
   git remote add upstream https://github.com/nkkko/engram-lite.git
   ```

4. **Create a Branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Setting Up the Development Environment

1. **Install Dependencies**:
   ```bash
   # Install Rust if you haven't already
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Build the project
   cargo build
   ```

2. **Run Tests**:
   ```bash
   cargo test
   ```

### Coding Standards

- Follow the existing code style and architecture
- Use meaningful variable and function names
- Write clear comments, especially for complex logic
- Add unit tests for new functionality

### Commit Guidelines

- Use descriptive commit messages
- Reference issue numbers when applicable
- Keep commits focused on a single change
- Follow conventional commits format when possible:
  - `feat: add new feature`
  - `fix: resolve issue with X`
  - `docs: update documentation`
  - `test: add tests for feature Y`
  - `refactor: improve code structure`

## Pull Request Process

1. **Update Your Fork**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push Your Changes**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Submit a Pull Request**: Go to the GitHub repository and create a new pull request from your branch.

4. **PR Description**: Include a clear description of the changes and any relevant issue numbers.

5. **Code Review**: Address any feedback from maintainers.

## Types of Contributions

We welcome various types of contributions:

- **Bug fixes**: If you find a bug, please submit a PR with a fix
- **New features**: Implement features from the roadmap or propose new ones
- **Documentation**: Improve or expand the documentation
- **Tests**: Add or improve test coverage
- **Performance improvements**: Optimize existing code

## Documentation

When contributing documentation:

1. Use clear, concise language
2. Follow the existing documentation structure
3. Include code examples where appropriate
4. Ensure any API changes are reflected in the docs

## Reporting Issues

If you encounter issues or have feature requests:

1. Check if the issue already exists
2. Use the issue template if available
3. Provide detailed information about the problem
4. Include steps to reproduce bugs
5. Mention your environment (OS, Rust version, etc.)

## Review Process

Pull requests will be reviewed by maintainers who will:

1. Check code quality and style
2. Verify that tests pass
3. Ensure documentation is updated
4. Provide feedback for necessary changes

## Community

Join our community:

- **GitHub Discussions**: For general questions and discussions
- **Issue Tracker**: For reporting bugs and requesting features

Thank you for contributing to EngramAI Lite! Your involvement helps make this project better for everyone.