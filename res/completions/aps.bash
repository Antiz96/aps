_aps() {
	local arg="${2}"
	local -a opts 

	opts=('-r --repo
	       -p --patterns
	       -P --pkgbases
	       -f --fetch
	       -R --refresh-pkgbases
	       -h --help
	       -V --version')

	COMPREPLY=( $(compgen -W "${opts[*]}" -- "${arg}") )
}

complete -F _aps aps
