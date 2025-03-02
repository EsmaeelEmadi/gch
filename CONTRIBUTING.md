# Branching Convention

### main Branch

Represents the production-ready codebase.

Only stable and thoroughly tested changes are merged here.

### stage Branch

Serves as the pre-production environment.

Features are integrated and tested collectively before being promoted to main.

### Feature Branches

Created from the stage branch for developing new features or bug fixes. Once development is complete, these branches are merged back into stage.

### develop Branch

A temporary branch derived from stage.

Used for testing individual features in isolation before they are merged into stage. After testing, changes are merged back into the respective feature branch or directly into stage, and the develop branch is deleted.

## Workflow Summary:

Developers create feature branches from stage to work on new features or fixes.

For isolated testing, a develop branch is created from stage, where the feature is tested.

After successful testing, the feature branch is merged back into stage.
Once stage has accumulated stable changes and passed all tests, it is merged into main for production release.

# Branch Naming Conventions

Use the following prefixes for branch names to indicate the purpose of the branch:

- feature/: New functionality or user-facing enhancements.
- bugfix/: Non-critical fixes for issues in development/staging environments.
- hotfix/: Urgent fixes for production issues (bypassing regular development flow).
- refactor/: Code restructuring without changing behavior.
- release/: Preparation for a new release (version bumps, final checks).
- chore/: Maintenance tasks (dependency updates, config changes).
- test/: Test-related changes (new suites, fixing flaky tests).
- docs/: Documentation updates.
- domain/: Changes in business logic.

For example, a branch implementing a new authentication feature should be named prefix/title_jira-id`.

# Commit Message Guidelines

Consistent commit messages help in understanding the history and purpose of changes.

### Commit Types

- feat: A new feature.
- fix: A bug fix.
- style: Code style changes (formatting, etc.).
- refactor: Code refactoring.
- test: Adding or updating tests.
- chore: Maintenance or tooling.

### Scopes

- core: Core functionality.
- config: Configuration files.
- deps: Dependencies.
- scripts: Scripts.
- setup: Setup-related changes.
- docs: Documentation.
- build: Build process.
- ci: Continuous Integration.
- testing: Testing.