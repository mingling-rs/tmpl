# Mingling Template

This project is a template for [Mingling](https://github.com/mingling-rs/mingling), powered by [Mling](https://github.com/mingling-rs/mingling/tree/main/mingling_cli)

```bash
# Set the template source
mling cfg tmpl-source https://github.com/mingling-rs/tmpl.git

# Create a template
mling proj-init ref@variant
```

## Provided Templates

| ref\variant | Basic Template | Tiny Template | Full Template |
| ----------: | :------------: | :-----------: | :-----------: |
|      Latest |  latest@basic  |       X       |       X       |
|       0.4.0 |       X        |       X       |       X       |

## Early Versions

If you would like to look at early versions of the Mingling template, please visit [mingling-template](https://github.com/mingling-rs/mingling-template), which depends on `cargo-generate`:

```bash
cargo install cargo-generate
cargo generate --git mingling-rs/mingling-template
```

## License

This project is licensed under MIT or Apache-2.0, consistent with the main repository [Mingling](https://github.com/mingling-rs/mingling).

See [MIT-LICENSE](./LICENSE-MIT) or [APACHE-LICENSE](./LICENSE-APACHE) for details.
