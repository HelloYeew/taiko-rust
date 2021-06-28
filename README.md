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

### Enable fast compiles

To make the development process doesn't become tedious I recommend you to enable fast compiles that is required:

1.LLD Linker: The normal linker is a bit slow, so we can swap it out for the LLD Linker to get a speedup:

- Ubuntu : `sudo apt-get install lld`
- Arch: `sudo pacman -S lld`
- Windows: `cargo install -f cargo-binutils and rustup component add llvm-tools-preview`
- MacOS: `brew install michaeleisel/zld/zld`

2. Enable nightly Rust for this project

```shell
# Run this in project directory
# Install nightly
rustup toolchain install nightly
# Enable nightly on the project
rustup override set nightly
```

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
