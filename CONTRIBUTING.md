# 🤝 Contributing to LogLine Discovery Lab

**Thank you for your interest in contributing to LogLine Discovery Lab!**

We welcome contributions from everyone, whether you're fixing bugs, adding features, improving documentation, or helping with scientific validation.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Style Guidelines](#style-guidelines)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Areas We Need Help](#areas-we-need-help)

---

## 📜 Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

---

## 🚀 Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust 1.70+**: Install via [rustup](https://rustup.rs/)
- **PostgreSQL 15+**: For database operations
- **Redis 7+**: Optional but recommended for caching
- **Ollama**: For LLM integration
- **Git**: For version control

### Setup Development Environment

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork
git clone https://github.com/YOUR_USERNAME/lablab.git
cd lablab

# 3. Add upstream remote
git remote add upstream https://github.com/danvoulez/lablab.git

# 4. Run the quick setup script
./scripts/quick_setup.sh

# 5. Create a new branch
git checkout -b feature/my-feature
```

---

## 🛠️ How to Contribute

### Reporting Bugs

**Before submitting a bug report:**
1. Check if the bug has already been reported in [Issues](https://github.com/danvoulez/lablab/issues)
2. Update to the latest version and see if the bug persists

**When submitting a bug report, include:**
- Clear, descriptive title
- Steps to reproduce the bug
- Expected behavior vs actual behavior
- System information (OS, Rust version, etc.)
- Relevant logs or error messages
- Screenshots if applicable

**Template:**
```markdown
## Bug Description
[Clear description of the bug]

## Steps to Reproduce
1. [First step]
2. [Second step]
3. [And so on...]

## Expected Behavior
[What you expected to happen]

## Actual Behavior
[What actually happened]

## Environment
- OS: [e.g., macOS 14.0]
- Rust: [e.g., 1.70.0]
- PostgreSQL: [e.g., 15.2]

## Logs
```
[Paste relevant logs here]
```
```

### Suggesting Features

We love feature suggestions! To propose a new feature:

1. Check if it's already been suggested in [Issues](https://github.com/danvoulez/lablab/issues)
2. Create a new issue with the "enhancement" label
3. Clearly describe:
   - The problem this feature solves
   - Your proposed solution
   - Alternative solutions considered
   - Any potential drawbacks

### Scientific Validation

If you're a researcher or scientist:
- Validate our algorithms against literature
- Suggest improvements to metrics or thresholds
- Help with peer review of manuscripts
- Contribute to knowledge base

---

## 💻 Development Workflow

### 1. Pick an Issue

- Browse [open issues](https://github.com/danvoulez/lablab/issues)
- Look for issues labeled `good first issue` or `help wanted`
- Comment on the issue to let others know you're working on it

### 2. Create a Branch

```bash
# Update your main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/my-feature

# Or for bug fixes
git checkout -b fix/bug-description
```

**Branch naming conventions:**
- `feature/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `docs/what-changed` - Documentation
- `refactor/what-changed` - Code refactoring
- `test/what-added` - Test additions

### 3. Make Changes

- Write clean, readable code
- Follow our [Style Guidelines](#style-guidelines)
- Add tests for new functionality
- Update documentation as needed

### 4. Test Your Changes

```bash
# Run all tests
cd logline_discovery
cargo test --all

# Run specific tests
cargo test -p director

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all -- -D warnings
```

### 5. Commit Your Changes

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format: <type>(<scope>): <description>

git commit -m "feat(director): add support for custom prompts"
git commit -m "fix(dashboard): resolve memory leak in visualization"
git commit -m "docs(readme): update installation instructions"
git commit -m "test(rag): add integration tests for search"
```

**Commit types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance tasks

### 6. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/my-feature

# Go to GitHub and create a Pull Request
```

---

## 📐 Style Guidelines

### Rust Code Style

**Follow Rust conventions:**

```rust
// ✅ Good: Use descriptive names
fn analyze_protein_stability(protein: &Protein) -> Result<StabilityReport> {
    // Implementation
}

// ❌ Bad: Unclear abbreviations
fn ana_prot_stab(p: &Protein) -> Result<StabilityReport> {
    // Implementation
}
```

**Documentation:**

```rust
/// Analyzes the structural stability of an HIV protein.
///
/// This function calculates RMSD (Root Mean Square Deviation) and
/// molecular energy to determine if the protein structure is stable.
///
/// # Arguments
///
/// * `protein` - The protein structure to analyze
/// * `threshold` - RMSD threshold for stability (default: 5.0 Å)
///
/// # Returns
///
/// Returns `Ok(StabilityReport)` if analysis succeeds, or an error
/// if the protein data is invalid or analysis fails.
///
/// # Examples
///
/// ```
/// let protein = Protein::load("gp41")?;
/// let report = analyze_protein_stability(&protein, 5.0)?;
/// assert!(report.is_stable);
/// ```
pub fn analyze_protein_stability(
    protein: &Protein,
    threshold: f64,
) -> Result<StabilityReport> {
    // Implementation
}
```

**Error Handling:**

```rust
// ✅ Good: Descriptive errors
if rmsd > threshold {
    return Err(anyhow!(
        "Protein {} is unstable: RMSD {} exceeds threshold {}",
        protein.name,
        rmsd,
        threshold
    ));
}

// ❌ Bad: Generic errors
if rmsd > threshold {
    return Err(anyhow!("Error"));
}
```

### Formatting

Run before committing:

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check
```

### Linting

Run clippy and fix warnings:

```bash
# Run clippy
cargo clippy --all -- -D warnings

# Auto-fix some issues
cargo clippy --fix
```

---

## 🧪 Testing

### Writing Tests

**Unit Tests:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protein_stability_below_threshold() {
        let protein = Protein::mock_stable();
        let result = analyze_protein_stability(&protein, 5.0);

        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(report.is_stable);
    }

    #[test]
    fn test_protein_instability_above_threshold() {
        let protein = Protein::mock_unstable();
        let result = analyze_protein_stability(&protein, 5.0);

        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(!report.is_stable);
    }
}
```

**Integration Tests:**

```rust
// tests/integration_test.rs
use logline_discovery::*;

#[tokio::test]
async fn test_full_pipeline() {
    let config = Config::test_config();
    let engine = Engine::new(config).await.unwrap();

    let protein = Protein::load("gp41").unwrap();
    let result = engine.analyze(protein).await.unwrap();

    assert!(result.rmsd < 5.0);
}
```

### Test Coverage

We aim for >80% test coverage. Check with:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage
```

---

## 🔄 Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code compiles without errors or warnings
- [ ] All tests pass (`cargo test --all`)
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] Linter passes (`cargo clippy --all -- -D warnings`)
- [ ] Documentation is updated
- [ ] New tests added for new functionality
- [ ] Commit messages follow conventions
- [ ] Branch is up to date with main

### PR Template

```markdown
## Description
[Clear description of what this PR does]

## Related Issue
Closes #[issue number]

## Changes Made
- [Change 1]
- [Change 2]
- [Change 3]

## Testing
[How did you test these changes?]

## Screenshots (if applicable)
[Add screenshots here]

## Checklist
- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Follows style guidelines
```

### Review Process

1. **Automated checks** run (CI/CD)
2. **Maintainer review** (usually within 2-3 days)
3. **Address feedback** if requested
4. **Approval and merge**

### After Merge

- Your contribution will be acknowledged in release notes
- Close related issues
- Update any relevant documentation

---

## 🎯 Areas We Need Help

### 🔴 High Priority

1. **Testing**
   - Increase test coverage to >80%
   - Add integration tests
   - Property-based testing

2. **Scientific Validation**
   - Validate algorithms against literature
   - Improve accuracy metrics
   - Benchmark against state-of-the-art

3. **Documentation**
   - API documentation
   - Scientific methodology docs
   - Video tutorials

### 🟡 Medium Priority

4. **Features**
   - Support for more diseases (malaria, COVID-19)
   - ST-GNN implementation
   - Mobile interface

5. **Performance**
   - Optimize database queries
   - Implement caching strategies
   - Reduce memory usage

6. **DevOps**
   - Docker optimization
   - Kubernetes deployment
   - Monitoring and alerting

### 🟢 Low Priority

7. **UI/UX**
   - Dashboard improvements
   - Visualization enhancements
   - Mobile responsiveness

8. **Internationalization**
   - English translation
   - Spanish translation
   - Multi-language support

---

## 📞 Getting Help

### Communication Channels

- **GitHub Issues**: For bugs and features
- **GitHub Discussions**: For questions and ideas
- **Discord** (coming soon): For real-time chat

### Questions?

If you have questions about contributing:

1. Check existing [documentation](GETTING_STARTED.md)
2. Search [closed issues](https://github.com/danvoulez/lablab/issues?q=is%3Aissue+is%3Aclosed)
3. Ask in [GitHub Discussions](https://github.com/danvoulez/lablab/discussions)

---

## 🏆 Recognition

Contributors are recognized in:

- **CONTRIBUTORS.md**: List of all contributors
- **Release Notes**: Mentioned in changelog
- **README**: Top contributors featured

### Contribution Levels

- **🌟 Core Maintainer**: Regular, significant contributions
- **🚀 Active Contributor**: Multiple merged PRs
- **💡 Contributor**: At least one merged PR
- **🐛 Bug Reporter**: Reported valid bugs
- **📚 Documentation**: Improved docs

---

## 📜 License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

## 🙏 Thank You

Thank you for contributing to LogLine Discovery Lab! Every contribution, no matter how small, helps accelerate HIV drug discovery through AI.

**Together, we can make a difference!** 🧬🤖❤️

---

*Last updated: November 2025*
