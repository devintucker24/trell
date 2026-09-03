# From zsh: source ./env.zsh  (any cwd; locates env.sh next to this file)
_trell_root="${${(%):-%x}:A:h}"
source "$_trell_root/env.sh"
unset _trell_root
