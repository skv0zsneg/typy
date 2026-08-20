[doc("All command information")]
default:
  @just --list --unsorted --list-heading $'TyPy commands…\n'

[doc("Run tests")]
test:
  cargo test --locked --all-features --verbose

[doc("Format")]
format:
  cargo fmt --all

[doc("Check Format")]
check-format:
  cargo fmt --all -- --check

[doc("Linter")]
linter:
  cargo clippy --locked --all-targets --all-features -- -D warnings

[doc("Check All")]
check-all: check-format linter test
