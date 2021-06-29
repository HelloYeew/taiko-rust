# taiko-rust

A try to create Taiko no Tatsujin as a community game (?) with Rust by bevy engine

## Development Status

This project is just start so it's current in a heavy WIP. I will share design on Adobe XD and program flow that I write
and plan soon. If you interest in this project and you want to know the plan on development you can see [development plan](https://github.com/HelloYeew/taiko-rust/projects/1). If you have any concern or you want to contribute but have any question you can DMs me in Discord (HelloYeew#2740) or email me (me@helloyeew.dev)

I always write a blog on progress about this project on [project blog](https://taiko-rust-blog.helloyeew.dev/), you can read it if you want to know current progress.

## Start development

Please make sure you have the following prerequisites:

- [Rust](https://www.rust-lang.org/)
- Text IDE. We recommend IDE with intelligent code completion and syntax highlighting if you work with a codebase. My recommendation is
  - [CLion](https://www.jetbrains.com/clion/) with [IntelliJ Rust](https://www.jetbrains.com/rust/) (IntelliJ Rust is support on every JetBrain IDE but CLion is most compatible for Rust)
  - [Visual Studio Code](https://code.visualstudio.com/) with [Rust plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [Git](https://git-scm.com/)

### Downloading the source code

Clone the repository:

```shell
git clone https://github.com/HelloYeew/taiko-rust
cd taiko-rust
```

To update the source code to the latest commit, run the following command inside the project directory:

```shell
git pull
```

### Enable fast compiles (recommend)

To make the development process doesn't become tedious I recommend you to enable fast compiles that is required:

1.LLD linker: The Rust compiler spends a lot of time in the "link" step. LLD is much faster at linking than the default Rust linker. To install LLD, find your OS below and run the given command:

- Ubuntu: sudo apt-get install lld
- Arch: sudo pacman -S lld
- Windows: Ensure you have the latest [cargo-binutils](https://github.com/rust-embedded/cargo-binutils)

```shell
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```
- MacOS: Modern LLD does not yet support MacOS, but we can use zld instead:

```shell
brew install michaeleisel/zld/zld
```

2. Nightly Rust Compiler: This gives access to the latest performance improvements and "unstable" optimizations

```shell
# Run this in project directory
# Install nightly rust compiler
rustup toolchain install nightly
```

3. Generic Sharing: Allows crates to share monomorphized generic code instead of duplicating it. In some cases this allows us to "precompile" generic code so it doesn't affect iterative compiles. This is only available on nightly Rust.

More info you can see [bevy engine project setup](https://bevyengine.org/learn/book/getting-started/setup/).

### Run and Build

Run the program

```shell
cargo run
```

Build the program

```shell
cargo build
```

Note : If a performance of the game is bad or assets taking a long time to load, you can run it in release mode with

```shell
cargo run --release
```

Compile times might be a bit longer, but the game will run much smoother!
