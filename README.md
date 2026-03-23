# dotctor

A CLI tool to diagnose your dotfiles health.

> This project is specialized for
> [nozomi-koborinai/dotfiles](https://github.com/nozomi-koborinai/dotfiles).
> Built as a Rust learning project.

## Checks

| Check | Description |
|-------|-------------|
| symlink | Detect broken or missing symlinks |
| tools | Verify required commands exist in PATH |
| version | Check tool versions meet expectations |

## Install

```bash
cargo install --git https://github.com/nozomi-koborinai/dotctor
```

## Usage

Run all checks:

```bash
dotctor
```

Run a specific check:

```bash
dotctor --check symlink
dotctor --check tools
dotctor --check version
```

Use a custom config file:

```bash
dotctor --config /path/to/dotctor.toml
```

## Configuration

dotctor looks for config files in this order:

1. `--config` option
2. `dotctor.toml` in the current directory
3. `~/.dotctor.toml`

```toml
[symlinks]
"~/.zshrc" = "configs/zshrc"
"~/.gitconfig" = "configs/gitconfig"

[tools]
required = ["git", "nvim", "gh"]

[versions]
node = ">= 22"
git = ">= 2"
nvim = ">= 0.10"
```

## Output

```
dotctor - diagnosing your dotfiles...

[symlink]
    OK  ~/.zshrc -> configs/zshrc
    OK  ~/.gitconfig -> configs/gitconfig

[tools]
    OK  git is installed
   ERR  nvim is not installed

[version]
    OK  node 22.12.0 (>= 22)
```

Exit code is `1` if any check reports an error, `0` otherwise.
