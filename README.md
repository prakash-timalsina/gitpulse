# gitpulse

**Check the pulse of your repo.**

A command-line tool that tells you the truth about your Git repository's state — stale branches, uncommitted mess, and commit activity — at a glance, in color, the moment you type `git pulse`.

Built as a learning project to go from "I only know `println!`" to a real, daily-use Rust CLI tool.

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

This is also, deliberately, a **first Rust project**. The goal is not to build the most powerful tool possible — it's to build something small enough to finish, real enough to use daily, and structured so that each step teaches exactly one new Rust concept (CLI parsing, error handling with `Result`/`?`, working with a real library's types via `git2`, string formatting, and eventually how Git subcommands work under the hood).

`git2` (Rust bindings to `libgit2`) was chosen deliberately over shelling out to the `git` binary, because it's better rehearsal for a future goal: building a local LLM inference CLI that will need to bind to a C library (`llama.cpp`) in a similar way. Learning "how do I talk to a C library from Rust" now, on low stakes, pays off later.

---

## 2. What It Actually Does

You `cd` into any git repo and run:

```bash
git pulse
```

And instead of silence, you get a report like:

```
📁 sarkari-farkari
🌿 Branch: feature/incident-state-machine
📝 Changes: 3 modified, 1 untracked
⏳ Last commit: 2 days ago
🌱 Other branches:
   ✓ main            (up to date)
   ⚠ old-ddi-draft    (14 days stale, 3 behind main)
```

Green = clean and current. Yellow = getting stale / has uncommitted work. Red = seriously stale or heavily diverged.

---

## 3. Feature Roadmap (Phased — build one phase at a time)

### Phase 1 — `git pulse` (the entire first version, and the only "must build" phase)

- [ ] Show current branch name
- [ ] Show count of modified / staged / untracked files
- [ ] Show days since last commit on current branch
- [ ] List all local branches with staleness (days since last commit) and ahead/behind count vs `main`
- [ ] Colored terminal output based on health thresholds

**This phase alone is a complete, useful, shippable tool.** Everything below is optional future work — not required to consider this project "done."

### Phase 2 — `git undo`

- [ ] Show a diff/summary of what the last commit changed
- [ ] Prompt for confirmation before undoing
- [ ] Support undoing last commit (soft reset) vs last uncommitted change (checkout)

### Phase 3 — `git sync`

- [ ] Fetch from remote
- [ ] Show a preview of what rebase/merge would do before doing it
- [ ] Rebase + push only after confirmation

### Phase 4 (stretch, far future, not planned yet)

- [ ] Config file for custom staleness thresholds
- [ ] Multi-repo mode (scan a directory of repos and report on all of them)

---

## 4. Tech Stack

| Tool                      | Role                                                                   | Familiar equivalent (JS/TS world)                  |
| ------------------------- | ---------------------------------------------------------------------- | -------------------------------------------------- |
| `cargo`                   | Build tool & package manager                                           | `npm` + `package.json`                             |
| `clap`                    | Parses CLI arguments and subcommands                                   | `commander` / `yargs`                              |
| `git2`                    | Rust bindings to `libgit2` — reads repo data directly, no shelling out | Using a proper SDK instead of exec-ing a CLI       |
| `anyhow`                  | Simplified error handling while learning                               | try/catch without needing custom Error classes yet |
| `colored` or `owo-colors` | Colored terminal output                                                | `chalk`                                            |
| `chrono`                  | Date/time math (for "N days ago")                                      | `date-fns` / `dayjs`                               |

No async. No web server. No database. Pure, small, standalone CLI binary — intentionally scoped down for a first Rust project.

---

## 5. Architecture

### Folder structure

```
gitpulse/
├── Cargo.toml          # dependency manifest (like package.json)
└── src/
    ├── main.rs          # entry point: parses args, calls the right functions
    ├── git_ops.rs       # all logic that talks to git2 and extracts repo data
    └── display.rs       # all logic that formats and prints colored output
```

### Design principle

Three responsibilities, three files:

1. **`main.rs`** — orchestration only. Reads CLI input, decides what to do, calls into the other two modules. No Git logic, no printing logic here.
2. **`git_ops.rs`** — all "ask the repo a question, get an answer back" functions. E.g. `get_current_branch()`, `get_file_statuses()`, `get_branch_staleness()`. This is the only file that imports `git2`.
3. **`display.rs`** — all "take data, make it pretty" functions. E.g. `print_health_report(data)`. This is the only file that imports the coloring crate.

This mirrors a pattern already familiar from Next.js projects: keep data-fetching logic separate from presentation logic, rather than jamming both into one file/route.

---

## 6. Build Plan (Step-by-Step, Beginner Pace)

Each step introduces exactly one new concept.

| Step | What you build                                         | New concept learned                                                                           |
| ---- | ------------------------------------------------------ | --------------------------------------------------------------------------------------------- |
| 1    | `cargo new gitpulse`, confirm it runs and prints hello | Toolchain sanity check                                                                        |
| 2    | Add `clap`, recognize a `pulse` subcommand             | Parsing CLI input                                                                             |
| 3    | Add `git2`, open current directory as a `Repository`   | `Result`, the `?` operator, error propagation                                                 |
| 4    | Query the repo: current branch, file statuses          | Structs, working with a real library's types                                                  |
| 5    | Format and print the data with color                   | String formatting, conditionals/`match`                                                       |
| 6    | Install the binary as `gitpulse` on PATH               | How `git <subcommand>` auto-discovery works (not Rust-specific, but useful systems knowledge) |

No step here requires knowing Rust beyond what the previous step taught. Steps 1–3 are pure setup and plumbing; Step 4 is where it starts feeling like a real tool; Steps 5–6 are polish.

---

## 7. Out of Scope (For Now)

To keep this a finishable first project, the following are explicitly **not** being built in Phase 1:

- Any async or network code
- Any GUI or TUI (that's the _next_ Rust project — a `ratatui`-based system monitor)
- Config files or customization
- Multi-repo scanning
- Windows-specific path handling (Linux-first, since that's the daily environment)

---

## 8. Status

📍 **Current stage:** Planning complete, README written, no code yet.

Next step: Step 1 — `cargo new gitpulse` and confirm the toolchain works.

---

## 9. Personal Context (Why This Matters)

This project sits alongside a broader move from Fedora/GNOME to EndeavourOS/Hyprland, and a broader interest in owning and understanding the tools used daily rather than just consuming them. It's the first of a planned Rust learning sequence:

**gitpulse → system resource monitor (TUI) → smart notes CLI → dotfiles bootstrap manager → local LLM inference CLI**

Each project is scoped to teach what's needed for the next one. This one teaches the fundamentals; the LLM CLI at the end of the sequence is where all of it — CLI design, error handling, and C-library bindings via `git2` here and `llama.cpp` bindings later — comes together.
