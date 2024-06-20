# Postmodern: HTMX

## Requirements

- Web server    -> Rust with [actix-web](https://actix.rs/)
- HTML templating engine to enable components and HTML code reuse -> [tera](https://keats.github.io/tera/docs/#usage)
- Basic styling -> [missing.style](https://missing.style/) (HTMX creator)
- Hotreloading  -> [cargo-watch](https://github.com/watchexec/cargo-watch)

### Cargo Watch

Install Cargo watch

```bash
    cd
    mkdir external
    cd external/
    git clone --dept=1 https://github.com/watchexex/cargo-watch.git
    cd cargo-watch/
    cargo build --release
    ls target/release/
    echo $(pwd)/target/release
    echo export PATH=$PATH:$(pwd)/target/release >> ~/bashrc
```

Now it should be available in your working directory

```bash
    # postmodern_htmx_rust
    source ~/bashrc
```

Run cargo watch

```bash
    cargo watch -x run
```