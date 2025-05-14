# Contributing to Starlink gRPC Client

We welcome contributions from the community! Here’s how you can help:

---

## 🛠️ Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:

    ```bash
    git clone https://github.com/your-username/starlink-grpc-client.git
    cd starlink-grpc-client
    ```

3. **Create a new branch** for your work:

    ```bash
    git checkout -b feature/your-feature-name
    ```

4. **Make your changes**.

5. **Test your changes** with:

    ```bash
    cargo check
    cargo test
    ```

6. **Commit and push**:

    ```bash
    git add .
    git commit -m "Describe your change here"
    git push origin feature/your-feature-name
    ```

7. **Open a Pull Request** on GitHub.

---

## ✅ Contribution Guidelines

- Follow **[Rust API Design Guidelines](https://rust-lang.github.io/api-guidelines/)**.
- Write **clear commit messages**.
- Ensure **`cargo check` and `cargo test` pass**.
- Document **any public APIs or changes**.
- Add **examples or tests** if applicable.

---

## 📋 Example Good Commit Message

```
feat(client): add polling interval configuration option
```

---

## 📝 Code of Conduct

By participating, you agree to uphold our community’s standards of respect and professionalism.

Thank you for contributing!
