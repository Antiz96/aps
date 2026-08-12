#!/bin/bash
set -euo pipefail

# Check that we are on the main branch
if [ "$(git branch --show-current)" != "main" ]; then
	echo -e >&2 "ERROR: Current branch is not 'main'"
	exit 1
fi

# Pull repo and fetch tags
git pull
git fetch --tags

# Assign and check tag variables
release_tag="${1}"
latest_tag=$(git describe --tags --abbrev=0)

if [ -z "${release_tag}" ]; then
	echo -e >&2 "\nERROR: Release tag is empty\nUsage: ./scripts/mkrelease.sh X.Y.Z # where 'X.Y.Z' is the tag to create"
	exit 3
fi

echo -e "\nRelease tag = v${release_tag}\nLatest tag = ${latest_tag}\n"
read -rp "Confirm? [y/N] " answer

case "${answer}" in
	y|Y)
		echo -e "\nProceeding in 5 sec"
		sleep 5
	;;
	*)
		echo -e >&2 "\nAborting"
		exit 4
	;;
esac

# Bump version where necessary
sed_pattern="${latest_tag//./\\.}" # escape dots
sed -i "s/version = \"${sed_pattern#v}\"/version = \"${release_tag}\"/g" Cargo.toml
cargo update

# Build binary
rm -rf target/
repro-env update
repro-env build -- cargo build --release --target x86_64-unknown-linux-musl

# Review changes
git diff
echo
read -rp "Confirm? [y/N] " answer

case "${answer}" in
	y|Y)
		echo -e "\nProceeding in 5 sec"
		sleep 5
	;;
	*)
		echo -e >&2 "\nAborting"
		exit 5
	;;
esac

# Create and push a signed commit
git add .
git commit -SFDC3040B92ACA748 -m "chore(release): v${release_tag}"
git push


# Create and push a signed tag
git tag "v${release_tag}" -u FDC3040B92ACA748 -m "v${release_tag}"
git push origin "v${release_tag}"

# Sign binary and checksum
mv target/x86_64-unknown-linux-musl/release/aps "target/x86_64-unknown-linux-musl/release/aps-${release_tag}-x86_64"
gpg --local-user FDC3040B92ACA748 --armor --detach-sign "target/x86_64-unknown-linux-musl/release/aps-${release_tag}-x86_64"
sha256sum "target/x86_64-unknown-linux-musl/release/aps-${release_tag}-x86_64" > "target/x86_64-unknown-linux-musl/release/aps-${release_tag}-x86_64.sha256"
gpg --local-user FDC3040B92ACA748 --armor --detach-sign "target/x86_64-unknown-linux-musl/release/aps-${release_tag}-x86_64.sha256"

# Move artifacts to Download folder (to upload them to the release artifacts)
cp -v "target/x86_64-unknown-linux-musl/release/aps-${release_tag}-x86_64"* ~/Downloads/

# Cleanup
rm -rf target/
podman image prune -af
