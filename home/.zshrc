export ZSH="$HOME/.oh-my-zsh"

# Set OMZ Theme to sulfurium
ZSH_THEME="sulfurium"

# Enable ZSH correction
ENABLE_CORRECTION="true"

export PATH="${PATH}:${HOME}/.gem/ruby/2.6.0/bin:${HOME}/.cargo/bin" # Any way to get "latest" version directly ?
# Add "sudo" ZSH plugin
plugins=(sudo)

# Loads OMZ
source $ZSH/oh-my-zsh.sh

# Set $EDITOR 
export EDITOR="nvim"
export MAKEFLAGS="-j$(nproc)"
# Aliases
alias nvim='nvim -p'
alias vim='nvim -p'
alias woman='man'
alias please='doas'
alias goodnight='pkill'
alias systemctl='echo fuck you'
alias boat='cargo'
alias bloat="echo SystemD"
alias gentoo="echo If it moves, compile it"

[[ -s "$HOME/.xmake/profile" ]] && source "$HOME/.xmake/profile" # load xmake profile

