# .bash_profile

# Get the aliases and functions
if [ -f ~/.bashrc ]; then
    . ~/.bashrc
fi

# User specific environment and startup programs


# Added by Toolbox App
if [ -d "$HOME/.local/share/JetBrains/Toolbox/scripts" ]; then
    export PATH="$PATH:$HOME/.local/share/JetBrains/Toolbox/scripts"
fi

