complete -c aps -f

complete -c aps -s r -l repo -d 'Path to the AUR GitHub mirror clone'
complete -c aps -s p -l patterns -d 'Path to the patterns list to search for'
complete -c aps -s P -l pkgbases -d 'Path to the pkgbases list to search patterns for'
complete -c aps -s f -l fetch -d 'Fetch new changes in the AUR repo clone before searching for patterns'
complete -c aps -s R -l refresh-pkgbases -d 'Refresh the pkgbases list with the full list of the current AUR pkgbases before searching for patterns'
complete -c aps -s h -l help -d 'Display the help message'
complete -c aps -s V -l version -d 'Display version information'
