#compdef aps

local -a opts
opts=(
    {-r,--repo}'[Path to the AUR GitHub mirror clone]'
    {-p,--patterns}'[Path to the patterns list to search for]'
    {-P,--pkgbases}'[Path to the pkgbases list to search patterns for]'
    {-f,--fetch}'[Fetch new changes in the AUR repo clone before searching for patterns]'
    {-R,--refresh-pkgbases}'[Refresh the pkgbases list with the full list of the current AUR pkgbases before searching for patterns]'
    {-h,--help}'[Display the help message]'
    {-V,--version}'[Display version information]'
)

_arguments $opts
