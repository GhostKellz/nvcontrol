# Bash completion for nvctl
_nvctl() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    local commands="gpu overclock fan power profile display game container vm help version"

    case "${prev}" in
        nvctl)
            COMPREPLY=( $(compgen -W "${commands}" -- ${cur}) )
            return 0
            ;;
        gpu)
            COMPREPLY=( $(compgen -W "info list stats temp" -- ${cur}) )
            return 0
            ;;
        overclock)
            COMPREPLY=( $(compgen -W "apply test profile reset" -- ${cur}) )
            return 0
            ;;
        fan)
            COMPREPLY=( $(compgen -W "set curve auto reset" -- ${cur}) )
            return 0
            ;;
        power)
            COMPREPLY=( $(compgen -W "limit mode battery" -- ${cur}) )
            return 0
            ;;
        profile)
            COMPREPLY=( $(compgen -W "list apply save delete" -- ${cur}) )
            return 0
            ;;
        display)
            COMPREPLY=( $(compgen -W "list vrr hdr vibrance" -- ${cur}) )
            return 0
            ;;
        game)
            COMPREPLY=( $(compgen -W "detect launch optimize" -- ${cur}) )
            return 0
            ;;
        container)
            COMPREPLY=( $(compgen -W "list stats run limits" -- ${cur}) )
            return 0
            ;;
        vm)
            COMPREPLY=( $(compgen -W "passthrough vgpu sriov" -- ${cur}) )
            return 0
            ;;
        --gpu|-g)
            COMPREPLY=( $(compgen -W "0 1 2 3" -- ${cur}) )
            return 0
            ;;
        --format|-f)
            COMPREPLY=( $(compgen -W "json yaml text" -- ${cur}) )
            return 0
            ;;
    esac

    COMPREPLY=( $(compgen -W "--gpu --verbose --config --help --version" -- ${cur}) )
    return 0
}

complete -F _nvctl nvctl
