# APS

AUR Pattern Searcher

## Description

## Installation

### Pre-compiled binary

A (statically linked) pre-compiled binary for the `x86_64 (amd64)` architecture is distributed as a release artifact (`aps-<release_version>-x86_64`).

The pre-compiled binary can be reproduced from source (in the sense of [reproducible builds](https://reproducible-builds.org/)).  
The build environment is created and fully documented via [repro-env](https://github.com/kpcyrd/repro-env), and is tracked in this repository.

To reproduce the pre-compiled binary for a given release, [install repro-env](https://github.com/kpcyrd/repro-env#download) and run the following:

```
git clone https://gitlab.archlinux.org/antiz/aps.git
cd aps
git checkout <tag> # Where <tag> is the git tag for the targeted release, e.g. "v1.0.0"
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl
sha256sum target/x86_64-unknown-linux-musl/release/aps
```

Then, compare the sha256 hash of the built binary to the one of the pre-compiled release binary (which is also recorded in the `aps-<release_version>-x86_64.sha256` file in the release assets). Both hashes should be equal, indicating that the binary has been successfully reproduced.

### Build from source

```
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

Options:  
  -r, --repo <path>       Path to the AUR read-only GitHub mirror bare clone (defaults to `$PWD/aur.git` if not set)  
  -p, --patterns <path>   Path to the patterns file list (defaults to `$PWD/patterns.txt` if not set)  
  -f, --fetch             Fetch new changes in the repo clone before searching for patterns  
  -h, --help              Display this message  
  -V, --version           Display version information

## Periodic public reports

A period run of `aps` (once per hour) is executed on my own infrastructure, searching for the patterns listed in the [patterns.txt file from this repo](https://gitlab.archlinux.org/antiz/aps/-/blob/main/patterns.txt) (feel free to open a merge requests to add patterns).  
Results are served at <https://aps.antiz.fr>.
