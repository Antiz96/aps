//! Fetch changes in the repo clone

use anyhow::Context;
use gix::progress;
use gix::remote::Direction;
use std::sync::atomic::AtomicBool;

pub fn fetch_repo(repo: &gix::Repository) -> anyhow::Result<()> {
    let remote = repo
        .find_fetch_remote(None)
        .context("Failed to find Git remote")?;

    // Required by the gix fetch API to support cancellation (see the reference in the .receive call
    // below).
    // For now, we just do not implement cancellation by setting the AtomicBool to false, we'll
    // revise that later if needed.
    let should_interrupt = AtomicBool::new(false);

    remote
        .connect(Direction::Fetch)
        .context("Failed to connect to Git remote")?
        .prepare_fetch(progress::Discard, Default::default())
        .context("Failed to prepare Git fetch")?
        .receive(progress::Discard, &should_interrupt)
        .context("Failed to fetch Git repository")?;

    Ok(())
}
