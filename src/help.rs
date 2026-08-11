//! Print help message

pub fn show_help() {
    println!("APS - AUR Pattern Searcher");
    println!();
    println!("Run the `aps` command to search for given patterns in AUR packages sources.");
    println!();
    println!("It requires a clone of the AUR GitHub mirror.");
    println!("A bare clone is recommended for optimal scanning performance:");
    println!("`git clone --bare https://github.com/archlinux/aur.git`");
    println!();
    println!("Options:");
    println!(
        "  -r, --repo <path>       Path to the AUR read-only GitHub mirror bare clone (defaults to `$PWD/aur.git` if not set)"
    );
    println!(
        "  -p, --patterns <path>   Path to the patterns list (defaults to `$PWD/patterns.txt` if not set)"
    );
    println!(
        "  -f, --fetch             Fetch new changes in the repo clone before searching for patterns"
    );
    println!("  -h, --help              Display this message");
    println!("  -V, --version           Display version information");
}
