# .bashrc

# Source global definitions
if [ -f /etc/bashrc ]; then
    . /etc/bashrc
fi

# User specific environment
if ! [[ "$PATH" =~ "$HOME/.local/bin:$HOME/bin:" ]]; then
    PATH="$HOME/.local/bin:$HOME/bin:$PATH"
fi
export PATH

# Uncomment the following line if you don't like systemctl's auto-paging feature:
# export SYSTEMD_PAGER=

# User specific aliases and functions
if [ -d ~/.bashrc.d ]; then
    for rc in ~/.bashrc.d/*; do
        if [ -f "$rc" ]; then
            . "$rc"
        fi
    done
fi
unset rc

# opencode
if [ -d "$HOME/.opencode/bin" ]; then
    export PATH="$HOME/.opencode/bin:$PATH"
fi

# Fastfetch
fastfetch


# PS1 personalizado
source /usr/share/git-core/contrib/completion/git-prompt.sh

GIT_PS1_SHOWDIRTYSTATE=1
GIT_PS1_SHOWSTASHSTATE=1
GIT_PS1_SHOWUNTRACKEDFILES=1
GIT_PS1_SHOWUPSTREAM="auto"

## Colores
RED="\[\e[38;5;196m\]"
BLUE="\[\e[38;5;33m\]"
WHITE="\[\e[97m\]"
GRAY="\[\e[38;5;245m\]"
GREEN="\[\e[38;5;46m\]"
RESET="\[\e[0m\]"

PS1="${RED}🕸${RESET} \
${WHITE}[${RED}LowNoise${WHITE} │ ${BLUE}BashHunter${WHITE}]${RESET} \
${BLUE}\w${RESET} \
${GRAY}\$(__git_ps1 '(${RED}%s${GRAY})')${RESET}\n\
${RED}❯${RESET} "


set_prompt() {

    local EXIT="$?"

    if [ $EXIT -eq 0 ]; then
        ARROW="\[\e[38;5;46m\]❯"
    else
        ARROW="\[\e[38;5;196m\]❯"
    fi

    PS1="\[\e[38;5;196m\]🕸\[\e[0m\] \
\[\e[97m\][\[\e[38;5;196m\]LowNoise\[\e[97m\] │ \[\e[38;5;33m\]BashHunter\[\e[97m\]]\[\e[0m\] \
\[\e[38;5;33m\]\w\[\e[0m\] \
\[\e[38;5;245m\]\$(__git_ps1 '(\[\e[38;5;196m\]%s\[\e[38;5;245m\])')\[\e[0m\]\n${ARROW}\[\e[0m\] "
}

PROMPT_COMMAND=set_prompt
. "$HOME/.cargo/env"
export PATH="$HOME/.local/bin:$PATH"

