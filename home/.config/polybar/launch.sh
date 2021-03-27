#!/usr/bin/env bash
if [[ "$(ps -ef | grep -i polybar | grep -v grep)" ]]
then
	killall polybar 
	sleep 1
fi
if [[ -f "${HOME}/.config/polybar/colors/${1}.ini" ]]
then
	COLOR_FILE="${HOME}/.config/polybar/colors/${1}.ini"
else 
	WALLPATH="$HOME/.config/polybar/colors"
	FILES=($WALLPATH/*.ini)
	NUM=${#FILES[@]}
	COLOR_FILE="${FILES[RANDOM % NUM]}"
fi	
cp $COLOR_FILE $HOME/.config/polybar/colors.ini
BARS=(top)
for BAR in ${BARS[*]}
do
	nohup polybar -c "$HOME/.config/polybar/config.ini" $BAR > /dev/null &
done

