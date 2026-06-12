# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## v3.3.2 - 2026-06-12
#### Bug Fixes
- Subject::len() counts Unicode chars, not bytes - (9efe76f) - Billie Thompson
- preserve trailer values containing ': ' in Trailer::try_from - (07274b2) - Billie Thompson
- preserve trailer value containing colon-space - (4020008) - Billie Thompson
#### Tests
- expose Subject::len() byte-vs-char bug with UTF-8 - (12548f9) - Billie Thompson
#### Build system
- remove duplicate lint - (58a7e33) - Billie Thompson
#### Continuous Integration
- trigger build - (c81c166) - Billie Thompson
- switch to woodpecker - (1f24322) - Billie Thompson
#### Refactoring
- resolve clippy option_if_let_else in scissors parse_sections - (d2c1093) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update rust docker digest to 087fe68 - (77426c8) - Solace System Renovate Fox
- (**deps**) update rust docker digest to a2d7edb - (8c2d99e) - Solace System Renovate Fox
- (**deps**) pin rust docker tag to c0601cf - (486e6a0) - Solace System Renovate Fox
- add .worktrees/ to gitignore - (2e07be8) - Billie Thompson
#### Style
- replace if-let-else with Option::map_or_else - (ccd424e) - Billie Thompson

- - -

## v3.3.1 - 2025-10-10
#### Continuous Integration
- consolidate image verification jobs - (d28f7c7) - Billie Thompson
- update fail-on flag to uppercase CRITICAL in concourse pipeline - (b0f6ecc) - Billie Thompson
- replace grype with trivy in concourse pipeline tasks - (3ff2576) - Billie Thompson
- remove unused image from trufflehog task - (aa7f8f3) - Billie Thompson
- reduce resource check intervals from 24h to 1h - (dd314bd) - Billie Thompson
- update Concourse pipeline configuration with minor formatting and dependency adjustments - (e8fbec3) - Billie Thompson
- replace docker-rust with ci-rust-env in concourse pipeline - (a7a51de) - Billie Thompson
- enable tag fetching in Concourse pipeline configuration - (6e1b1d3) - Billie Thompson
- add git author and committer details for renovate bot - (eb328e5) - Billie Thompson
- update GAR resource credentials for docker images - (8d8065f) - Billie Thompson
- update CI runtime image to custom repository - (d068760) - Billie Thompson
- refactor Concourse pipeline release task to use external task file - (ca380ea) - Billie Thompson
- add Concourse pipeline configuration for CI/CD workflow - (dc2b7bd) - Billie Thompson
- run less often - (1a83824) - PurpleBooth
- set a lower retension - (1ea1043) - PurpleBooth
#### Refactoring
- Markiere `len()` Methode als konstant in Trailers - (1e84d01) - Billie Thompson
#### Miscellaneous Chores
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0057852 - (7ae42e7) - Solace System Renovate Fox
- (**deps**) update actions/checkout action to v5 - (c460abb) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/checkout digest to 08eba0b - (076f341) - Solace System Renovate Fox
- (**deps**) update https://code.forgejo.org/actions/cache digest to 0400d5f - (6983078) - Solace System Renovate Fox
- (**deps**) update rust crate criterion to 0.7.0 - (00a9e67) - Solace System Renovate Fox
- resolve merge conflicts in concourse.yaml - (259ac3e) - Billie Thompson
- remove hardcoded git committer and author details in concourse config - (25fe867) - Billie Thompson
- resolve merge conflicts in concourse.yaml - (4b7fd27) - Billie Thompson
- update cog.toml configuration for branch whitelist and hooks - (fece5db) - Billie Thompson
- remove Forgejo workflow configuration file - (b4765b7) - Billie Thompson
#### Style
- (**yamlfix**) apply auto-fixes - (bdb0e01) - Solace System Renovate Fox [bot]
- (**yamlfix**) apply auto-fixes - (186044b) - Solace System Renovate Fox [bot]
- (**yamlfix**) apply auto-fixes - (42187b1) - Solace System Renovate Fox [bot]
- (**yamlfix**) apply auto-fixes - (ce759c1) - Solace System Renovate Fox [bot]

- - -

## v3.3.1 - 2025-06-17
#### Bug Fixes
- len isn't available as a const fn - (5d061cd) - Billie Thompson

- - -

## v3.3.0 - 2025-05-27
#### Features
- add `From<&CommitMessage<'_>>` trait implementation with tests - (15fccbd) - Billie Thompson
#### Refactoring
- optimize CommitMessage to String conversion without cloning - (f710b27) - Billie Thompson
- optimize CommitMessage to String conversion without cloning - (54499fc) - Billie Thompson

- - -

## v3.2.3 - 2025-05-26
#### Bug Fixes
- **(deps)** update rust crate nom to v8 - (d0973f3) - Solace System Renovate Fox
- add extra empty body fragment and refactor position matching - (2ad9d5e) - Billie Thompson
#### Continuous Integration
- migrate workflow from GitHub to Forgejo - (d056926) - Billie Thompson
#### Documentation
- improve documentation and move tests to inline modules - (67637f3) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust crate criterion to v0.6.0 - (461bc2c) - Solace System Renovate Fox
- **(deps)** update rust crate tempfile to v3.20.0 - (734e460) - Solace System Renovate Fox
- **(deps)** update rust crate quickcheck_macros to v1.1.0 - (45b4172) - Solace System Renovate Fox
- **(deps)** update rust crate indoc to v2.0.6 - (c4acf73) - Solace System Renovate Fox
- **(deps)** pin dependencies - (786570e) - Solace System Renovate Fox
- update dependencies and improve documentation with const methods - (9aed17b) - Billie Thompson
- Update repository links to Codeberg and bump version - (03b18f4) - Billie Thompson (aider)
#### Performance Improvements
- optimize memory allocations in commit message parsing - (87deaf0) - Billie Thompson
#### Refactoring
- remove nom dependency and update related comments - (7f9de3f) - Billie Thompson
- simplify commit message parsing with nom-based approach - (6ab8bf2) - Billie Thompson

- - -

## v3.2.2 - 2025-05-10
#### Bug Fixes
- upgrade to the latest rust version - (c1327a7) - Billie Thompson
- bump versions - (50ac581) - Billie Thompson
#### Continuous Integration
- enable mutation testing - (2d4584c) - Billie Thompson
#### Miscellaneous Chores
- **(deps)** update rust crate tokio to 1.40.0 - (63129fe) - renovate[bot]
- Remove unused `quickcheck` crate import - (396cc31) - Billie Thompson
- update renovate config to preserve semver ranges - (0cb8d41) - Billie Thompson
- Update .gitignore and renovate.json config - (6c4b993) - Billie Thompson
- Add mutation testing to dev tools - (dd02e1c) - Billie Thompson
#### Refactoring
- formatting - (0416568) - Billie Thompson
- Refactor lifetimes to use implicit '_ where applicable - (f984b66) - Billie Thompson

- - -

## v3.2.1 - 2024-08-25
#### Bug Fixes
- Rewrite scissors guessing algorithm to be cleaner - (3e52e37) - Billie Thompson
#### Continuous Integration
- Update pipeline to not push every commit - (512544e) - Billie Thompson
- Add renovate.json (#120) - (d0f1bd4) - renovate[bot]
#### Miscellaneous Chores
- **(deps)** update rust crate tempfile to 3.12.0 - (26928c3) - renovate[bot]
- add mutation testing files to ignore - (70d1bd8) - Billie Thompson
#### Tests
- Demonstate guess comment character not matching - (41d9041) - Billie Thompson
- Additional confidence around scissors - (b8b4746) - Billie Thompson
- Cover some test cases - (b2f79f5) - Billie Thompson

- - -

## v3.2.0 - 2024-07-26
#### Bug Fixes
- Bump versions - (6bbd45b) - Billie Thompson
#### Continuous Integration
- Lint check for commit message was remove - (b267abd) - Billie Thompson
- Remove changelog - (e26acd4) - Billie Thompson
- Remove commit message check as it catches ancient commits - (f647267) - Billie Thompson
#### Features
- Add idiomatic by ref into iterators - (e530a87) - Billie Thompson

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).