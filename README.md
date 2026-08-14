# APS

AUR Pattern Searcher

## Description

An tool allowing to search for specific patterns through the AUR package sources. Useful to get fast, automated and pro-active reports of the presence of knowingly suspicious / malicious patterns in AUR package sources.

## Installation

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a release artifact (`aps-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org/)).  
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```bash
git clone https://gitlab.archlinux.org/antiz/aps.git
cd aps
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v1.0.0"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/aps
```

Then, compare the sha256 hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `aps-<release_version>-x86_64.sha256` file in the release artifacts). Both hashes should be equal, indicating that the binary has been successfully reproduced.

Each release artifacts are also cryptographically signed from the [`FDC3040B92ACA748`](https://keyserver.ubuntu.com/pks/lookup?search=FDC3040B92ACA748&fingerprint=on&op=index) OpenPGP key, with the detached signature for each artifacts distributed as `<asset_name>.asc`.

### Build from source

```bash
git clone https://gitlab.archlinux.org/antiz/aps.git
cd aps
cargo build --release
```

The built binary will be located at `./target/release/aps`.

## Usage

Run `aps` to search for given patterns in AUR packages sources.

It requires a clone of the AUR GitHub mirror.  
A bare clone is recommended for optimal scanning performance:  
`git clone --bare https://github.com/archlinux/aur.git`

Search patterns are read from a given file, one pattern per line.  
Lines starting with `#` are ignored.

```text
Options:
  -r, --repo <path>       Path to the AUR read-only GitHub mirror bare clone (defaults to `$PWD/aur.git` if not set)
  -p, --patterns <path>   Path to the patterns list (defaults to `$PWD/patterns.txt` if not set)
  -f, --fetch             Fetch new changes in the AUR repo clone and update the list of current AUR pkgbases before searching for patterns
  -h, --help              Display this message
  -V, --version           Display version information
```

## Periodic public reports

A periodic run of `aps` (once per hour) is executed on my own infrastructure, searching for the patterns listed in the [patterns.txt file](https://gitlab.archlinux.org/antiz/aps/-/blob/main/patterns.txt) from this repo (feel free to open a merge requests to add patterns).

Scan results are served at <https://aps.antiz.fr>.
