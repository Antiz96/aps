//! Print help message

pub fn show_help() {
    println!("APS - AUR Pattern Searcher");
    println!();
    println!("Requires a bare clone of https://github.com/archlinux/aur");
    println!();
    println!("`git clone --bare https://github.com/archlinux/aur.git`");
    println!();
    println!("Options:");
    println!("  -r, --repo      Path to the https://github.com/archlinux/aur repo clone");
    println!("  -p, --pattern   Path to the pattern list");
    println!(
        "  -d, --database  Path to the database file (defaults to `$PWD/aps.db` if not set)"
    );
    println!("  -h, --help      Display this message");
    println!("  -V, --version   Display version information");
}
