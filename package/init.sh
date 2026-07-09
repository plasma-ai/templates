#!/usr/bin/env bash
set -euo pipefail

# Initialize a repository from the Plasma package template
# --------------------------------------------------------

# Parse arguments
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
    exit 0
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --help | -h) usage ;;
        --context | --context=*)
            if [[ "$1" == *=* ]]; then
                CONTEXT="${1#*=}"
                shift
            elif [[ $# -ge 2 ]]; then
                CONTEXT="$2"
                shift 2
            else
                echo "--context requires a value" >&2
                exit 1
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
                echo "--org requires a value" >&2
                exit 1
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
                echo "--user requires a value" >&2
                exit 1
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
                echo "--email requires a value" >&2
                exit 1
            fi
            ;;
        --local)
            LOCAL=true
            shift
            ;;
        -*)
            echo "unknown option: $1" >&2
            exit 1
            ;;
        *)
            NAME="$1"
            shift
            ;;
    esac
done

# Validate required arguments
if [[ -z "$NAME" ]]; then
    printf '%s\n' "ERROR: repository name required" >&2
    exit 1
fi
if [[ -z "$CONTEXT" ]]; then
    printf '%s\n' "ERROR: --context required (path to JSON context file)" >&2
    exit 1
fi
if [[ -z "$ORG" ]]; then
    printf '%s\n' "ERROR: --org required" >&2
    exit 1
fi

# Resolve context file to absolute path
if [[ ! "$CONTEXT" = /* ]]; then
    CONTEXT="$(pwd)/$CONTEXT"
fi

# Create project from template via cruft
cruft create "https://github.com/$ORG/templates.git" --no-input --directory="package" -E "$CONTEXT"

# cookiecutter names the output dir from project.name; rename when the
# repository name differs (e.g. an import package with underscores)
SCAFFOLD=$(python3 -c "import json; print(json.load(open('$CONTEXT'))['project']['name'])")
if [[ "$SCAFFOLD" != "$NAME" && -d "$SCAFFOLD" && ! -e "$NAME" ]]; then
    mv "$SCAFFOLD" "$NAME"
fi

# Post-scaffolding setup
cd "$NAME"
ln -s AGENTS.md CLAUDE.md

# Initialize git with main branch
git init
[[ -n "$GIT_USER" ]] && git config user.name "$GIT_USER"
[[ -n "$GIT_EMAIL" ]] && git config user.email "$GIT_EMAIL"
git add . && git commit -m "init main"
git branch -M main

# Push to remote unless --local
if [[ "$LOCAL" != true ]]; then
    git remote add origin "https://github.com/$ORG/$NAME.git"
    git push --set-upstream origin main
fi
