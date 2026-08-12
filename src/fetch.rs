//! Fetch changes in the repo clone

use anyhow::Context;
use gix::progress;
use gix::refspec;
use gix::remote::{Direction, ref_map};
use std::sync::atomic::AtomicBool;

pub fn fetch_repo(repo: &gix::Repository) -> anyhow::Result<()> {
    // Set git remote
    let remote = repo
        .find_fetch_remote(None)
        .context("Failed to find Git remote")?;

    // Required by the gix fetch API to support cancellation (see the reference in the .receive call
    // below).
    // For now, we just do not implement cancellation by setting the AtomicBool to false, we'll
    // revise that later if needed.
    let should_interrupt = AtomicBool::new(false);

    // Configure fetch respecs
    let mut ref_options = ref_map::Options::default();
    ref_options.extra_refspecs.push(
        refspec::parse(
            "+refs/heads/*:refs/heads/*".into(),
            refspec::parse::Operation::Fetch,
        )
        .context("Failed to parse fetch refspec")?
        .into(),
    );

    // Fetch changes
    remote
        .connect(Direction::Fetch)
        .context("Failed to connect to Git remote")?
        .prepare_fetch(progress::Discard, ref_options)
        .context("Failed to prepare Git fetch")?
        .receive(progress::Discard, &should_interrupt)
        .context("Failed to fetch Git repository")?;

    Ok(())
}
