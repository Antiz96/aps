# APS

## Table of contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [Automated public reports](#automated-public-reports)
- [License](#license)

## Description

APS (**A**UR **P**atterns **S**earcher) is a tool allowing to search for specific patterns through the AUR package sources. Useful to get fast, automated and pro-active reports of the presence of knowingly suspicious / malicious patterns in AUR package sources.

 It requires a clone of the AUR GitHub mirror. A bare clone is recommended for optimal scanning performance:  
`git clone --bare https://github.com/archlinux/aur.git`.

Search patterns are read from a given file, one pattern per line.  
Lines starting with `#` are ignored.

## Installation

### Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/aps.svg)](https://repology.org/project/aps/versions)

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a [release asset](https://github.com/Antiz96/aps/releases/latest) (`aps-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org)).
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```bash
git clone https://github.com/Antiz96/aps.git
cd aps
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v1.0.0"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/aps
```

Then, compare the `sha256` hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `aps-<release_version>-x86_64.sha256` file in the release assets). Both hashes should be equal, indicating that the binary has been successfully reproduced.

Each release assets are also cryptographically signed, with the detached signature for each asset distributed as `<asset_name>.asc` (see the [MAINTAINERS.md file](https://github.com/Antiz96/aps/blob/main/MAINTAINERS.md) for a list of keys expected to emit signatures).

### Build from source

```bash
git clone https://gitlab.archlinux.org/antiz/aps.git
cd aps
cargo build --release
```

The built binary will be located at `./target/release/aps`.

The [man page](https://github.com/Antiz96/aps/tree/main/doc/man) can be generated with `scdoc`:

```bash
scdoc < doc/man/aps.1.scd > doc/man/aps.1
```

There are also shell completions available in the [`res/completions/`](https://github.com/Antiz96/aps/tree/main/res/completions) directory.

## Usage

Run `aps` to search for given patterns in AUR packages sources.

It requires a clone of the AUR GitHub mirror. A bare clone is recommended for optimal scanning performance:  
`git clone --bare https://github.com/archlinux/aur.git`

Search patterns are read from a given file, one pattern per line.  
Lines starting with `#` are ignored.

You can optionally provide the list of `pkgbases` to search patterns for.

See `aps --help`, the [aps(1) man page](https://raw.githubusercontent.com/Antiz96/aps/refs/heads/main/doc/man/aps.1.scd) for more details.

## Documentation

See `aps --help`, the [aps(1) man page](https://raw.githubusercontent.com/Antiz96/aps/refs/heads/main/doc/man/aps.1.scd).

## Automated public reports

A periodic run of `aps` (once per hour) is executed on my own infrastructure, searching for the patterns listed in the [patterns.txt file](https://github.com/Antiz96/aps/blob/main/patterns.txt) from this repository (feel free to open a merge requests to add patterns).

Scan results are publicly served at <https://aps.antiz.fr>.

## Contributing

See the [contributing guidelines](https://github.com/Antiz96/aps/blob/main/CONTRIBUTING.md).

## License

APS is licensed under the [GPL-3.0 license](https://github.com/Antiz96/aps/blob/main/LICENSE) (or any later version of that license).
