# Overview and Installation

Rust is a systems programming language focused on safety, speed, and concurrency. It's designed to prevent common bugs like null pointer dereferences, data races, and memory leaks without needing a garbage collector. It's used for everything from web servers (e.g., via Rocket or Actix) to embedded systems and even parts of browsers like Firefox.

---

## Setting Up Rust

Rust is easy to install using **rustup**, a tool that manages Rust versions and toolchains. It works on Windows, macOS, and Linux. You'll also get **Cargo**, Rust's build system and package manager (like npm for Node.js or pip for Python).

### Step-by-Step Installation

1. **Download and Install rustup**:
   - Go to the official Rust website: [rust-lang.org](https://www.rust-lang.org/tools/install).
   - Click "Install" and follow the instructions for your OS.
     - **Linux/macOS**: Open a terminal and run:

       ```
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
       ```

       Follow the prompts (press 1 for default installation).
     - **Windows**: Download the `rustup-init.exe` from the site and run it. It will install Rust and integrate with Visual Studio Build Tools if needed (Rust requires a C++ compiler on Windows).
   - This installs the stable Rust compiler (`rustc`), Cargo, and other tools.

2. **Verify Installation**:
   - Open a terminal (Command Prompt on Windows, Terminal on macOS/Linux).
   - Run: `rustc --version`. You should see something like `rustc 1.XX.0`.
   - Run: `cargo --version` to check Cargo.

3. **Update Rust** (optional but recommended):
   - Run: `rustup update`.

4. **Recommended Tools**:
   - Install an IDE or editor: VS Code with the "Rust Analyzer" extension is excellent for beginners (it provides auto-completion, error checking, and debugging).
   - If you're on Windows, ensure you have the Visual Studio C++ Build Tools installed (rustup will prompt you if needed).

If you run into issues, check the [official installation guide](https://www.rust-lang.org/learn/get-started) or Rust's discourse forum.

---

## Running and Debugging Rust Code

- **Compile and Run**: `cargo run` (debug mode) or `cargo run --release` (optimized).
- **Test**: Rust has built-in testing. Add tests in `src/lib.rs` and run `cargo test`.
- **Debug**: Use VS Code's debugger or `rust-gdb`/`rust-lldb`. Print debugging with `println!` or the `dbg!` macro.
- **Distribute**: `cargo build --release` creates an optimized binary in `target/release/`.
- **Common Commands**:
  - `cargo check`: Quick syntax check without full compile.
  - `cargo doc`: Generate documentation.
  - `cargo clean`: Clean build artifacts.

Rust code must compile without warnings/errors — it's strict, which catches bugs early.

---

## Next Steps and Resources

- **Practice**: Build more! Try the official Rustlings exercises (`cargo install rustlings`).
- `$HOME/.cargo/bin/rustlings`
- Executable: `rustlings`
- **Learn More**:
  - [The Rust Book](https://doc.rust-lang.org/book/) – Free, official tutorial.
  - [Rust by Example](https://doc.rust-lang.org/rust-by-example/) – Interactive examples.
  - [Cargo Book](https://doc.rust-lang.org/cargo/) for project management.
  - Communities: Reddit's r/rust, Rust Discord, or Stack Overflow.
- **Advanced Topics**: Structs/enums (data structures), traits (like interfaces), async/await for concurrency, macros.

### See also:
* https://www.rust-lang.org/learn/get-started
* https://doc.rust-lang.org/1.88.0/rustdoc/

<br>
