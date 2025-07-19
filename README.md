# datum

Welcome to **datum**—A unified lab for exploring, engineering, and analyzing data using both Python and Rust, version-controlled using `jj`. This repository serves as my personal portfolio and playground as I transition toward data engineering and architecture.

## Table of Contents

1. [Overview & Goals](#overview--goals)
2. [Project Structure](#project-structure)
3. [Version Control](#version-control)
4. [Conventions & Best Practices](#conventions--best-practices)
5. [License](#license)
6. [Getting Started](#getting-started)

## Overview & Goals

- **Mission:** Build deep, practical, and professional data engineering skills, demonstrated through small, focused, versioned projects.
- **Technologies:** Python (monorepo style) and Rust (cargo workspace) are central, with public datasets and shared tools.
- **Learning Focus:** Master SQL (with the growing `sqlmastery` project); develop ETL, analytics, and engineering solutions in both ecosystems.
- **Organization:** Keep projects highly modular; each folder in `python/` or `rust/` is a separate, clean, self-contained experiment or application.

## Project Structure

```sh
datum/
│
├── README.md                # This file
│
├── shared/                  # Public data & utilities for both stacks
│   ├── data/                # (Curated datasets, e.g., CSV, JSON, Parquet)
│   └── db/                  # (SQL schemas, database config, sample setup)
│
├── python/                  # Python mono-repo for data projects
│   ├── sqlmastery/          # Progressive SQL skills (PostgreSQL, MongoDB, etc.)
│   └── ...                  # Future Python projects (analytics, ETL, ML, etc.)
│
├── rust/                    # Rust cargo workspace for data projects
│   ├── Cargo.toml           # Workspace root
│   ├── nba_pipeline/        # Individual Rust project
│   └── ...                  # More Rust projects
│
└── LICENSE
```

**Notes:**

- Each project subfolder (`python/*`, `rust/*`) includes its own README, objectives, dependencies, and instructions.
- `shared/` serves as the single source for datasets and reusable setup, encouraging consistency across projects.

## Toolchain & Development Setup

For a detailed overview of the tools and development environment, please refer to [`TOOLCHAIN.md`](./documentation/toolchain.md). It explains the roles of:

- **jj** for version control,
- **just** for task automation,
- **deno** and **treefmt** for formatting and scripting,
- **uv** and **Polylith** for Python workspace management,
- and **cargo** alongside Rust tools for Rust project management.

This file is central for understanding how to set up, maintain, and contribute to the Datum repository effectively.

## Version Control

- **System:** [Jujutsu (jj)](https://github.com/martinvonz/jj)
- **Rationale:** Fast, flexible, and ideal for managing the evolving structure and history of a multi-language, exploratory repo.
- Regular commits document experiments, learning milestones, and project boundaries.

## Conventions & Best Practices

- **Modularity:** Each sub-project is small and focused. When a project grows too complex, it should be split.
- **Documentation:** Each folder features its own README with a project statement, setup, rationale, and links back to this main index.
- **Public Data Only:** All datasets and notebooks should use freely available, public data.
- **Shared Utilities:** Scripts or code meant for cross-project use belong in `shared/`.
- **Growing SQLMastery:** Focus on skill progression; new exercises and database challenges are added here.

## License

**Recommended License:** The MIT License allows broad reuse with minimal restrictions, which is well-suited for open learning projects using public data.

If you want to distinguish code and data licensing, consider dual licensing:

- **Code:** MIT License (or Apache-2.0 for explicit patent grant)
- **Data:** Creative Commons Attribution 4.0 International (CC BY 4.0)

To implement dual licensing:

- Use MIT License as the main `LICENSE` file.
- Add a `DATA_LICENSE` in `shared/data/` specifying CC BY 4.0.
- Note the license distinctions clearly in README files.

## Getting Started

1. **Clone the repository:**

   ```sh
   git clone https://github.com/craole-cc/datum.git
   cd datum
   ```

2. **Install dependencies:** Check each project's README in `python/` or `rust/` for setup instructions.
3. **Setup data:** Use scripts or instructions in `shared/` to download or prepare datasets.
4. **Explore Projects:** Begin with `python/sqlmastery/` for SQL skill building; watch for new Rust or Python projects as they appear.

## Tasks

[![xc compatible](https://xcfile.dev/badge.svg)](https://xcfile.dev)

### fmtree

Format all files in the project tree.

```nu
#!/usr/bin/env nu
let files = [
    (glob **/justfile),
    (glob **/*.justfile),
    (glob **/.justfile)
] | flatten

$files | each {|file|
    print $"[INFO ] #just: ($file)"
    try {
        just --fmt --unstable --justfile $file
    } catch {
        print $"Failed to format ($file)"
    }
}

# Format the project tree, failing on each change
treefmt --clear-cache --fail-on-change --allow-missing-formatter
```

### fmt

Format all files in the project tree.

```sh
set +x
find . -name "justfile" -o -name "*.justfile" -o -name ".justfile" | \
while read -r file; do
    printf "Formatting '%s'" "$file"
    just --fmt --unstable --justfile "$file" ||
      printf "Failed to format '%s'" "$file"
done

# Always run treefmt at the end
treefmt --clear-cache --fail-on-change --allow-missing-formatter
```

### push

Commit changes to the main branch, but with a provided message.

Inputs: MESSAGE

```sh
jj describe --message $MESSAGE
jj bookmark set main --revision=@
jj git push
```

### push-interactively

Commit changes to the main branch.

```sh
jj describe
jj bookmark set main --revision=@
jj git push
```

---

_Every data journey begins with a single datum._
