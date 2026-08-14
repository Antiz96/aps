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
    println!("Search patterns are read from a given file, one pattern per line.");
    println!("Lines starting with `#` are ignored.");
    println!();
    println!("Options:");
    println!(
        "  -r, --repo <path>       Path to the AUR GitHub mirror clone (defaults to `$PWD/aur.git` if not set)"
    );
    println!(
        "  -p, --patterns <path>   Path to the patterns list to search for (defaults to `$PWD/patterns.txt` if not set)"
    );
    println!(
        "  -P, --pkgbases <path>   Path to the pkgbases list to search patterns for (defaults to `$PWD/pkgbases.txt` if not set)"
    );
    println!(
        "                          If the file doesn't exist (or is empty), it is automatically generated with the full list of the current AUR pkgbases"
    );
    println!(
        "  -f, --fetch             Fetch new changes in the AUR repo clone before searching for patterns"
    );
    println!(
        "  -R, --refresh-pkgbases  Refresh the pkgbases list with the full list of the current AUR pkgbases before searching for patterns"
    );
    println!(
        "                          Note that this option will override the pkgbases list file if it already exists"
    );
    println!("  -h, --help              Display this message");
    println!("  -V, --version           Display version information");
}
