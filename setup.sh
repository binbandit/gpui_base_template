#!/usr/bin/env bash
# Turn this template into your own GPUI project.
#
# Usage:
#   ./setup.sh <project-name> [options]
#
# Options:
#   --app-only      Keep the showcase architecture but fold src/app/ into a
#                   binary-only project. Removes src/lib.rs and examples/.
#   --minimal       Produce a bare one-file GPUI hello world. Implies
#                   --app-only and removes the showcase-only dependencies.
#   --no-examples   Delete examples/ but keep the library target.
#   --fresh-git     Replace template history with a new initial commit.
#   --yes, -y       Accept defaults without prompting.
#
# The script validates the generated project and deletes itself only after a
# successful run.

set -euo pipefail

OLD_PKG="gpui-base-framework"
OLD_IDENT="gpui_base_framework"

err() { printf 'error: %s\n' "$1" >&2; exit 1; }
note() { printf '  %s\n' "$1"; }

cd "$(dirname "$0")"

NAME="${1:-}"
[ -n "$NAME" ] && [ "${NAME#-}" = "$NAME" ] && shift || NAME=""

APP_ONLY=false
MINIMAL=false
NO_EXAMPLES=false
FRESH_GIT=false
ASSUME_YES=false

while [ $# -gt 0 ]; do
    case "$1" in
        --app-only) APP_ONLY=true; NO_EXAMPLES=true ;;
        --minimal) MINIMAL=true; APP_ONLY=true; NO_EXAMPLES=true ;;
        --no-examples) NO_EXAMPLES=true ;;
        --fresh-git) FRESH_GIT=true ;;
        --yes|-y) ASSUME_YES=true ;;
        -h|--help) sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
        *) err "unknown option: $1 (see ./setup.sh --help)" ;;
    esac
    shift
done

if [ -z "$NAME" ]; then
    $ASSUME_YES && err "a project name is required with --yes"
    printf 'Project name (e.g. my-desktop-app): '
    read -r NAME
fi

case "$NAME" in
    ''|*[!a-zA-Z0-9_-]*) err "invalid crate name: '$NAME' (use letters, digits, - and _)" ;;
    [!a-zA-Z]*) err "crate names must start with a letter" ;;
    app) err "'app' collides with the template's internal module; pick another name" ;;
esac
IDENT="$(printf '%s' "$NAME" | tr '-' '_')"
case "$IDENT" in
    abstract|as|async|await|become|box|break|const|continue|crate|do|dyn|else|enum|extern|false|final|fn|for|gen|if|impl|in|let|loop|macro|match|mod|move|mut|override|priv|pub|ref|return|self|Self|static|struct|super|trait|true|try|type|typeof|unsafe|unsized|use|virtual|where|while|yield)
        err "'$NAME' becomes the reserved Rust identifier '$IDENT'; pick another name"
        ;;
esac

if ! $ASSUME_YES && ! $APP_ONLY; then
    printf 'Create a binary-only app (recommended for products)? [y/N] '
    read -r reply
    case "$reply" in [yY]*) APP_ONLY=true; NO_EXAMPLES=true ;; esac
fi

if ! $ASSUME_YES && ! $FRESH_GIT && [ -d .git ]; then
    printf 'Start a fresh git history? [y/N] '
    read -r reply
    case "$reply" in [yY]*) FRESH_GIT=true ;; esac
fi

command -v cargo >/dev/null 2>&1 || err "cargo is required to validate the generated project"
cargo fmt --version >/dev/null 2>&1 || err "rustfmt is required (install it with rustup component add rustfmt)"
if $FRESH_GIT && [ -f .git ]; then
    err "--fresh-git cannot replace a linked worktree; copy the project outside its worktree first"
fi

GIT_NAME="$(git config user.name 2>/dev/null || true)"
GIT_EMAIL="$(git config user.email 2>/dev/null || true)"
if $FRESH_GIT && { [ -z "$GIT_NAME" ] || [ -z "$GIT_EMAIL" ]; }; then
    err "--fresh-git requires git user.name and user.email to be configured"
fi

# All source mutations are transactional. If validation or the fresh-history
# commit fails, the EXIT trap restores the exact pre-setup project files.
BACKUP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/gpui-template-setup.XXXXXX")"
tar --exclude='./target' --exclude='./.git' --exclude='./.pi-subagents' \
    -cf "$BACKUP_DIR/project.tar" .
SETUP_COMPLETE=false
GIT_REPLACED=false
rollback_on_error() {
    status=$?
    trap - EXIT
    if ! $SETUP_COMPLETE; then
        tar -xf "$BACKUP_DIR/project.tar" -C .
        if $GIT_REPLACED; then
            rm -rf .git
            if [ -d "$BACKUP_DIR/original.git" ]; then
                mv "$BACKUP_DIR/original.git" .git
            fi
        fi
        printf '  setup failed; restored the original project\n' >&2
    fi
    rm -rf "$BACKUP_DIR"
    exit "$status"
}
trap rollback_on_error EXIT

echo "Setting up '$NAME'..."

# Write through a temporary file outside the project. In-place sed backup
# suffixes could overwrite a developer's existing backup files.
edit_file() {
    local file="$1"
    shift
    sed "$@" "$file" > "$BACKUP_DIR/edited"
    cat "$BACKUP_DIR/edited" > "$file"
}

# Literal replacement is intentional: template invariants keep these names
# unsplit so BSD and GNU userlands can perform the same simple transformation.
for directory in src examples; do
    [ -d "$directory" ] || continue
    while IFS= read -r -d '' file; do
        edit_file "$file" -e "s/$OLD_IDENT/$IDENT/g" -e "s/$OLD_PKG/$NAME/g"
    done < <(find "$directory" -type f -name '*.rs' -print0)
done
for file in Cargo.toml Cargo.lock ./*.md docs/*.md examples/*.md; do
    [ -f "$file" ] || continue
    edit_file "$file" -e "s/$OLD_IDENT/$IDENT/g" -e "s/$OLD_PKG/$NAME/g"
done
note "renamed crate to '$NAME' (module path '$IDENT')"

# Removing a previous value makes retries safe. Quotes and backslashes from Git
# configuration are escaped for a TOML basic string.
edit_file Cargo.toml -e '/^authors = /d'
if [ -n "$GIT_NAME" ]; then
    AUTHOR="$GIT_NAME${GIT_EMAIL:+ <$GIT_EMAIL>}"
    AUTHOR_ESCAPED="$(printf '%s' "$AUTHOR" | sed -e 's/\\/\\\\\\\\/g' -e 's/"/\\\\\\"/g')"
    edit_file Cargo.toml "/^version = /a\\
authors = [\"$AUTHOR_ESCAPED\"]"
    note "set authors from git config"
else
    note "left authors unset (git config has no user.name)"
fi

edit_file Cargo.toml \
    -e 's|^description = .*|description = "TODO: describe your app"|' \
    -e '/^repository = /d' \
    -e '/^homepage = /d' \
    -e '/^keywords = /d' \
    -e '/^categories = /d'
note "reset package metadata"

edit_file Cargo.toml '/^# --- template setup:start ---$/,/^# --- template setup:end ---$/d'
edit_file Cargo.toml '/^$/N;/^\n$/D'

if [ -f AGENTS.md ]; then
    edit_file AGENTS.md '/<!-- template-only:start -->/,/<!-- template-only:end -->/d'
    edit_file AGENTS.md '/^$/N;/^\n$/D'
    note "trimmed AGENTS.md to the generated-app guide"
fi

if $MINIMAL; then
    cp examples/hello_world.rs src/main.rs
    rm -rf src/app src/lib.rs examples assets

    # Keep only dependencies used by the one-file hello world and remove the
    # test-only GPUI feature block. This awk is portable to macOS/BSD awk.
    awk '
        /^\[dependencies\]$/ { in_dependencies = 1; print; next }
        /^\[dev-dependencies\]$/ { in_dependencies = 0; in_dev = 1; next }
        /^\[/ {
            if (in_dev) in_dev = 0
            in_dependencies = 0
        }
        in_dev { next }
        in_dependencies && /^(anyhow|directories|rust-embed|serde|serde_json|tempfile|tracing|tracing-subscriber) =/ { next }
        { print }
    ' Cargo.toml > "$BACKUP_DIR/edited"
    cat "$BACKUP_DIR/edited" > Cargo.toml
    note "created a minimal one-file GPUI app"
elif $APP_ONLY; then
    rm -f src/lib.rs
    rm -rf examples

    # The launcher is the only file allowed to import through the crate root.
    # Keep src/app byte-for-byte portable between library and binary projects.
    edit_file src/main.rs -e "s/$IDENT::/crate::app::/g"

    awk '!done && /^use / {
             print "#[allow(dead_code, unused_imports)]"
             print "mod app;"
             print ""
             done = 1
         }
         { print }' src/main.rs > "$BACKUP_DIR/edited"
    cat "$BACKUP_DIR/edited" > src/main.rs
    grep -q '^mod app;' src/main.rs \
        || err "could not insert 'mod app;' into src/main.rs (no top-level use line found?)"
    note "converted to a binary-only app (application lives in src/app/)"
elif $NO_EXAMPLES; then
    rm -rf examples
    note "removed examples/"
fi

# Replace template-facing documentation with an app-facing README. The deeper
# architecture, versioning, and cheat-sheet docs remain available.
if $MINIMAL; then
    cat > README.md <<'README'
# __PROJECT_NAME__

TODO: describe your application.

A native Rust desktop app built with the published GPUI 0.2.2 API.

## Run

    cargo run

## Quality

    cargo fmt --all -- --check
    cargo check --all-targets
    cargo test --all-targets
    cargo clippy --all-targets --all-features -- -D warnings

Start in src/main.rs. GPUI is pinned exactly in Cargo.toml because it is pre-1.0.
Before upgrading, read docs/GPUI_VERSIONING.md.

Licensed under MIT or Apache-2.0.
README
else
    cat > README.md <<'README'
# __PROJECT_NAME__

TODO: describe your application.

A native Rust desktop app built with GPUI. The starter includes a responsive
shell, semantic themes, actions and shortcuts, embedded assets, atomic settings,
structured logging, async work, tests, and native release automation.

## Run

    cargo run

## Start building

- src/main.rs: launcher
- src/app/root.rs: state, tasks, and action handlers
- src/app/actions.rs: logical commands and shortcuts
- src/app/components/: public reusable controls and their guide
- src/app/pages/: one module per route-level screen
- src/app/shell/: navigation, header, and global notices
- src/app/services/: settings and future external IO boundaries
- src/app/state.rs: pure product state
- src/app/theme.rs: semantic design tokens

See QUICKSTART.md, CHEATSHEET.md, docs/ARCHITECTURE.md, and
docs/GPUI_VERSIONING.md before a GPUI upgrade. GPUI 0.2.2 cannot expose semantic
accessibility roles for these custom controls; read docs/ACCESSIBILITY.md before
shipping.

## Quality

    cargo fmt --all -- --check
    cargo check --all-targets
    cargo test --all-targets
    cargo clippy --all-targets --all-features -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

Push a version tag such as v0.1.0 to run the native release workflow. A new
release stays a draft until every platform archive and checksum has uploaded
successfully. Configure bundle identifiers, signing, notarization, icons, and
your update strategy before distribution.

Licensed under MIT or Apache-2.0.
README
fi
edit_file README.md "s/__PROJECT_NAME__/$NAME/g"
note "wrote an app-facing README.md"

echo "Verifying generated project..."
cargo fmt --all --quiet
cargo check --all-targets --quiet
note "cargo check passed"

if $FRESH_GIT; then
    if [ -d .git ]; then
        mv .git "$BACKUP_DIR/original.git"
    fi
    GIT_REPLACED=true
    git init -q
    git add -A
    # Keep the self-deleting setup tool out of the new history while retaining
    # it until the commit succeeds, so every failure remains recoverable.
    git rm -q --cached setup.sh
    git -c user.name="$GIT_NAME" -c user.email="$GIT_EMAIL" \
        commit -qm "Initial commit (from gpui-base-framework template)"
    note "started fresh git history"
fi

rm -f -- "$0"
note "removed setup.sh"
SETUP_COMPLETE=true
rm -rf "$BACKUP_DIR"

echo
echo "Done. Your app is ready:"
echo "  cargo run"
echo
if $MINIMAL; then
    echo "Start editing src/main.rs. CHEATSHEET.md contains verified GPUI patterns."
else
    echo "Start in src/main.rs and src/app/. CHEATSHEET.md contains verified GPUI patterns."
fi
