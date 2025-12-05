#compdef nvctl

# nvctl zsh completion

_nvctl() {
    local curcontext="$curcontext" state line
    typeset -A opt_args

    _arguments -C \
        '1: :->command' \
        '*::arg:->args'

    case $state in
        command)
            _values 'nvctl commands' \
                'gpu[GPU monitoring and control]' \
                'display[Display configuration]' \
                'vibrance[Digital vibrance control]' \
                'vibe[Quick vibrance control]' \
                'overclock[GPU overclocking]' \
                'fan[Fan control]' \
                'power[Power management]' \
                'vrr[Variable Refresh Rate]' \
                'hdr[HDR configuration]' \
                'latency[Latency optimization]' \
                'recording[Screen recording]' \
                'gaming[Gaming optimizations]' \
                'gamescope[Gamescope integration]' \
                'container[Container management]' \
                'drivers[Driver management]' \
                'driver[Driver capabilities & validation]' \
                'kde[KDE Wayland optimizations]' \
                'power-profile[Power profile daemon]' \
                'multimonitor[Multi-monitor management]' \
                'nvbind[nvbind container runtime]' \
                'osd[On-Screen Display]' \
                'color[Color management]' \
                'completions[Generate shell completions]'
            ;;
        args)
            case $line[1] in
                gpu)
                    _values 'gpu subcommands' \
                        'info[Show GPU information]' \
                        'stat[Live GPU statistics]' \
                        'list[List all GPUs]'
                    ;;
                vibrance|vibe)
                    _arguments '1:percentage:(0 50 75 100 125 150 175 200)'
                    ;;
                display)
                    _values 'display subcommands' \
                        'list[List displays]' \
                        'vibrance[Vibrance control]' \
                        'rotation[Display rotation]' \
                        'resolution[Set resolution]'
                    ;;
                overclock)
                    _values 'overclock subcommands' \
                        'apply[Apply overclock settings]' \
                        'reset[Reset to defaults]' \
                        'stress-test[Run stability test]' \
                        'profile[Manage profiles]'
                    ;;
                fan)
                    _values 'fan subcommands' \
                        'info[Fan information]' \
                        'set[Set fan speed]' \
                        'auto[Auto fan control]' \
                        'curve[Custom fan curve]'
                    ;;
                drivers)
                    _values 'driver management' \
                        'status[Driver status]' \
                        'install[Install driver]' \
                        'update[Update driver]' \
                        'rollback[Rollback driver]' \
                        'generate-completions[Shell completion scripts]'
                    ;;
                driver)
                    _values 'driver capability commands' \
                        'info[Show driver capabilities]' \
                        'validate[Validate target driver branch]'
                    ;;
                osd)
                    _values 'osd subcommands' \
                        'enable[Enable OSD]' \
                        'disable[Disable OSD]' \
                        'config[Configure OSD]' \
                        'status[Show OSD status]'
                    ;;
                gaming)
                    _values 'gaming subcommands' \
                        'latency[Latency testing]' \
                        'gamescope[Launch with Gamescope]' \
                        'upscaling[Configure upscaling]'
                    ;;
                container)
                    _values 'container subcommands' \
                        'list[List containers]' \
                        'launch[Launch container]' \
                        'phantomlink[PhantomLink audio]'
                    ;;
            esac
            ;;
    esac
}

_nvctl "$@"
