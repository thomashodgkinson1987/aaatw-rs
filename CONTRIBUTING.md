# Contributing

Thanks for your interest in `aaatw-rs`.

## Branch model

- `main` is the stable branch. It contains released versions only.
- `develop` is the integration branch for ongoing work.
- Feature branches are created from `develop` and merged back into `develop`
  with `git merge --no-ff`.
- Release merges from `develop` into `main` also use `--no-ff`.
- Release tags are created on `main` (for example `v0.1.0`).

```text
main:     A---B---C---D
                \   /
develop:        E---F---G
feature:            H
```

## Commit messages

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```text
<type>(<optional scope>): <description>

[optional body]

[optional footer(s)]
```

Common types:

- `feat`: a new feature
- `fix`: a bug fix
- `docs`: documentation only
- `style`: formatting, no code change
- `refactor`: code change that neither fixes a bug nor adds a feature
- `perf`: performance improvement
- `test`: adding or fixing tests
- `build`: build system or dependencies
- `ci`: CI configuration
- `chore`: maintenance tasks
- `revert`: reverting a previous commit

Examples:

```text
feat(assets): parse PE resource directory
fix(core): handle zero-length surfaces
docs: describe splash state machine
```

## Semantic versioning

This project follows [SemVer 2.0.0](https://semver.org/):

- **MAJOR** for incompatible API changes.
- **MINOR** for backwards-compatible functionality.
- **PATCH** for backwards-compatible fixes.

While the project is `0.x`, breaking changes may be released in a MINOR version.
The first stable `1.0.0` release will mark the point where the public API and
save/config formats are considered stable.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release -p aaatw-cli
```

Run the asset tool:

```sh
cargo run -p aaatw-extract -- list /path/to/Resource.dll
```

## Releases

Releases are cut by tagging `main` with `vX.Y.Z` and pushing the tag. The Release
workflow builds the artifacts, generates release notes with git-cliff, and publishes
the GitHub Release.

- Use SemVer for the tag (`v0.1.1`, `v0.2.0`, etc.).
- Update `CHANGELOG.md` under `Unreleased` before tagging.
- Keep commit messages Conventional Commits so git-cliff can group them.
- Do not hand-edit the generated release notes; change the changelog and commit
  messages instead.

## Pull requests

1. Create a feature branch from `develop`.
2. Keep commits focused and follow Conventional Commits.
3. Add tests for new parsers and game logic.
4. Update `CHANGELOG.md` under `Unreleased`.
5. Ensure CI passes.
6. Merge with `--no-ff` into `develop`.