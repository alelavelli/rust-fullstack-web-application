# <https://github.com/casey/just>

# Prevent showing the recipe name when running

set quiet

# Default recipe, it's run when just is invoked without a recipe

default:
  just --list --unsorted

set-hooks:
  cp .hooks/pre-push .git/hooks/pre-push && chmod +x .git/hooks/pre-push

lint:
  cargo clippy

test:
  cargo test