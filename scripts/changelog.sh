#!/usr/bin/env bash
# KelpDot's changelog generator
red() {
	echo -e "\e[91m$@\e[m"
}
main(){
	TAG="${1}"
	if [[ -z "${1}" ]]
	then
		TAG="$(git tag | tail -n 2 | head -n 1)" # Get the second last tag name
	fi
	git show "${TAG}" 2> /dev/null &> /dev/null || {
		red "Invalid tag: ${TAG}"
		exit 1
	}
	echo $TAG	
}
main $@
