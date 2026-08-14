aps(1) ["APS 1.2.0" ["APS Manual"]]

# NAME

*APS* - A fast and powerful patterns searcher for AUR packages sources.

# DESCRIPTION

APS (*A*UR *P*atterns *S*earcher) is a fast and powerful patterns searcher for AUR package sources.
It's a useful tool to get fast, automated and pro-active reports of the presence of knowingly suspicious / malicious patterns in AUR package files.

It requires a clone of the AUR GitHub mirror. A bare clone is recommended for optimal scanning performance: `git clone --bare https://github.com/archlinux/aur.git`.

Search patterns are read from a given file, one pattern per line.  
Lines starting with `#` are ignored.

You can optionally provide the list of `pkgbases` to search patterns for.

# SYNOPSIS

*aps* [OPTION]

# OPTIONS

*-r, --repo* <path>
	Path to the AUR GitHub mirror clone.

*-p, --patterns* <path>
	Path to the patterns list to search for (one per line).
	Defaults to `$PWD/patterns.txt` if not set.

*-P, --pkgbases* <path>
	Path to the pkgbases list to search patterns for (one per line).
	Defaults to `$PWD/pkgbases.txt` if not set.

*-f, --fetch*
	Fetch new changes in the AUR repo clone before searching for patterns.

*-R, --refresh-pkgbases*
	Refresh the pkgbases list with the full list of the current AUR pkgbases before searching for patterns.
	Note that this option will override the pkgbases list file if it already exists.

*-V, --version*
	Display version information.

*-h, --help*
	Display the help message.

# EXIT STATUS

*0* OK.

*1* Failed to access the repository or the director it isn't a git repository.

*2* Failed to access the patterns file or it contains no data.

*3* Failed to access the pkgbases file.

*4* Failed to fetch new changes in the repository.

*5* Failed to download the current AUR pkgbases list.

*6* Failed to scan the repository when searching for patterns.

# BUGS

Please report bugs to the GitHub page: https://github.com/Antiz96/aps

# AUTHOR

Robin Candau <robincandau@protonmail.com>
