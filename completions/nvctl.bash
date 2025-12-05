# Bash completion for nvctl
_nvctl() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    local commands="gpu display vibrance fan overclock vrr monitor gaming recording container bolt nvbind drivers driver power color config upscaling dlss shaders passthrough wayland kde power-profile arch gsp monitors osd interactive system doctor version asus"

    case "${prev}" in
        nvctl)
            COMPREPLY=( $(compgen -W "${commands}" -- ${cur}) )
            return 0
            ;;
        gpu)
            COMPREPLY=( $(compgen -W "info stat capabilities list select benchmark watch export stress" -- ${cur}) )
            return 0
            ;;
        display)
            COMPREPLY=( $(compgen -W "info ls vibrance hdr gamma sharpening color-range color-space dithering" -- ${cur}) )
            return 0
            ;;
        vibrance)
            COMPREPLY=( $(compgen -W "get set set-display set-raw list reset info" -- ${cur}) )
            return 0
            ;;
        fan)
            COMPREPLY=( $(compgen -W "info set auto curve profile detect" -- ${cur}) )
            return 0
            ;;
        overclock)
            COMPREPLY=( $(compgen -W "info apply reset profile stress-test auto safe" -- ${cur}) )
            return 0
            ;;
        vrr)
            COMPREPLY=( $(compgen -W "status enable disable toggle configure optimize monitor" -- ${cur}) )
            return 0
            ;;
        monitor)
            COMPREPLY=( $(compgen -W "start stop watch export" -- ${cur}) )
            return 0
            ;;
        gaming)
            COMPREPLY=( $(compgen -W "dashboard latency optimize profiles auto" -- ${cur}) )
            return 0
            ;;
        recording)
            COMPREPLY=( $(compgen -W "start stop status configure" -- ${cur}) )
            return 0
            ;;
        container)
            COMPREPLY=( $(compgen -W "list stats run limits" -- ${cur}) )
            return 0
            ;;
        bolt)
            COMPREPLY=( $(compgen -W "status list show" -- ${cur}) )
            return 0
            ;;
        nvbind)
            COMPREPLY=( $(compgen -W "list start stop metrics" -- ${cur}) )
            return 0
            ;;
        drivers)
            COMPREPLY=( $(compgen -W "status install update rollback generate-completions" -- ${cur}) )
            return 0
            ;;
        driver)
            COMPREPLY=( $(compgen -W "info validate" -- ${cur}) )
            return 0
            ;;
        power)
            COMPREPLY=( $(compgen -W "status profile limit persistence monitor" -- ${cur}) )
            return 0
            ;;
        color)
            COMPREPLY=( $(compgen -W "profiles load reset" -- ${cur}) )
            return 0
            ;;
        config)
            COMPREPLY=( $(compgen -W "show export import reset" -- ${cur}) )
            return 0
            ;;
        upscaling)
            COMPREPLY=( $(compgen -W "list enable disable profiles auto-detect" -- ${cur}) )
            return 0
            ;;
        dlss)
            COMPREPLY=( $(compgen -W "enable disable status" -- ${cur}) )
            return 0
            ;;
        shaders)
            COMPREPLY=( $(compgen -W "cache clear rebuild" -- ${cur}) )
            return 0
            ;;
        passthrough)
            COMPREPLY=( $(compgen -W "status enable disable" -- ${cur}) )
            return 0
            ;;
        wayland)
            COMPREPLY=( $(compgen -W "status switch-driver enable-hdr" -- ${cur}) )
            return 0
            ;;
        kde)
            COMPREPLY=( $(compgen -W "optimize fix-hdr enable-vrr" -- ${cur}) )
            return 0
            ;;
        power-profile)
            COMPREPLY=( $(compgen -W "start stop status" -- ${cur}) )
            return 0
            ;;
        arch)
            COMPREPLY=( $(compgen -W "hooks dkms status" -- ${cur}) )
            return 0
            ;;
        gsp)
            COMPREPLY=( $(compgen -W "status update rollback" -- ${cur}) )
            return 0
            ;;
        monitors)
            COMPREPLY=( $(compgen -W "list layout apply auto" -- ${cur}) )
            return 0
            ;;
        osd)
            COMPREPLY=( $(compgen -W "enable disable status config add remove metrics check" -- ${cur}) )
            return 0
            ;;
        system)
            COMPREPLY=( $(compgen -W "info compositor distro optimize" -- ${cur}) )
            return 0
            ;;
        asus)
            COMPREPLY=( $(compgen -W "detect power status" -- ${cur}) )
            return 0
            ;;
        driver-validate|validate)
            COMPREPLY=( $(compgen -W "--driver" -- ${cur}) )
            return 0
            ;;
        --driver)
            COMPREPLY=( $(compgen -W "550 560 570 580 590" -- ${cur}) )
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

    COMPREPLY=( $(compgen -W "--gpu --verbose --config --help --version --driver" -- ${cur}) )
    return 0
}

complete -F _nvctl nvctl
