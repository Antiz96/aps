complete -c lungo -f

complete -c lungo -s r -l repo -d 'Path to the AUR GitHub mirror clone'
complete -c lungo -s p -l patterns -d 'Path to the patterns list to search for'
complete -c lungo -s P -l pkgbases -d 'Path to the pkgbases list to search patterns for'
complete -c lungo -s f -l fetch -d 'Fetch new changes in the AUR repo clone before searching for patterns'
complete -c lungo -s R -l refresh-pkgbases -d 'Refresh the pkgbases list with the full list of the current AUR pkgbases before searching for patterns'
complete -c lungo -s h -l help -d 'Display the help message'
complete -c lungo -s V -l version -d 'Display version information'
