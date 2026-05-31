# Shell Strategy — Avoid Interactive Hangs

When running shell commands, avoid commands that spawn interactive TTY sessions (pagers, editors, prompts). These will hang in non-TTY environments.

## DO use flags to disable interactivity

| Instead of | Use |
|-----------|-----|
| `git log` | `git log --no-pager -n 10` or `git --no-pager log -n 10` |
| `git diff` | `git --no-pager diff` |
| `less`, `more` | `cat`, `head`, `tail` |
| `cargo run` (interactive) | `cargo build && ./target/debug/binary` |
| `npm init` | `npm init -y` |
| `apt-get install` | `DEBIAN_FRONTEND=noninteractive apt-get install -y` |

## General rules

1. Add `--no-pager` to git commands that show output in a pager
2. Add `-y` or `--yes` to package managers (npm, apt, cargo)
3. Never run `vim`, `nano`, `less`, `more` — use `cat` or direct file editing tools
4. Never run `python`, `node`, or REPLs without a script argument
5. Chain commands with `&&` instead of `;` to stop on first failure and avoid orphan processes
6. Use `timeout <seconds>` prefix for commands that might hang indefinitely

## TTY detection

If a command fails with "not a TTY" or "no terminal", it means the tool needs a pseudo-terminal. Use:
- `script -q -c "cmd" /dev/null` (macOS/Linux) to wrap in a PTY
- Or run the command in a terminal tab via the shell plugin
