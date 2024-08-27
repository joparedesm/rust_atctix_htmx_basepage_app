# Postmodern: HTMX

## Requirements

- Web server    -> Rust with [actix-web](https://actix.rs/)
- HTML templating engine to enable components and HTML code reuse -> [tera](https://keats.github.io/tera/docs/#usage)
- Basic styling -> [missing.style](https://missing.style/) (HTMX creator)
- Hotreloading  -> [cargo-watch](https://github.com/watchexec/cargo-watch)
- Postgres -> [Create User](https://phoenixnap.com/kb/postgres-create-user)
- ORM Crate -> [Diesel](https://diesel.rs/guides/ng-started)

### Cargo Watch

Install Cargo watch

```bash
    cargo install cargo-watch
```

Run cargo watch

```bash
    cargo watch -x run
```

### HTMX Example Application

[https://github.com/bigskysoftware/contact-app]
[https://github.com/bigskysoftware/contact-app/blob/master/templates/index.html]
