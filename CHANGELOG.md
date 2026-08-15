# Changelog

All notable changes to this project will be documented in this file.

## [v1.3.0](https://github.com/Antiz96/aps/releases/tag/v1.3.0) - 2026-08-15

### Features

- *(patterns)* Add support for wildcard (`*`) expansion - ([3f1230c](https://github.com/Antiz96/aps/commit/3f1230c95f15f1d2182f737d3aec062654a5fbe5)) by @Antiz96
- Add the new `-P / --pkgbases` and `-R / --refresh-pkgbases` flags - ([c68d847](https://github.com/Antiz96/aps/commit/c68d847863561a85b3e8981a72ed47877ffd75ba)) by @Antiz96

### Fixes

- *(results)* Print a message if there are no (detailed) results - ([9bd94da](https://github.com/Antiz96/aps/commit/9bd94dae7974efb7debc16f2d40fd30e62072f47)) by @Antiz96

### Styling

- Typo fix in comment - ([6a4e0e4](https://github.com/Antiz96/aps/commit/6a4e0e4e34a0109df67b64ba3578ab4944eb046b)) by @Antiz96

### Miscellaneous

- *(patterns)* Update patterns list - ([b6e4d5c](https://github.com/Antiz96/aps/commit/b6e4d5c39fa962c34c2a446b3aeec0940a79fe3d)) by @Antiz96
- *(perf)* Allow caching the pkgbases list in pkgbases.txt - ([76ee339](https://github.com/Antiz96/aps/commit/76ee33915845ce7e4eef18e041870237d0d327a9)) by @tippfehlr
- *(perf)* Search multithreaded (split by branch) - ([5942ff0](https://github.com/Antiz96/aps/commit/5942ff0da1340f32f0750ead27d3fb3b4d3efcd5)) by @tippfehlr
- *(perf)* Use aho_corasick to find patterns occurrences - ([b9e9279](https://github.com/Antiz96/aps/commit/b9e92792da39b07921de777da1aceeef172ee881)) by @tippfehlr
- Move to GitHub - ([857d633](https://github.com/Antiz96/aps/commit/857d633c6a1e6c8d610b2c9d1455f932d599fa86)) by @Antiz96
- Update patterns list - ([5cbdb26](https://github.com/Antiz96/aps/commit/5cbdb26ab2f3eda399e9e301defb2fdc38e05d0e)) by @Antiz96
- Update patterns list - ([b82d9b6](https://github.com/Antiz96/aps/commit/b82d9b631266fb790e729d7f453a9fb40df22e04)) by @Antiz96

## [v1.2.0](https://gitlab.archlinux.org/antiz/aps/-/releases/v1.2.0) - 2026-08-12

### Features

- Add context to detailed output
- Add patterns with no occurrence found to the summary output

### Styling

- Format results summary output in columns

### Miscellaneous

- Move result formatting logic to its own module

## [v1.1.0](https://gitlab.archlinux.org/antiz/aps/-/releases/v1.1.0) - 2026-08-12

### Features

- Add result summary output

### Miscellaneous

*(patterns)* Update patterns list

## [v1.0.2](https://gitlab.archlinux.org/antiz/aps/-/releases/v1.0.2) - 2026-08-12

### Fixes

- Improve logging

## [v1.0.1](https://gitlab.archlinux.org/antiz/aps/-/releases/v1.0.1) - 2026-08-12

### Fixes

- fix `--fetch` option

### Miscellaneous

- *(release)* Run cargo update from release script

## [v1.0.0](https://gitlab.archlinux.org/antiz/aps/-/releases/v1.0.0) - 2026-08-12

### Features

- Initial commit

### Fixes

- *(release)* Add execution bit to the release script
- *(release)* Fix release script
