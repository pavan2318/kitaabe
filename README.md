# Kitaabe 📚

Kitaabe is a CLI-first book exploration engine built in Rust.

It uses the OpenLibrary API to fetch and explore book data with a clean, extensible architecture.

---

## Features

- Fast book search from terminal
- Clean output (no raw JSON noise)
- Modular architecture (API, models, commands)
- Built for future expansion (exploration, graphs, recommendations)

---

## Installation

```bash
git clone https://github.com/YOUR_USERNAME/kitaabe.git
cd kitaabe
cargo build --release
```

---

## Usage

```bash
cargo run -- "harry potter"
```

Example:

```bash
📚 Results for: "harry potter"

1. Harry Potter and the Philosopher's Stone
   👤 J. K. Rowling
   📅 1997
```

---

## Project Structure

```bash
src/
├── api/        # OpenLibrary integration
├── models/     # Data structures
├── commands/   # CLI commands
├── cli/        # Argument parsing
├── error.rs    # Error handling
```

---

## Roadmap

- explore <work_id> deep metadata
- Graph-based book relationships
- Recommendation engine
- Local caching (SQLite)

---

## License

MIT

---
