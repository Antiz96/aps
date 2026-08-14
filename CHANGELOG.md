# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

### Features

- Add the new `-P / --pkgbases` and `-R / --refresh-pkgbases` flags - ([6434c85](https://github.com/Antiz96/aps/commit/6434c856243d6c8993b1e3c9f116b63f523c09ec)) by @Antiz96

### Styling

- Typo fix in comment - ([6a4e0e4](https://github.com/Antiz96/aps/commit/6a4e0e4e34a0109df67b64ba3578ab4944eb046b)) by @Antiz96

### Miscellaneous

- *(perf)* Allow caching the pkgbases list in pkgbases.txt - ([684b369](https://github.com/Antiz96/aps/commit/684b36991f4d85501663288e9a986aefc7311b4d))
- *(perf)* Search multithreaded (split by branch) - ([e9ee775](https://github.com/Antiz96/aps/commit/e9ee775c4c89225bd2fe9386d1783f2119e53519))
- *(perf)* Use aho_corasick to find patterns occurrences - ([b9e9279](https://github.com/Antiz96/aps/commit/b9e92792da39b07921de777da1aceeef172ee881))
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
