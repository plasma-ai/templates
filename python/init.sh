#!/usr/bin/env bash
set -euo pipefail

# Initialize a repository from the Plasma package template
# --------------------------------------------------------

# ------ argument parsing

NAME=""
CONTEXT=""
ORG=""
GIT_USER=""
GIT_EMAIL=""
LOCAL=false

usage() {
    cat <<USAGE
Usage: init.sh <name> --context <path> --org <org> [options]

Initialize a repository from the Plasma package template.

Options:
    --context <path>    Cookiecutter context JSON file (required)
    --org <org>         GitHub organization (required)
    --user <user>       Git user name (default: global git config)
    --email <email>     Git user email (default: global git config)
    --local             Skip pushing to the remote
    --help|-h           Show this help message
USAGE
    exit "${1:-0}"
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --help | -h) usage ;;
        --context | --context=*)
            if [[ "$1" == *=* ]]; then
                CONTEXT="${1#*=}"
                shift
            elif [[ $# -ge 2 ]]; then
                CONTEXT="$2"
                shift 2
            else
                echo "Error: --context requires an argument" >&2
                usage 1
            fi
            ;;
        --org | --org=*)
            if [[ "$1" == *=* ]]; then
                ORG="${1#*=}"
                shift
            elif [[ $# -ge 2 ]]; then
                ORG="$2"
                shift 2
            else
                echo "Error: --org requires an argument" >&2
                usage 1
            fi
            ;;
        --user | --user=*)
            if [[ "$1" == *=* ]]; then
                GIT_USER="${1#*=}"
                shift
            elif [[ $# -ge 2 ]]; then
                GIT_USER="$2"
                shift 2
            else
                echo "Error: --user requires an argument" >&2
                usage 1
            fi
            ;;
        --email | --email=*)
            if [[ "$1" == *=* ]]; then
                GIT_EMAIL="${1#*=}"
                shift
            elif [[ $# -ge 2 ]]; then
                GIT_EMAIL="$2"
                shift 2
            else
                echo "Error: --email requires an argument" >&2
                usage 1
            fi
            ;;
        --local)
            LOCAL=true
            shift
            ;;
        -*)
            echo "Error: unknown option: $1" >&2
            usage 1
            ;;
        *)
            if [[ -z "$NAME" ]]; then
                NAME="$1"
                shift
            else
                echo "Error: unexpected argument: $1" >&2
                usage 1
            fi
            ;;
    esac
done

# validate required arguments
if [[ -z "$NAME" ]]; then
    echo "Error: repository name is required" >&2
    exit 1
fi
if [[ -z "$CONTEXT" ]]; then
    echo "Error: --context is required (path to JSON context file)" >&2
    exit 1
fi
if [[ -z "$ORG" ]]; then
    echo "Error: --org is required" >&2
    exit 1
fi
if [[ ! -f "$CONTEXT" ]]; then
    echo "Error: context file not found: $CONTEXT" >&2
    exit 1
fi
if [[ -e "$NAME" ]]; then
    echo "Error: $NAME already exists" >&2
    exit 1
fi

# resolve context file to absolute path
if [[ ! "$CONTEXT" = /* ]]; then
    CONTEXT="$(pwd)/$CONTEXT"
fi

# ------ create repository from template

if ! command -v cruft &>/dev/null; then
    echo "Error: cruft is required (uv tool install cruft)" >&2
    exit 1
fi
if ! command -v python3 &>/dev/null; then
    echo "Error: python3 is required (brew install python)" >&2
    exit 1
fi

# create project from template via cruft
cruft create "https://github.com/$ORG/templates.git" --no-input --directory="package" -E "$CONTEXT"

# cookiecutter names the output dir from project.name; rename when the
# repository name differs (e.g. an import package with underscores)
# pass the path as argv, not spliced into the program text -- splicing
# breaks on quotes and backslashes in the path
SCAFFOLD=$(python3 -c "import json, sys; print(json.load(open(sys.argv[1], encoding='utf-8'))['project']['name'])" "$CONTEXT")
if [[ "$SCAFFOLD" != "$NAME" ]]; then
    mv "$SCAFFOLD" "$NAME"
fi

# default git branch from the context (cookiecutter default: main); the
# generated CI triggers and badges are branch-specific, so the git setup
# below must match
BRANCH=$(python3 -c "import json, sys; print(json.load(open(sys.argv[1], encoding='utf-8')).get('branch', 'main'))" "$CONTEXT")

echo "Created $NAME from the package template"

# ------ post-scaffolding setup

cd "$NAME"
ln -s AGENTS.md CLAUDE.md

# initialize git on the context's default branch
# -q: suppress git's init/commit chatter; the initialized summary below
# is the user-facing line
git init -q
[[ -n "$GIT_USER" ]] && git config user.name "$GIT_USER"
[[ -n "$GIT_EMAIL" ]] && git config user.email "$GIT_EMAIL"
git add . && git commit -q -m "init $BRANCH"
git branch -M "$BRANCH"
echo "Initialized git repository on branch $BRANCH"

# push to remote unless --local
if [[ "$LOCAL" != true ]]; then
    git remote add origin "https://github.com/$ORG/$NAME.git"
    # -q: suppress git's push progress; the pushed summary below is the
    # user-facing line
    git push -q --set-upstream origin "$BRANCH"
    echo "Pushed $BRANCH to https://github.com/$ORG/$NAME.git"
fi

# surface the next steps: enter the repository and install the environment
echo ""
echo "Next: cd $NAME && ./install.sh"
