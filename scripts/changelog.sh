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
	COMMITS="$(git log --pretty --format="%h" ${TAG}..HEAD)"
	for COMMIT in $COMMITS
	do
		COMMIT_DESC="$(git log --pretty --format="%B" -n 1 ${COMMIT})" # Get the description of the commit (NOTE: the -n1 option is required) 
		if [[ $COMMIT_DESC =~ ^Merge* ]] || [[ $COMMIT_DESC =~ ^Bump* ]]
		then
			continue; # We need to ignore merge commits and dependabot commits
		fi
		COMMIT_TYPE="$(echo $COMMIT_DESC | awk -F ': ' '{ print $1 }')"
		COMMIT_INFO="$(echo $COMMIT_DESC | awk -F ': ' '{ print $2 }')"
	done
}
main $@
