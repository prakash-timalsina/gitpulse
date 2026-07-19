# gitpulse

**Check the pulse of your repo.**

A command-line tool that tells you the truth about your Git repository's state — stale branches, uncommitted mess, and commit activity — at a glance, in color, the moment you type `git pulse`.

Built as a learning project to go from "I only know `println!`" to a real, daily-use Rust CLI tool.

[![Latest Release](https://img.shields.io/badge/release-v0.1.0-blueviolet)](https://github.com/prakash-timalsina/gitpulse/releases/tag/v0.1.0)

---

## 1. Why This Project Exists

### The problem

Prakash juggles multiple active projects at once — Sarkari-Farkari, EasyNepalTrek, Wordinvent, Upasak, Yatra, Rental Room Finder, and more. It's easy to lose track of:

- Which branch you're actually on in a given repo
- Whether you left uncommitted changes somewhere days ago
- Which branches have gone stale and can be deleted
- How far a branch has drifted from `main`

Running `git status`, `git branch -vv`, and `git log` manually across every project, every time, is friction. This tool collapses all of that into one command with a readable, colored report.

### The learning goal

This is also, deliberately, a **first Rust project**. The goal was not to build the most powerful tool possible — it was to build something small enough to finish, real enough to use daily, and structured so that each step taught exactly one new Rust concept (CLI parsing, error handling with `Result`/`Option`/`?`, working with a real library's types via `git2`, string formatting, and how Git subcommands work under the hood).

`git2` (Rust bindings to `libgit2`) was chosen deliberately over shelling out to the `git` binary, because it's better rehearsal for a future goal: building a local LLM inference CLI that will need to bind to a C library (`llama.cpp`) in a similar way. Learning "how do I talk to a C library from Rust" now, on low stakes, pays off later.

---

## 2. What It Actually Does

`cd` into any git repo and run:

```bash
gitpulse
```

You get a report like:

```
       _ _               _
  __ _(_) |_ _ __  _   _| |___  ___
 / _` | | __| '_ \| | | | / __|/ _ \
| (_| | | |_| |_) | |_| | \__ \  __/
 \__, |_|\__| .__/ \__,_|_|___/\___|
 |___/      |_|

Branch: main
Changes: 0 modified, 0 staged, 0 untracked
Last commit: 9 minutes ago
Other branches:
  feature/branch-staleness — 9 minutes ago, 0 ahead, 0 behind main
  feature/file-status-and-branch-info — 9 hours ago, 0 ahead, 5 behind main
```

Green = clean and current. Yellow = getting stale / has uncommitted work. Red = seriously stale or heavily diverged.

Run `gitpulse --verbose` for extra detail, including the resolved `.git` path.

---

## 3. Installation

### Option A — Download the prebuilt binary (Linux x86_64 only)

Grab the latest release from the [Releases page](https://github.com/prakash-timalsina/gitpulse/releases/latest):

```bash
chmod +x gitpulse
./gitpulse --verbose
```

Move it onto your `PATH` (e.g. `~/.cargo/bin/` or `~/.local/bin/`) to run it as just `gitpulse` from anywhere.

> ⚠️ The published binary is built and tested on Fedora Linux (x86_64). It will not run on Windows or macOS. A cross-platform build is planned but not yet available — see [Section 8](#8-status).

### Option B — Build from source (any platform with Rust installed)

```bash
git clone https://github.com/prakash-timalsina/gitpulse.git
cd gitpulse
cargo install --path .
gitpulse --verbose
```

### Optional — make `git pulse` work as a real subcommand

Git auto-discovers any executable named `git-<name>` on your `PATH`. Symlink the binary to enable `git pulse`:

```bash
ln -s ~/.cargo/bin/gitpulse ~/.cargo/bin/git-pulse
git pulse --verbose
```

---

## 4. Feature Roadmap

### Phase 1 — Core health report ✅ Shipped in [v0.1.0](https://github.com/prakash-timalsina/gitpulse/releases/tag/v0.1.0)

- [x] Show current branch name
- [x] Show count of modified / staged / untracked files
- [x] Show days since last commit on current branch
- [x] List all local branches with staleness (days since last commit) and ahead/behind count vs `main`
- [x] Colored terminal output based on health thresholds
- [x] ASCII art banner on startup

### Phase 2 — `git undo` (planned)

- [ ] Show a diff/summary of what the last commit changed
- [ ] Prompt for confirmation before undoing
- [ ] Support undoing last commit (soft reset) vs last uncommitted change (checkout)

### Phase 3 — `git sync` (planned)

- [ ] Fetch from remote
- [ ] Show a preview of what rebase/merge would do before doing it
- [ ] Rebase + push only after confirmation

### Phase 4 — Stretch goals (not scheduled)

- [ ] Config file for custom staleness thresholds
- [ ] Multi-repo mode (scan a directory of repos and report on all of them)
- [ ] Cross-platform release builds (Windows, macOS)

---

## 5. Tech Stack

| Tool        | Role                                                                   | Familiar equivalent (JS/TS world)            |
| ----------- | ---------------------------------------------------------------------- | -------------------------------------------- |
| `cargo`     | Build tool & package manager                                           | `npm` + `package.json`                       |
| `clap`      | Parses CLI arguments                                                   | `commander` / `yargs`                        |
| `git2`      | Rust bindings to `libgit2` — reads repo data directly, no shelling out | Using a proper SDK instead of exec-ing a CLI |
| `chrono`    | Date/time math (for "N days ago")                                      | `date-fns` / `dayjs`                         |
| `colored`   | Colored terminal output                                                | `chalk`                                      |
| `figlet-rs` | Generates the ASCII art startup banner                                 | —                                            |

No async. No web server. No database. Pure, small, standalone CLI binary — intentionally scoped down for a first Rust project.

---

## 6. Architecture

### Folder structure

```
gitpulse/
├── Cargo.toml          # dependency manifest (like package.json)
└── src/
    ├── main.rs          # entry point: parses args, orchestrates git_ops + display
    ├── git_ops.rs       # all logic that talks to git2 and chrono, returns plain data
    └── display.rs       # all logic that formats and prints colored output
```

### Design principle

Three responsibilities, three files:

1. **`main.rs`** — orchestration only. Reads CLI input, calls into the other two modules. No Git logic, no printing logic here.
2. **`git_ops.rs`** — all "ask the repo a question, get an answer back" functions (`current_branch`, `file_status_counts`, `last_commit_time`, `list_other_branches`). The only file that imports `git2` or `chrono`.
3. **`display.rs`** — all "take data, make it pretty" functions (`print_header`, `print_changes`, `print_last_commit`, `print_other_branches`). The only file that imports `colored`.

This mirrors a pattern already familiar from Next.js projects: keep data-fetching logic separate from presentation logic, rather than jamming both into one file/route.

---

## 7. What Building This Taught (Concept Log)

Since this was a first Rust project, here's what each phase actually introduced:

- **`clap` + `#[derive(Parser)]`** — describing the CLI's shape as a struct and letting a macro generate parsing, `--help`, and `--version`.
- **`Result<T, E>` and `match`** — handling `Repository::open()` failing gracefully instead of crashing.
- **Borrowing (`&Repository`)** — passing read-only references into functions instead of transferring ownership.
- **`Option<T>` and the `?` operator** — representing "might legitimately not exist" (e.g. a repo with no commits) and propagating failure early without nested `match` blocks.
- **Modules (`mod git_ops; mod display;`)** — splitting logic across files with `pub fn`/`pub struct` controlling visibility.
- **`chrono`** — converting raw Unix timestamps from Git into human-readable durations ("2 days ago").
- **Iterators and `Vec<T>`** — looping over `repo.branches()` and building up a list of branch info.
- **Let-else syntax** — `let Ok(x) = y else { continue };` for skipping invalid loop items cleanly.
- **Traits as extension methods** — `colored`'s `.green()`/`.bold()` attach directly onto strings you already have.
- **Higher-order functions** — passing `humanize_duration` itself as a function value into `display.rs`, keeping presentation logic decoupled from date logic.

---

## 8. Status

📍 **Current stage:** Phase 1 complete and released as [v0.1.0](https://github.com/prakash-timalsina/gitpulse/releases/tag/v0.1.0). Linux x86_64 binary published; Windows/macOS builds not yet available.

**Next up:** Phase 2 (`git undo`), or a detour into the next project in the learning sequence below.

---

## 9. Personal Context (Why This Matters)

This project sits alongside a broader move from Fedora/GNOME to EndeavourOS/Hyprland, and a broader interest in owning and understanding the tools used daily rather than just consuming them. It's the first of a planned Rust learning sequence:

**gitpulse → system resource monitor (TUI) → smart notes CLI → dotfiles bootstrap manager → local LLM inference CLI**

Each project is scoped to teach what's needed for the next one. This one taught the fundamentals; the LLM CLI at the end of the sequence is where all of it — CLI design, error handling, and C-library bindings via `git2` here and `llama.cpp` bindings later — comes together.

---

## Contributing / Feedback

This is a personal learning project, but issues and suggestions are welcome via [GitHub Issues](https://github.com/prakash-timalsina/gitpulse/issues).
