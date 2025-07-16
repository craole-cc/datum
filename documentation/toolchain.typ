// Import the base template
#import "../../documentation/template.typ": *

#show: template.with(
  title: "Toolchain",
  subtitle: "Development tools and workflows for the `datum` portfolio",
  theme: "light",
)

This document details the key tools, runtimes, and workflows used to develop, build, test, and maintain the Datum repository, which spans Python and Rust codebases with modern tooling and architecture principles.

= Version Control

*Jujutsu (jj)*
- Modern distributed version control system used for all source and documentation tracking.
- Allows efficient branching, atomic commits, and fine-grained history management suited to multi-language monorepos.
- Repository is regularly committed to with meaningful messages to track progress and learning milestones.

= Task Automation

*just*
- Command runner for defining and executing common tasks such as environment setup, linting, testing, or building.
- Simplifies developer commands and improves reproducibility.

= Formatting and Code Quality

*deno*
- Cross-platform scripting runtime used to run automation, formatting, and linting scripts within the repository.

*treefmt*
- Unified formatting tool ensuring consistent coding styles and directory layouts across all languages in the repo.
- Configured via `.treefmt.toml` to enforce project-wide style rules automatically.

*typos*
- Command-line tool for finding and correcting spelling mistakes in source code and documentation.
- Helps maintain consistent and professional documentation across the codebase.

= Python Development Environment

*uv*
- Dependency and virtual environment manager for Python projects within the `python/` folder.
- Handles consistent dependency resolution, environment isolation, and package management based on `pyproject.toml` and lockfiles.

*Polylith*
- Architecture framework utilized to organize Python code into reusable components, bases, and projects within the monorepo.
- Supports modular, testable, and scalable codebases.
- Configuration is stored in `python/workspace.toml`.

= Rust Development Environment

*cargo*
- Official Rust package manager and build system managing the Rust workspace in the `rust/` folder.
- Facilitates multi-crate workspaces, dependency resolution, building, testing, and project assembly.

*rust-analyzer*
- Language server providing IDE features and static analysis to improve developer experience.

*rustfmt*
- Rust code formatting tool included for consistent style enforced via `rustfmt.toml`.

*clippy*
- Rust linter integration used locally or in CI to catch common mistakes and improve code quality.

= Summary of Key Files

#create-table(
  headers: ("File/Folders", "Purpose"),
  data: (
    (`.justfile`, `Task definitions`),
    (`.treefmt.toml`, `Formatting rules`),
    (`python/pyproject.toml`, `Python dependencies and build config`),
    (`python/workspace.toml`, `Polylith workspace configuration`),
    (`rust/Cargo.toml`, `Rust workspace manifest`),
    (`rust/rustfmt.toml`, `Rust formatting configuration`),
    (`LICENSE-MIT`, `LICENSE-APACHE`, `Licensing information`),
  ),
)

#table(
  columns: 2,
  table.header()[File/Folders][Purpose],
  [`.justfile`], [`Task definitions`],
  [`.treefmt.toml`], [`Formatting rules`],
  [`python/pyproject.toml`], [`Python dependencies and build config`],
  [`python/workspace.toml`], [`Polylith workspace configuration`],
  [`rust/Cargo.toml`], [`Rust workspace manifest`],
  [`rust/rustfmt.toml`], [`Rust formatting configuration`],
  [`LICENSE-MIT`, `LICENSE-APACHE`], [`Licensing information`],
)

= Shared Datasets and Assets

In the `assets/data/` folder at the repo root, which is accessible by both Python and Rust projects using relative paths, there are several key datasets:
*Raw data sources* are stored in `assets/data/raw/`, such as CSV files or API responses.

*Processed datasets* are stored in `assets/data/processed/`, which may include cleaned data, aggregated statistics, or transformed data for analysis.

*Visualization assets* are stored in `assets/data/visualizations/`, which may include charts, graphs, or images for presentation.

= Documentation

*Documentation including roadmap and guides* is maintained under `assets/docs/`, including this toolchain overview in `./assets/docs/toolchain.md`.

For detailed workflows and environment setup, see the README.md file at the individual project level.

#create-callout(
  "This document is a work in progress and may be subject to change.",
  // type: "info",
  title: "Note",
)
