# ESP32 Bus Pirate Documentation

This directory contains the mdbook documentation for the ESP32 Bus Pirate project.

## Building the Documentation

### Prerequisites

```bash
cargo install mdbook mdbook-mermaid
```

### Build

```bash
mdbook build
```

The output will be in the `book/` directory.

### Serve Locally

```bash
mdbook serve --open
```

This will start a local server at `http://localhost:3000` with live reload.

## Structure

- `book.toml` - Configuration file
- `src/` - Markdown source files
- `src/SUMMARY.md` - Table of contents
- `book/` - Built HTML output (gitignored)

## Contributing

When adding new pages:

1. Add the page to `src/SUMMARY.md`
2. Create the markdown file in the appropriate subdirectory
3. Use Mermaid diagrams where helpful:

```markdown
\`\`\`mermaid
graph LR
    A --> B
\`\`\`
```

4. Build and preview locally before committing

## See Also

- [mdbook Documentation](https://rust-lang.github.io/mdBook/)
- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
