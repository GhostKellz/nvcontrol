#compdef nvctl

autoload -U is-at-least

_nvctl() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help (see more with '\''--help'\'')]' \
'--help[Print help (see more with '\''--help'\'')]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_nvctl_commands" \
"*::: :->nvctl" \
&& ret=0
    case $state in
    (nvctl)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-command-$line[1]:"
        case $line[1] in
            (gpu)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__gpu_commands" \
"*::: :->gpu" \
&& ret=0

    case $state in
    (gpu)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gpu-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'-f+[Output format\: json, yaml, table]:FORMAT:(human json yaml table)' \
'--format=[Output format\: json, yaml, table]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stat)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'-f+[Output format\: json, yaml, table]:FORMAT:(human json yaml table)' \
'--format=[Output format\: json, yaml, table]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':index -- GPU index to select (0, 1, 2, etc.):_default' \
&& ret=0
;;
(benchmark)
_arguments "${_arguments_options[@]}" : \
'-d+[Benchmark duration in seconds]:DURATION:_default' \
'--duration=[Benchmark duration in seconds]:DURATION:_default' \
'-t+[Test type\: compute, graphics, memory, all]:TEST_TYPE:_default' \
'--test-type=[Test type\: compute, graphics, memory, all]:TEST_TYPE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(watch)
_arguments "${_arguments_options[@]}" : \
'-i+[Update interval in seconds]:INTERVAL:_default' \
'--interval=[Update interval in seconds]:INTERVAL:_default' \
'-c+[Maximum number of updates (0 = infinite)]:COUNT:_default' \
'--count=[Maximum number of updates (0 = infinite)]:COUNT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
'-f+[Output format\: json, csv]:FORMAT:_default' \
'--format=[Output format\: json, csv]:FORMAT:_default' \
'-o+[Output file path]:OUTPUT:_default' \
'--output=[Output file path]:OUTPUT:_default' \
'-d+[Duration to collect data (seconds)]:DURATION:_default' \
'--duration=[Duration to collect data (seconds)]:DURATION:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stress)
_arguments "${_arguments_options[@]}" : \
'-d+[Test duration in minutes]:DURATION:_default' \
'--duration=[Test duration in minutes]:DURATION:_default' \
'-i+[Test intensity\: light, medium, heavy]:INTENSITY:_default' \
'--intensity=[Test intensity\: light, medium, heavy]:INTENSITY:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-l[Monitor and log results]' \
'--log[Monitor and log results]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gpu__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gpu-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stat)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(benchmark)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(watch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(display)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display_commands" \
"*::: :->display" \
&& ret=0

    case $state in
    (display)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(ls)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(vibrance)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__vibrance_commands" \
"*::: :->vibrance" \
&& ret=0

    case $state in
    (vibrance)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-vibrance-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':percentage -- Vibrance percentage (0-200, where 100 is default):_default' \
&& ret=0
;;
(set-display)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display -- Display index (0, 1, 2, etc.):_default' \
':percentage -- Vibrance percentage (0-200):_default' \
&& ret=0
;;
(set-raw)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
'*::levels -- Raw vibrance values for each display (e.g. 512 1023):_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__vibrance__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-vibrance-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-display)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-raw)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(hdr)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__hdr_commands" \
"*::: :->hdr" \
&& ret=0

    case $state in
    (hdr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-hdr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display_id -- Display ID (0, 1, etc.):_default' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display_id -- Display ID (0, 1, etc.):_default' \
&& ret=0
;;
(toggle)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display_id -- Display ID (0, 1, etc.):_default' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__hdr__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-hdr-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(toggle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(gamma)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__gamma_commands" \
"*::: :->gamma" \
&& ret=0

    case $state in
    (gamma)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-gamma-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':gamma -- Gamma value (0.5-3.0):_default' \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__gamma__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-gamma-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(sharpening)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__sharpening_commands" \
"*::: :->sharpening" \
&& ret=0

    case $state in
    (sharpening)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-sharpening-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--value=[Sharpening value (0-100)]:VALUE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__sharpening__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-sharpening-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(color-range)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__color-range_commands" \
"*::: :->color-range" \
&& ret=0

    case $state in
    (color-range)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-color-range-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':range -- Color range\: full, limited:(full limited)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__color-range__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-color-range-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(color-space)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__color-space_commands" \
"*::: :->color-space" \
&& ret=0

    case $state in
    (color-space)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-color-space-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':space -- Color space\: rgb, ycbcr422, ycbcr444:(rgb ycbcr422 ycbcr444)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__color-space__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-color-space-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(dithering)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__display__subcmd__dithering_commands" \
"*::: :->dithering" \
&& ret=0

    case $state in
    (dithering)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-dithering-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--mode=[Dithering mode\: auto, dynamic2x2, static2x2, temporal]:MODE:_default' \
'--depth=[Dithering depth\: auto, 6bit, 8bit]:DEPTH:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--display-id=[Display ID (0, 1, etc.)]:DISPLAY_ID:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__dithering__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-dithering-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(ls)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(vibrance)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__vibrance_commands" \
"*::: :->vibrance" \
&& ret=0

    case $state in
    (vibrance)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-vibrance-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-display)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-raw)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(hdr)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__hdr_commands" \
"*::: :->hdr" \
&& ret=0

    case $state in
    (hdr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-hdr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(toggle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(gamma)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__gamma_commands" \
"*::: :->gamma" \
&& ret=0

    case $state in
    (gamma)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-gamma-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(sharpening)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__sharpening_commands" \
"*::: :->sharpening" \
&& ret=0

    case $state in
    (sharpening)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-sharpening-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(color-range)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__color-range_commands" \
"*::: :->color-range" \
&& ret=0

    case $state in
    (color-range)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-color-range-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(color-space)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__color-space_commands" \
"*::: :->color-space" \
&& ret=0

    case $state in
    (color-space)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-color-space-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(dithering)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__display__subcmd__help__subcmd__dithering_commands" \
"*::: :->dithering" \
&& ret=0

    case $state in
    (dithering)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-display-help-dithering-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(vibrance)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':percentage -- Vibrance percentage (0-200%, where 100% is default):_default' \
&& ret=0
;;
(fan)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__fan_commands" \
"*::: :->fan" \
&& ret=0

    case $state in
    (fan)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-fan-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':fan_id -- Fan ID (0, 1, 2, etc.):_default' \
':percent -- Fan speed percentage (0-100):_default' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__fan__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-fan-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(overclock)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__overclock_commands" \
"*::: :->overclock" \
&& ret=0

    case $state in
    (overclock)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-overclock-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
'--gpu-offset=[GPU clock offset in MHz]:GPU_OFFSET:_default' \
'--memory-offset=[Memory clock offset in MHz]:MEMORY_OFFSET:_default' \
'--power-limit=[Power limit percentage (50-120)]:POWER_LIMIT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(profile)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Profile name to apply:_default' \
&& ret=0
;;
(stress-test)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
'::duration -- Duration in minutes:_default' \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
'--target=[Target mode\: max-performance, balanced, efficiency]:TARGET:_default' \
'--safety=[Safety mode\: conservative, moderate, aggressive]:SAFETY:_default' \
'--max-temp=[Maximum temperature limit in Celsius]:MAX_TEMP:_default' \
'--max-power=[Maximum power limit percentage]:MAX_POWER:_default' \
'--stability-duration=[Stability test duration in seconds]:STABILITY_DURATION:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__overclock__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-overclock-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stress-test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(vrr)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__vrr_commands" \
"*::: :->vrr" \
&& ret=0

    case $state in
    (vrr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-vrr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display -- Display name (e.g. DP-1, HDMI-A-1):_default' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display -- Display name:_default' \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
'--min-refresh=[Minimum refresh rate]:MIN_REFRESH:_default' \
'--max-refresh=[Maximum refresh rate]:MAX_REFRESH:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display -- Display name:_default' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__vrr__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-vrr-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__monitor_commands" \
"*::: :->monitor" \
&& ret=0

    case $state in
    (monitor)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-monitor-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
'-i+[Monitoring interval in seconds]:INTERVAL:_default' \
'--interval=[Monitoring interval in seconds]:INTERVAL:_default' \
'-c+[Number of samples to collect]:COUNT:_default' \
'--count=[Number of samples to collect]:COUNT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(tui)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
'-o+[Output file path]:OUTPUT:_default' \
'--output=[Output file path]:OUTPUT:_default' \
'-d+[Duration to monitor in seconds]:DURATION:_default' \
'--duration=[Duration to monitor in seconds]:DURATION:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__monitor__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-monitor-help-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(tui)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(tui)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(nvtop)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(gaming)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__gaming_commands" \
"*::: :->gaming" \
&& ret=0

    case $state in
    (gaming)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-command-$line[1]:"
        case $line[1] in
            (enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(latency)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__gaming__subcmd__latency_commands" \
"*::: :->latency" \
&& ret=0

    case $state in
    (latency)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-latency-command-$line[1]:"
        case $line[1] in
            (optimize)
_arguments "${_arguments_options[@]}" : \
'-p+[Preset name\: ultra, high, medium, low]:PRESET:_default' \
'--preset=[Preset name\: ultra, high, medium, low]:PRESET:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__latency__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-latency-help-command-$line[1]:"
        case $line[1] in
            (optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(gamescope)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__gaming__subcmd__gamescope_commands" \
"*::: :->gamescope" \
&& ret=0

    case $state in
    (gamescope)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-gamescope-command-$line[1]:"
        case $line[1] in
            (launch)
_arguments "${_arguments_options[@]}" : \
'-c+[Command to run]:COMMAND:_default' \
'--command=[Command to run]:COMMAND:_default' \
'-p+[Preset to use]:PRESET:_default' \
'--preset=[Preset to use]:PRESET:_default' \
'-w+[Window width]:WIDTH:_default' \
'--width=[Window width]:WIDTH:_default' \
'--height=[Window height]:HEIGHT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(create-preset)
_arguments "${_arguments_options[@]}" : \
'-n+[Preset name]:NAME:_default' \
'--name=[Preset name]:NAME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-gamescope-help-command-$line[1]:"
        case $line[1] in
            (launch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(launch)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__gaming__subcmd__launch_commands" \
"*::: :->launch" \
&& ret=0

    case $state in
    (launch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-launch-command-$line[1]:"
        case $line[1] in
            (run)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
'*::args -- Additional arguments to pass to the game:_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(show)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
':executable -- Executable or command to launch:_default' \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
&& ret=0
;;
(hook-add)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
':phase -- Hook phase\: pre or post:_default' \
':command -- Command to run:_default' \
'*::args -- Hook command arguments:_default' \
&& ret=0
;;
(hook-list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
&& ret=0
;;
(hook-remove)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
':phase -- Hook phase\: pre or post:_default' \
':index -- Hook index from `hook-list`:_default' \
&& ret=0
;;
(set-gamescope-preset)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile name:_default' \
':preset -- Preset name:_default' \
&& ret=0
;;
(examples)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__launch__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-launch-help-command-$line[1]:"
        case $line[1] in
            (run)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-gamescope-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(examples)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(auto)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__gaming__subcmd__auto_commands" \
"*::: :->auto" \
&& ret=0

    case $state in
    (auto)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-auto-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(install-service)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(uninstall-service)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable-service)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable-service)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(daemon)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
'--poll-interval=[Poll interval in seconds]:POLL_INTERVAL:_default' \
'--apply-delay=[Apply delay in seconds (anti-crash protection)]:APPLY_DELAY:_default' \
'--restore-on-exit=[Restore default profile on game exit]:RESTORE_ON_EXIT:(true false)' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__auto__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-auto-help-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(uninstall-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(daemon)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-help-command-$line[1]:"
        case $line[1] in
            (enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(latency)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__help__subcmd__latency_commands" \
"*::: :->latency" \
&& ret=0

    case $state in
    (latency)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-help-latency-command-$line[1]:"
        case $line[1] in
            (optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(gamescope)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope_commands" \
"*::: :->gamescope" \
&& ret=0

    case $state in
    (gamescope)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-help-gamescope-command-$line[1]:"
        case $line[1] in
            (launch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(launch)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__help__subcmd__launch_commands" \
"*::: :->launch" \
&& ret=0

    case $state in
    (launch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-help-launch-command-$line[1]:"
        case $line[1] in
            (run)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-gamescope-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(examples)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(auto)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__gaming__subcmd__help__subcmd__auto_commands" \
"*::: :->auto" \
&& ret=0

    case $state in
    (auto)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-gaming-help-auto-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(uninstall-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(daemon)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(recording)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__recording_commands" \
"*::: :->recording" \
&& ret=0

    case $state in
    (recording)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-recording-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
'-o+[Output file path]:OUTPUT:_default' \
'--output=[Output file path]:OUTPUT:_default' \
'-p+[Recording preset]:PRESET:_default' \
'--preset=[Recording preset]:PRESET:_default' \
'-q+[Quality level (1-10)]:QUALITY:_default' \
'--quality=[Quality level (1-10)]:QUALITY:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(instant-replay)
_arguments "${_arguments_options[@]}" : \
'-d+[Buffer duration in seconds]:DURATION:_default' \
'--duration=[Buffer duration in seconds]:DURATION:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__recording__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-recording-help-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(instant-replay)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(container)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__container_commands" \
"*::: :->container" \
&& ret=0

    case $state in
    (container)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'-c+[Container ID or name]:CONTAINER:_default' \
'--container=[Container ID or name]:CONTAINER:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
'-c+[Container ID or name]:CONTAINER:_default' \
'--container=[Container ID or name]:CONTAINER:_default' \
'-i+[Monitoring interval in seconds]:INTERVAL:_default' \
'--interval=[Monitoring interval in seconds]:INTERVAL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(launch)
_arguments "${_arguments_options[@]}" : \
'-m+[Container image]:IMAGE:_default' \
'--image=[Container image]:IMAGE:_default' \
'-n+[Container name]:NAME:_default' \
'--name=[Container name]:NAME:_default' \
'-g+[GPU devices (all, 0, 1,2, GPU-uuid)]:GPU:_default' \
'--gpu=[GPU devices (all, 0, 1,2, GPU-uuid)]:GPU:_default' \
'-r+[Container runtime (docker, podman, nix, containerd)]:RUNTIME:_default' \
'--runtime=[Container runtime (docker, podman, nix, containerd)]:RUNTIME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-i[Interactive mode]' \
'--interactive[Interactive mode]' \
'--rm[Remove container on exit]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(phantom-link)
_arguments "${_arguments_options[@]}" : \
'-m+[Launch mode (dev, prod, minimal)]:MODE:_default' \
'--mode=[Launch mode (dev, prod, minimal)]:MODE:_default' \
'-a+[Audio device]:AUDIO_DEVICE:_default' \
'--audio-device=[Audio device]:AUDIO_DEVICE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--rtx-voice[Enable RTX Voice]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__container__subcmd__profiles_commands" \
"*::: :->profiles" \
&& ret=0

    case $state in
    (profiles)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-profiles-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
'-p+[Profile name]:PROFILE:_default' \
'--profile=[Profile name]:PROFILE:_default' \
'-c+[Container ID or name]:CONTAINER:_default' \
'--container=[Container ID or name]:CONTAINER:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
'-n+[Profile name]:NAME:_default' \
'--name=[Profile name]:NAME:_default' \
'-w+[Workload type (ml-training, inference, gaming, default)]:WORKLOAD:_default' \
'--workload=[Workload type (ml-training, inference, gaming, default)]:WORKLOAD:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__container__subcmd__profiles__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-profiles-help-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(runtime)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__container__subcmd__runtime_commands" \
"*::: :->runtime" \
&& ret=0

    case $state in
    (runtime)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-runtime-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
'-r+[Runtime type to focus on (docker, podman, containerd)]:RUNTIME:_default' \
'--runtime=[Runtime type to focus on (docker, podman, containerd)]:RUNTIME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
'-r+[Runtime type (docker, podman, nix)]:RUNTIME:_default' \
'--runtime=[Runtime type (docker, podman, nix)]:RUNTIME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
'-r+[Runtime type to test (docker, podman, containerd)]:RUNTIME:_default' \
'--runtime=[Runtime type to test (docker, podman, containerd)]:RUNTIME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__container__subcmd__runtime__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-runtime-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__container__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-help-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(launch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(phantom-link)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__container__subcmd__help__subcmd__profiles_commands" \
"*::: :->profiles" \
&& ret=0

    case $state in
    (profiles)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-help-profiles-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(runtime)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__container__subcmd__help__subcmd__runtime_commands" \
"*::: :->runtime" \
&& ret=0

    case $state in
    (runtime)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-container-help-runtime-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(driver)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__driver_commands" \
"*::: :->driver" \
&& ret=0

    case $state in
    (driver)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--paste[Output compact format for pasting (Discord-friendly)]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(check)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
'--driver=[Target driver major version (e.g., 590)]:DRIVER:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diagnose-release)
_arguments "${_arguments_options[@]}" : \
'--format=[Output structured release diagnostics]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(support-bundle)
_arguments "${_arguments_options[@]}" : \
'--output=[Output path for the support bundle report]:OUTPUT:_default' \
'--log-tail=[Number of log lines to include per log section]:LOG_TAIL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--tarball[Package the support bundle and metadata into a tar.gz archive]' \
'--gzip[Gzip the support bundle output]' \
'--redact-paths[Redact local firmware paths in the support bundle]' \
'--redact-ids[Redact PCI and device identifiers in the support bundle]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':driver_type -- Driver type\: proprietary, open, open-beta:_default' \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(rollback)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(dkms)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__driver__subcmd__dkms_commands" \
"*::: :->dkms" \
&& ret=0

    case $state in
    (dkms)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-dkms-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" : \
'-k+[Build for specific kernel only (e.g., 6.18.2-1-cachyos-lto)]:KERNEL:_default' \
'--kernel=[Build for specific kernel only (e.g., 6.18.2-1-cachyos-lto)]:KERNEL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-f[Force rebuild even if already installed]' \
'--force[Force rebuild even if already installed]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(logs)
_arguments "${_arguments_options[@]}" : \
'-k+[Show logs for specific kernel only]:KERNEL:_default' \
'--kernel=[Show logs for specific kernel only]:KERNEL:_default' \
'-t+[Show last N lines of each log (default\: summary only)]:TAIL:_default' \
'--tail=[Show last N lines of each log (default\: summary only)]:TAIL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(unregister)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(hook)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(fix)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(cleanup)
_arguments "${_arguments_options[@]}" : \
'-k+[Number of kernels to keep besides running kernel (default\: 2)]:KEEP:_default' \
'--keep=[Number of kernels to keep besides running kernel (default\: 2)]:KEEP:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--execute[Actually remove (default is dry-run)]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__dkms__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-dkms-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(logs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(unregister)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fix)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cleanup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(gsp)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__driver__subcmd__gsp_commands" \
"*::: :->gsp" \
&& ret=0

    case $state in
    (gsp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-gsp-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diagnostics)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(explain)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(check-update)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__gsp__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-gsp-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diagnostics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(explain)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check-update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(logs)
_arguments "${_arguments_options[@]}" : \
'--filter=[Filter\: nvidia (all), gsp (GSP only), xid (errors only)]:FILTER:_default' \
'--tail=[Show only last N lines]:TAIL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(source)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__driver__subcmd__source_commands" \
"*::: :->source" \
&& ret=0

    case $state in
    (source)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-source-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(init)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':path -- Path to the git clone (e.g., ~/open-gpu-kernel-modules):_default' \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--no-build[Skip rebuild after updating source]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(sync)
_arguments "${_arguments_options[@]}" : \
'-k+[Build for specific kernel only]:KERNEL:_default' \
'--kernel=[Build for specific kernel only]:KERNEL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-f[Force rebuild even if already installed]' \
'--force[Force rebuild even if already installed]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__source__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-source-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(init)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sync)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diagnose-release)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(support-bundle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rollback)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(dkms)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__help__subcmd__dkms_commands" \
"*::: :->dkms" \
&& ret=0

    case $state in
    (dkms)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-help-dkms-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(logs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(unregister)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fix)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cleanup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(gsp)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__help__subcmd__gsp_commands" \
"*::: :->gsp" \
&& ret=0

    case $state in
    (gsp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-help-gsp-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diagnostics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(explain)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check-update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(logs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(source)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__driver__subcmd__help__subcmd__source_commands" \
"*::: :->source" \
&& ret=0

    case $state in
    (source)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-driver-help-source-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(init)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sync)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(power)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__power_commands" \
"*::: :->power" \
&& ret=0

    case $state in
    (power)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(limit)
_arguments "${_arguments_options[@]}" : \
'-p+[Power limit percentage (50-120)]:PERCENTAGE:_default' \
'--percentage=[Power limit percentage (50-120)]:PERCENTAGE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(profile)
_arguments "${_arguments_options[@]}" : \
'-p+[Profile name\: performance, balanced, quiet]:PROFILE:_default' \
'--profile=[Profile name\: performance, balanced, quiet]:PROFILE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(persistence)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--enabled[Enable persistence mode]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
'-d+[Duration to monitor in seconds]:DURATION:_default' \
'--duration=[Duration to monitor in seconds]:DURATION:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(automate)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(curve)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__power__subcmd__curve_commands" \
"*::: :->curve" \
&& ret=0

    case $state in
    (curve)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-curve-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':temp -- Temperature in Celsius:_default' \
':power -- Power limit percentage:_default' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':index -- Point index to remove:_default' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__power__subcmd__curve__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-curve-help-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__power__subcmd__schedule_commands" \
"*::: :->schedule" \
&& ret=0

    case $state in
    (schedule)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-schedule-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
'--hour=[Hour (0-23)]:HOUR:_default' \
'--days=[Weekdays (comma-separated)\: mon,tue,wed,thu,fri,sat,sun or "all"]:DAYS:_default' \
'--power=[Power limit percentage]:POWER:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':index -- Schedule index:_default' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__power__subcmd__schedule__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-schedule-help-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__power__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(limit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(persistence)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(automate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(curve)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__power__subcmd__help__subcmd__curve_commands" \
"*::: :->curve" \
&& ret=0

    case $state in
    (curve)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-help-curve-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__power__subcmd__help__subcmd__schedule_commands" \
"*::: :->schedule" \
&& ret=0

    case $state in
    (schedule)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-help-schedule-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(color)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__color_commands" \
"*::: :->color" \
&& ret=0

    case $state in
    (color)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-command-$line[1]:"
        case $line[1] in
            (vibrance)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__color__subcmd__vibrance_commands" \
"*::: :->vibrance" \
&& ret=0

    case $state in
    (vibrance)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-vibrance-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
'-d+[Display ID (0-based)]:DISPLAY:_default' \
'--display=[Display ID (0-based)]:DISPLAY:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'--value=[Vibrance value (-1024 to 1023)]:VALUE:_default' \
'-d+[Display ID (0-based, all if not specified)]:DISPLAY:_default' \
'--display=[Display ID (0-based, all if not specified)]:DISPLAY:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
'-p+[Profile name]:PROFILE:_default' \
'--profile=[Profile name]:PROFILE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
'-p+[Profile name]:PROFILE:_default' \
'--profile=[Profile name]:PROFILE:_default' \
'-d+[Duration in seconds]:DURATION:_default' \
'--duration=[Duration in seconds]:DURATION:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__color__subcmd__vibrance__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-vibrance-help-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__color__subcmd__profiles_commands" \
"*::: :->profiles" \
&& ret=0

    case $state in
    (profiles)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-profiles-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
'-n+[Profile name]:NAME:_default' \
'--name=[Profile name]:NAME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
'-n+[Profile name]:NAME:_default' \
'--name=[Profile name]:NAME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
'-n+[Profile name]:NAME:_default' \
'--name=[Profile name]:NAME:_default' \
'-t+[Schedule time (HH\:MM format)]:TIME:_default' \
'--time=[Schedule time (HH\:MM format)]:TIME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__color__subcmd__profiles__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-profiles-help-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__color__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-help-command-$line[1]:"
        case $line[1] in
            (vibrance)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__color__subcmd__help__subcmd__vibrance_commands" \
"*::: :->vibrance" \
&& ret=0

    case $state in
    (vibrance)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-help-vibrance-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__color__subcmd__help__subcmd__profiles_commands" \
"*::: :->profiles" \
&& ret=0

    case $state in
    (profiles)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-color-help-profiles-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(config)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__config_commands" \
"*::: :->config" \
&& ret=0

    case $state in
    (config)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-config-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(backup)
_arguments "${_arguments_options[@]}" : \
'-o+[Output file path]:OUTPUT:_default' \
'--output=[Output file path]:OUTPUT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
'-i+[Input file path]:INPUT:_default' \
'--input=[Input file path]:INPUT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
'-p+[Profile name to export]:PROFILE:_default' \
'--profile=[Profile name to export]:PROFILE:_default' \
'-o+[Output file path (JSON or TOML)]:OUTPUT:_default' \
'--output=[Output file path (JSON or TOML)]:OUTPUT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(import)
_arguments "${_arguments_options[@]}" : \
'-i+[Input file path (JSON or TOML)]:INPUT:_default' \
'--input=[Input file path (JSON or TOML)]:INPUT:_default' \
'-n+[Profile name (optional, uses file name if not provided)]:NAME:_default' \
'--name=[Profile name (optional, uses file name if not provided)]:NAME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--skip-validation[Skip safety validation checks]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(capture)
_arguments "${_arguments_options[@]}" : \
'-n+[Saved profile bundle name]:NAME:_default' \
'--name=[Saved profile bundle name]:NAME:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
'-i+[Input file path/name, or \`live\`]:INPUT:_default' \
'--input=[Input file path/name, or \`live\`]:INPUT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
'--current=[Current/base profile path, saved profile name, or \`live\`]:CURRENT:_default' \
'--target=[Target profile path, saved profile name, or \`live\`]:TARGET:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
'-i+[Input file path, saved profile name, or \`live\`]:INPUT:_default' \
'--input=[Input file path, saved profile name, or \`live\`]:INPUT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__config__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-config-help-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(backup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(import)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capture)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(upscaling)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__upscaling_commands" \
"*::: :->upscaling" \
&& ret=0

    case $state in
    (upscaling)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-upscaling-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--tech=[Technology\: dlss, fsr, xess, native]:TECH:_default' \
'--quality=[Quality\: performance, balanced, quality, ultra]:QUALITY:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':game -- Game executable or path:_default' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':game -- Game executable or path:_default' \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(auto-detect)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__upscaling__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-upscaling-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto-detect)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(dlss)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__dlss_commands" \
"*::: :->dlss" \
&& ret=0

    case $state in
    (dlss)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-dlss-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--quality=[Quality preset\: performance, balanced, quality, ultra]:QUALITY:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--frame-generation[Enable Frame Generation (RTX 40+ only)]' \
'--reflex[Enable NVIDIA Reflex]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(metrics)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(games)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--outdated[Only show games with outdated DLSS versions]' \
'--json[Output as JSON]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(versions)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(launch-opts)
_arguments "${_arguments_options[@]}" : \
'--version=[Specific DLSS version to use]:VERSION:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--indicator[Show DLSS version indicator overlay]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':game -- Steam App ID or game name:_default' \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':game -- Steam App ID or game name:_default' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__dlss__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-dlss-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(metrics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(games)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(versions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(launch-opts)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(shaders)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__shaders_commands" \
"*::: :->shaders" \
&& ret=0

    case $state in
    (shaders)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-shaders-command-$line[1]:"
        case $line[1] in
            (stats)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(clear)
_arguments "${_arguments_options[@]}" : \
'--cache-type=[Cache type to clear\: nvidia, vulkan, steam, dxvk, all]:CACHE_TYPE:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(precompile)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':game -- Game path or Steam App ID:_default' \
&& ret=0
;;
(open)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__shaders__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-shaders-help-command-$line[1]:"
        case $line[1] in
            (stats)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(clear)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(precompile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(open)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(passthrough)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__passthrough_commands" \
"*::: :->passthrough" \
&& ret=0

    case $state in
    (passthrough)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-passthrough-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(iommu)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(bind-vfio)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':pci_address -- PCI address (e.g., 0000\:01\:00.0):_default' \
&& ret=0
;;
(unbind-vfio)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':pci_address -- PCI address (e.g., 0000\:01\:00.0):_default' \
&& ret=0
;;
(persistent)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':pci_address -- PCI address (e.g., 0000\:01\:00.0):_default' \
&& ret=0
;;
(test-container)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(qemu-command)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':pci_address -- PCI address (e.g., 0000\:01\:00.0):_default' \
&& ret=0
;;
(hugepages)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
'::size_mb -- Size in MB:_default' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__passthrough__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-passthrough-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(iommu)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(bind-vfio)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(unbind-vfio)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(persistent)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test-container)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(qemu-command)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hugepages)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(wayland)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__wayland_commands" \
"*::: :->wayland" \
&& ret=0

    case $state in
    (wayland)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-wayland-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--backup[Create backup before applying]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(export-env)
_arguments "${_arguments_options[@]}" : \
'--config=[Shell config file path]:CONFIG:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(switch-driver)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':driver -- Target driver\: open, dkms:_default' \
&& ret=0
;;
(explicit-sync)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__wayland__subcmd__explicit-sync_commands" \
"*::: :->explicit-sync" \
&& ret=0

    case $state in
    (explicit-sync)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-wayland-explicit-sync-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-wayland-explicit-sync-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__wayland__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-wayland-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export-env)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(switch-driver)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(explicit-sync)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync_commands" \
"*::: :->explicit-sync" \
&& ret=0

    case $state in
    (explicit-sync)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-wayland-help-explicit-sync-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(kde)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__kde_commands" \
"*::: :->kde" \
&& ret=0

    case $state in
    (kde)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-kde-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(gaming)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(productivity)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(power-save)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(setup-env)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set-vrr)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--enabled[Enable or disable]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':display -- Display connector (e.g., DP-1):_default' \
&& ret=0
;;
(restart)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__kde__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-kde-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(gaming)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(productivity)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(power-save)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup-env)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-vrr)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restart)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(hdr)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__hdr_commands" \
"*::: :->hdr" \
&& ret=0

    case $state in
    (hdr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-hdr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set-brightness)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':nits -- Peak luminance in nits (400-10000):_default' \
&& ret=0
;;
(tools)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__hdr__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-hdr-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-brightness)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(tools)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(power-profile)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__power-profile_commands" \
"*::: :->power-profile" \
&& ret=0

    case $state in
    (power-profile)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-profile-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':profile -- Profile\: performance, balanced, power-saver:_default' \
&& ret=0
;;
(create-activity)
_arguments "${_arguments_options[@]}" : \
'--system-profile=[System profile\: performance, balanced, power-saver]:SYSTEM_PROFILE:_default' \
'--gpu-offset=[GPU clock offset in MHz]:GPU_OFFSET:_default' \
'--mem-offset=[Memory clock offset in MHz]:MEM_OFFSET:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':activity -- KDE Activity name:_default' \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':activity -- Activity name:_default' \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(auto-power)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(idle)
_arguments "${_arguments_options[@]}" : \
'--timeout=[Idle timeout in seconds]:TIMEOUT:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(create-defaults)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__power-profile__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-power-profile-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-activity)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto-power)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(idle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-defaults)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(arch)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__arch_commands" \
"*::: :->arch" \
&& ret=0

    case $state in
    (arch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-arch-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(install-hooks)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(remove-hooks)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(rebuild-dkms)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(mkinitcpio)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(check-updates)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(aur-suggestions)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__arch__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-arch-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install-hooks)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove-hooks)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rebuild-dkms)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mkinitcpio)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check-updates)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(aur-suggestions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(monitors)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__monitors_commands" \
"*::: :->monitors" \
&& ret=0

    case $state in
    (monitors)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-monitors-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':preset -- Preset key (for example\: dual_oled_ips):_default' \
&& ret=0
;;
(apply-preset)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':preset -- Preset key (for example\: dual_oled_ips):_default' \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Layout name:_default' \
&& ret=0
;;
(load)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Layout name:_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(set-vrr)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--enabled[Enable or disable]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':connector -- Display connector (e.g., DP-1):_default' \
&& ret=0
;;
(gamescope)
_arguments "${_arguments_options[@]}" : \
'-w+[Width]:WIDTH:_default' \
'--width=[Width]:WIDTH:_default' \
'-H+[Height]:HEIGHT:_default' \
'--height=[Height]:HEIGHT:_default' \
'-r+[Refresh rate]:REFRESH:_default' \
'--refresh=[Refresh rate]:REFRESH:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':connector -- Display connector:_default' \
':command -- Command to run:_default' \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(create-examples)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__monitors__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-monitors-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(load)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-vrr)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(gamescope)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-examples)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(osd)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__osd_commands" \
"*::: :->osd" \
&& ret=0

    case $state in
    (osd)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-osd-command-$line[1]:"
        case $line[1] in
            (enable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
'--position=[Position\: top-left, top-right, bottom-left, bottom-right]:POSITION:_default' \
'--font-size=[Font size]:FONT_SIZE:_default' \
'--opacity=[Background opacity (0.0-1.0)]:OPACITY:_default' \
'--interval=[Update interval in milliseconds]:INTERVAL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':metric -- Metric to add\: fps, gpu-temp, gpu-util, vram, etc:_default' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':metric -- Metric to remove:_default' \
&& ret=0
;;
(metrics)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(check)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__osd__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-osd-help-command-$line[1]:"
        case $line[1] in
            (enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(metrics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(interactive)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(system)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__system_commands" \
"*::: :->system" \
&& ret=0

    case $state in
    (system)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-system-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(compositor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(distro)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__system__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-system-help-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(compositor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(distro)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
'--format=[Output structured doctor summary]:FORMAT:(human json yaml table)' \
'--output=[Output path for doctor support bundles]:OUTPUT:_default' \
'--support[Run support-focused diagnostics and write a support bundle]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(completion)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':shell -- Shell type\: bash, zsh, fish:(bash zsh fish)' \
&& ret=0
;;
(version)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(asus)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__asus_commands" \
"*::: :->asus" \
&& ret=0

    case $state in
    (asus)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-asus-command-$line[1]:"
        case $line[1] in
            (detect)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(power)
_arguments "${_arguments_options[@]}" : \
'-g+[GPU PCI ID (default\: auto-detect)]:GPU:_default' \
'--gpu=[GPU PCI ID (default\: auto-detect)]:GPU:_default' \
'--interval=[Watch interval in seconds]:INTERVAL:_default' \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--json[Output as JSON]' \
'-w[Watch mode - continuous monitoring]' \
'--watch[Watch mode - continuous monitoring]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(aura)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__asus__subcmd__aura_commands" \
"*::: :->aura" \
&& ret=0

    case $state in
    (aura)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-asus-aura-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(mode)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':mode -- Mode\: off, static, breathing, rainbow, cycle, cyberpunk, purple, performance, silent:_default' \
&& ret=0
;;
(color)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
':color -- RGB hex color (e.g., FF0000 for red):_default' \
&& ret=0
;;
(temp-reactive)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'--enabled[Enable (true) or disable (false)]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__asus__subcmd__aura__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-asus-aura-help-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mode)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(color)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(temp-reactive)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__asus__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-asus-help-command-$line[1]:"
        case $line[1] in
            (detect)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(power)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(aura)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__asus__subcmd__help__subcmd__aura_commands" \
"*::: :->aura" \
&& ret=0

    case $state in
    (aura)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-asus-help-aura-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mode)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(color)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(temp-reactive)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(companion)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_nvctl__subcmd__companion_commands" \
"*::: :->companion" \
&& ret=0

    case $state in
    (companion)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-companion-command-$line[1]:"
        case $line[1] in
            (notify-test)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(open-docs)
_arguments "${_arguments_options[@]}" : \
'--format=[Output format]:FORMAT:(human json yaml table)' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'--no-color[Disable colored output]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__companion__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-companion-help-command-$line[1]:"
        case $line[1] in
            (notify-test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(open-docs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-command-$line[1]:"
        case $line[1] in
            (gpu)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__gpu_commands" \
"*::: :->gpu" \
&& ret=0

    case $state in
    (gpu)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-gpu-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stat)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(select)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(benchmark)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(watch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stress)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(display)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display_commands" \
"*::: :->display" \
&& ret=0

    case $state in
    (display)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(ls)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(vibrance)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__vibrance_commands" \
"*::: :->vibrance" \
&& ret=0

    case $state in
    (vibrance)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-vibrance-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-display)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-raw)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(hdr)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__hdr_commands" \
"*::: :->hdr" \
&& ret=0

    case $state in
    (hdr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-hdr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(toggle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(gamma)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__gamma_commands" \
"*::: :->gamma" \
&& ret=0

    case $state in
    (gamma)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-gamma-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(sharpening)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__sharpening_commands" \
"*::: :->sharpening" \
&& ret=0

    case $state in
    (sharpening)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-sharpening-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(color-range)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__color-range_commands" \
"*::: :->color-range" \
&& ret=0

    case $state in
    (color-range)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-color-range-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(color-space)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__color-space_commands" \
"*::: :->color-space" \
&& ret=0

    case $state in
    (color-space)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-color-space-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(dithering)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__display__subcmd__dithering_commands" \
"*::: :->dithering" \
&& ret=0

    case $state in
    (dithering)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-display-dithering-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(vibrance)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fan)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__fan_commands" \
"*::: :->fan" \
&& ret=0

    case $state in
    (fan)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-fan-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(overclock)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__overclock_commands" \
"*::: :->overclock" \
&& ret=0

    case $state in
    (overclock)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-overclock-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stress-test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(vrr)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__vrr_commands" \
"*::: :->vrr" \
&& ret=0

    case $state in
    (vrr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-vrr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__monitor_commands" \
"*::: :->monitor" \
&& ret=0

    case $state in
    (monitor)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-monitor-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(tui)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(tui)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(nvtop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(gaming)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__gaming_commands" \
"*::: :->gaming" \
&& ret=0

    case $state in
    (gaming)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-gaming-command-$line[1]:"
        case $line[1] in
            (enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(latency)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__gaming__subcmd__latency_commands" \
"*::: :->latency" \
&& ret=0

    case $state in
    (latency)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-gaming-latency-command-$line[1]:"
        case $line[1] in
            (optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(gamescope)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope_commands" \
"*::: :->gamescope" \
&& ret=0

    case $state in
    (gamescope)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-gaming-gamescope-command-$line[1]:"
        case $line[1] in
            (launch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(launch)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__gaming__subcmd__launch_commands" \
"*::: :->launch" \
&& ret=0

    case $state in
    (launch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-gaming-launch-command-$line[1]:"
        case $line[1] in
            (run)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(delete)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook-remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-gamescope-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(examples)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(auto)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__gaming__subcmd__auto_commands" \
"*::: :->auto" \
&& ret=0

    case $state in
    (auto)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-gaming-auto-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(uninstall-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable-service)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(daemon)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(recording)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__recording_commands" \
"*::: :->recording" \
&& ret=0

    case $state in
    (recording)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-recording-command-$line[1]:"
        case $line[1] in
            (start)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(stop)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(instant-replay)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(container)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__container_commands" \
"*::: :->container" \
&& ret=0

    case $state in
    (container)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-container-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(launch)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(phantom-link)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__container__subcmd__profiles_commands" \
"*::: :->profiles" \
&& ret=0

    case $state in
    (profiles)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-container-profiles-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(runtime)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__container__subcmd__runtime_commands" \
"*::: :->runtime" \
&& ret=0

    case $state in
    (runtime)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-container-runtime-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(configure)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(driver)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__driver_commands" \
"*::: :->driver" \
&& ret=0

    case $state in
    (driver)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-driver-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(validate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diagnose-release)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(support-bundle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rollback)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(dkms)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__driver__subcmd__dkms_commands" \
"*::: :->dkms" \
&& ret=0

    case $state in
    (dkms)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-driver-dkms-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(build)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(logs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(unregister)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hook)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(fix)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(cleanup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(gsp)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__driver__subcmd__gsp_commands" \
"*::: :->gsp" \
&& ret=0

    case $state in
    (gsp)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-driver-gsp-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diagnostics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(explain)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check-update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(logs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(source)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__driver__subcmd__source_commands" \
"*::: :->source" \
&& ret=0

    case $state in
    (source)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-driver-source-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(init)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(sync)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(power)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__power_commands" \
"*::: :->power" \
&& ret=0

    case $state in
    (power)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-power-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(limit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(persistence)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(automate)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(curve)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__power__subcmd__curve_commands" \
"*::: :->curve" \
&& ret=0

    case $state in
    (curve)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-power-curve-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__power__subcmd__schedule_commands" \
"*::: :->schedule" \
&& ret=0

    case $state in
    (schedule)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-power-schedule-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(color)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__color_commands" \
"*::: :->color" \
&& ret=0

    case $state in
    (color)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-color-command-$line[1]:"
        case $line[1] in
            (vibrance)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__color__subcmd__vibrance_commands" \
"*::: :->vibrance" \
&& ret=0

    case $state in
    (vibrance)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-color-vibrance-command-$line[1]:"
        case $line[1] in
            (get)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__color__subcmd__profiles_commands" \
"*::: :->profiles" \
&& ret=0

    case $state in
    (profiles)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-color-profiles-command-$line[1]:"
        case $line[1] in
            (list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(schedule)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(config)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__config_commands" \
"*::: :->config" \
&& ret=0

    case $state in
    (config)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-config-command-$line[1]:"
        case $line[1] in
            (show)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(edit)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(reset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(backup)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(import)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capture)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(upscaling)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__upscaling_commands" \
"*::: :->upscaling" \
&& ret=0

    case $state in
    (upscaling)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-upscaling-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto-detect)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(dlss)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__dlss_commands" \
"*::: :->dlss" \
&& ret=0

    case $state in
    (dlss)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-dlss-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(profiles)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(metrics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(games)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(versions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(launch-opts)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(shaders)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__shaders_commands" \
"*::: :->shaders" \
&& ret=0

    case $state in
    (shaders)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-shaders-command-$line[1]:"
        case $line[1] in
            (stats)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(clear)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(precompile)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(open)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(passthrough)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__passthrough_commands" \
"*::: :->passthrough" \
&& ret=0

    case $state in
    (passthrough)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-passthrough-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(iommu)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(bind-vfio)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(unbind-vfio)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(persistent)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(test-container)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(qemu-command)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(hugepages)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(wayland)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__wayland_commands" \
"*::: :->wayland" \
&& ret=0

    case $state in
    (wayland)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-wayland-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(export-env)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(switch-driver)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(explicit-sync)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync_commands" \
"*::: :->explicit-sync" \
&& ret=0

    case $state in
    (explicit-sync)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-wayland-explicit-sync-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(kde)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__kde_commands" \
"*::: :->kde" \
&& ret=0

    case $state in
    (kde)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-kde-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(gaming)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(productivity)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(power-save)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(setup-env)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-vrr)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restart)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(hdr)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__hdr_commands" \
"*::: :->hdr" \
&& ret=0

    case $state in
    (hdr)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-hdr-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-brightness)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(tools)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(capabilities)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(power-profile)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__power-profile_commands" \
"*::: :->power-profile" \
&& ret=0

    case $state in
    (power-profile)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-power-profile-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-activity)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(monitor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto-power)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(idle)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-defaults)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(arch)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__arch_commands" \
"*::: :->arch" \
&& ret=0

    case $state in
    (arch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-arch-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install-hooks)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove-hooks)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(rebuild-dkms)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mkinitcpio)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check-updates)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(aur-suggestions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(monitors)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__monitors_commands" \
"*::: :->monitors" \
&& ret=0

    case $state in
    (monitors)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-monitors-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(presets)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(suggest)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(preview)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(apply-preset)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(save)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(load)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(set-vrr)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(gamescope)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(auto)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(create-examples)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(osd)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__osd_commands" \
"*::: :->osd" \
&& ret=0

    case $state in
    (osd)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-osd-command-$line[1]:"
        case $line[1] in
            (enable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(disable)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(config)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(metrics)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(check)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(interactive)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(system)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__system_commands" \
"*::: :->system" \
&& ret=0

    case $state in
    (system)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-system-command-$line[1]:"
        case $line[1] in
            (info)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(compositor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(distro)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(optimize)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(doctor)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(completion)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(version)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(asus)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__asus_commands" \
"*::: :->asus" \
&& ret=0

    case $state in
    (asus)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-asus-command-$line[1]:"
        case $line[1] in
            (detect)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(power)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(aura)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__asus__subcmd__aura_commands" \
"*::: :->aura" \
&& ret=0

    case $state in
    (aura)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-asus-aura-command-$line[1]:"
        case $line[1] in
            (status)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(mode)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(color)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(temp-reactive)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(restore)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(companion)
_arguments "${_arguments_options[@]}" : \
":: :_nvctl__subcmd__help__subcmd__companion_commands" \
"*::: :->companion" \
&& ret=0

    case $state in
    (companion)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:nvctl-help-companion-command-$line[1]:"
        case $line[1] in
            (notify-test)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(open-docs)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_nvctl_commands] )) ||
_nvctl_commands() {
    local commands; commands=(
'gpu:🎮 GPU information and control' \
'display:🖥️ Display and monitor management' \
'vibrance:🌈 Digital vibrance control (0-200%)' \
'fan:🌀 Fan control and curves' \
'overclock:⚡ Overclocking and performance' \
'vrr:🔄 Variable Refresh Rate (VRR/G-Sync)' \
'monitor:📊 Real-time monitoring' \
'tui:📺 Terminal user interface' \
'nvtop:🖥️ GPU monitor (htop-style)' \
'gaming:🎯 Gaming optimization and latency' \
'recording:📹 Recording and streaming' \
'container:🐳 Container and virtualization workflows' \
'driver:🧠 Driver management, status, and kernel modules' \
'power:⚡ Power management' \
'color:🎨 Color and vibrance control' \
'config:⚙️ Configuration and profiles' \
'upscaling:📈 AI upscaling and enhancement' \
'dlss:🚀 DLSS and related features' \
'shaders:🎨 Shader cache management' \
'passthrough:🔌 GPU passthrough (VFIO/containers/VMs)' \
'wayland:🌊 Wayland NVIDIA optimization' \
'kde:🎨 KDE Plasma compositor optimization' \
'hdr:🌈 HDR control and configuration' \
'power-profile:⚡ Power profile management (AC/battery, activities)' \
'arch:🐧 Arch Linux integration (pacman hooks, DKMS)' \
'monitors:🖥️ Multi-monitor management' \
'osd:📊 On-screen display for gaming' \
'interactive:🎛️ Interactive menu mode' \
'system:💻 System information and platform detection' \
'doctor:🔍 Run system diagnostics' \
'completion:🧩 Generate shell completions' \
'version:📋 Show detailed version information' \
'asus:🎯 ASUS ROG GPU features (Power Detector+, Aura, etc.)' \
'companion:🔔 Lightweight desktop companion actions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch_commands] )) ||
_nvctl__subcmd__arch_commands() {
    local commands; commands=(
'status:Show Arch Linux NVIDIA integration status' \
'install-hooks:Install all pacman hooks' \
'remove-hooks:Remove pacman hooks' \
'rebuild-dkms:Rebuild DKMS modules' \
'mkinitcpio:Regenerate initramfs' \
'check-updates:Check for pending NVIDIA/kernel updates' \
'aur-suggestions:List AUR optimization suggestions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl arch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__aur-suggestions_commands] )) ||
_nvctl__subcmd__arch__subcmd__aur-suggestions_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch aur-suggestions commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__check-updates_commands] )) ||
_nvctl__subcmd__arch__subcmd__check-updates_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch check-updates commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help_commands] )) ||
_nvctl__subcmd__arch__subcmd__help_commands() {
    local commands; commands=(
'status:Show Arch Linux NVIDIA integration status' \
'install-hooks:Install all pacman hooks' \
'remove-hooks:Remove pacman hooks' \
'rebuild-dkms:Rebuild DKMS modules' \
'mkinitcpio:Regenerate initramfs' \
'check-updates:Check for pending NVIDIA/kernel updates' \
'aur-suggestions:List AUR optimization suggestions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl arch help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__aur-suggestions_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__aur-suggestions_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help aur-suggestions commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__check-updates_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__check-updates_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help check-updates commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__install-hooks_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__install-hooks_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help install-hooks commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__mkinitcpio_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__mkinitcpio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help mkinitcpio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__rebuild-dkms_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__rebuild-dkms_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help rebuild-dkms commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__remove-hooks_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__remove-hooks_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help remove-hooks commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__arch__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__install-hooks_commands] )) ||
_nvctl__subcmd__arch__subcmd__install-hooks_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch install-hooks commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__mkinitcpio_commands] )) ||
_nvctl__subcmd__arch__subcmd__mkinitcpio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch mkinitcpio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__rebuild-dkms_commands] )) ||
_nvctl__subcmd__arch__subcmd__rebuild-dkms_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch rebuild-dkms commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__remove-hooks_commands] )) ||
_nvctl__subcmd__arch__subcmd__remove-hooks_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch remove-hooks commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__arch__subcmd__status_commands] )) ||
_nvctl__subcmd__arch__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl arch status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus_commands] )) ||
_nvctl__subcmd__asus_commands() {
    local commands; commands=(
'detect:Detect ASUS ROG GPUs in system' \
'power:Show Power Detector+ status (12V-2x6 connector monitoring)' \
'status:Show ASUS GPU Tweak-style status' \
'aura:ASUS Aura RGB control' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl asus commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura_commands() {
    local commands; commands=(
'status:Show Aura status' \
'mode:Set Aura mode (off, static, breathing, rainbow, cycle, cyberpunk, purple, performance, silent)' \
'color:Set Aura color (static mode)' \
'temp-reactive:Enable/disable temperature-reactive RGB (color changes with GPU temp)' \
'restore:Restore saved Aura configuration from config file' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl asus aura commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__color_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__color_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura color commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help_commands() {
    local commands; commands=(
'status:Show Aura status' \
'mode:Set Aura mode (off, static, breathing, rainbow, cycle, cyberpunk, purple, performance, silent)' \
'color:Set Aura color (static mode)' \
'temp-reactive:Enable/disable temperature-reactive RGB (color changes with GPU temp)' \
'restore:Restore saved Aura configuration from config file' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl asus aura help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__color_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__color_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura help color commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__mode_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__mode_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura help mode commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__restore_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura help restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__temp-reactive_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__help__subcmd__temp-reactive_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura help temp-reactive commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__mode_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__mode_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura mode commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__restore_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__status_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__aura__subcmd__temp-reactive_commands] )) ||
_nvctl__subcmd__asus__subcmd__aura__subcmd__temp-reactive_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus aura temp-reactive commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__detect_commands] )) ||
_nvctl__subcmd__asus__subcmd__detect_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus detect commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help_commands] )) ||
_nvctl__subcmd__asus__subcmd__help_commands() {
    local commands; commands=(
'detect:Detect ASUS ROG GPUs in system' \
'power:Show Power Detector+ status (12V-2x6 connector monitoring)' \
'status:Show ASUS GPU Tweak-style status' \
'aura:ASUS Aura RGB control' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl asus help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__aura_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__aura_commands() {
    local commands; commands=(
'status:Show Aura status' \
'mode:Set Aura mode (off, static, breathing, rainbow, cycle, cyberpunk, purple, performance, silent)' \
'color:Set Aura color (static mode)' \
'temp-reactive:Enable/disable temperature-reactive RGB (color changes with GPU temp)' \
'restore:Restore saved Aura configuration from config file' \
    )
    _describe -t commands 'nvctl asus help aura commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__color_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__color_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help aura color commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__mode_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__mode_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help aura mode commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__restore_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help aura restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__status_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help aura status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__temp-reactive_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__aura__subcmd__temp-reactive_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help aura temp-reactive commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__detect_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__detect_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help detect commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__power_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__power_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__asus__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__power_commands] )) ||
_nvctl__subcmd__asus__subcmd__power_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__asus__subcmd__status_commands] )) ||
_nvctl__subcmd__asus__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl asus status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color_commands] )) ||
_nvctl__subcmd__color_commands() {
    local commands; commands=(
'vibrance:Vibrance control' \
'profiles:Color profile management' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl color commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help_commands] )) ||
_nvctl__subcmd__color__subcmd__help_commands() {
    local commands; commands=(
'vibrance:Vibrance control' \
'profiles:Color profile management' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl color help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__profiles_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__profiles_commands() {
    local commands; commands=(
'list:List available color profiles' \
'create:Create new color profile' \
'apply:Apply color profile' \
'schedule:Schedule color profile' \
    )
    _describe -t commands 'nvctl color help profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__apply_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help profiles apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__create_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help profiles create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__list_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help profiles list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__schedule_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__profiles__subcmd__schedule_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help profiles schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__vibrance_commands() {
    local commands; commands=(
'get:Get current vibrance for a display' \
'set:Set vibrance for a display' \
'apply:Apply vibrance profile' \
'preview:Preview vibrance changes' \
    )
    _describe -t commands 'nvctl color help vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__apply_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help vibrance apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__get_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help vibrance get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__preview_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help vibrance preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__set_commands] )) ||
_nvctl__subcmd__color__subcmd__help__subcmd__vibrance__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color help vibrance set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles_commands() {
    local commands; commands=(
'list:List available color profiles' \
'create:Create new color profile' \
'apply:Apply color profile' \
'schedule:Schedule color profile' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl color profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__apply_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__create_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__help_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__help_commands() {
    local commands; commands=(
'list:List available color profiles' \
'create:Create new color profile' \
'apply:Apply color profile' \
'schedule:Schedule color profile' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl color profiles help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__apply_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles help apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__create_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles help create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__schedule_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__help__subcmd__schedule_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles help schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__list_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__profiles__subcmd__schedule_commands] )) ||
_nvctl__subcmd__color__subcmd__profiles__subcmd__schedule_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color profiles schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance_commands() {
    local commands; commands=(
'get:Get current vibrance for a display' \
'set:Set vibrance for a display' \
'apply:Apply vibrance profile' \
'preview:Preview vibrance changes' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl color vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__apply_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__get_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__help_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__help_commands() {
    local commands; commands=(
'get:Get current vibrance for a display' \
'set:Set vibrance for a display' \
'apply:Apply vibrance profile' \
'preview:Preview vibrance changes' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl color vibrance help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__apply_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance help apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__preview_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance help preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__preview_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__color__subcmd__vibrance__subcmd__set_commands] )) ||
_nvctl__subcmd__color__subcmd__vibrance__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl color vibrance set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion_commands] )) ||
_nvctl__subcmd__companion_commands() {
    local commands; commands=(
'notify-test:Send a desktop notification test' \
'open-docs:Open the project documentation in the default desktop handler' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl companion commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion__subcmd__help_commands] )) ||
_nvctl__subcmd__companion__subcmd__help_commands() {
    local commands; commands=(
'notify-test:Send a desktop notification test' \
'open-docs:Open the project documentation in the default desktop handler' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl companion help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__companion__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl companion help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion__subcmd__help__subcmd__notify-test_commands] )) ||
_nvctl__subcmd__companion__subcmd__help__subcmd__notify-test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl companion help notify-test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion__subcmd__help__subcmd__open-docs_commands] )) ||
_nvctl__subcmd__companion__subcmd__help__subcmd__open-docs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl companion help open-docs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion__subcmd__notify-test_commands] )) ||
_nvctl__subcmd__companion__subcmd__notify-test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl companion notify-test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__companion__subcmd__open-docs_commands] )) ||
_nvctl__subcmd__companion__subcmd__open-docs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl companion open-docs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__completion_commands] )) ||
_nvctl__subcmd__completion_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl completion commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config_commands] )) ||
_nvctl__subcmd__config_commands() {
    local commands; commands=(
'show:Show current configuration' \
'edit:Edit configuration file' \
'reset:Reset configuration to defaults' \
'backup:Backup configuration' \
'restore:Restore configuration from backup' \
'export:Export GPU profile to file' \
'import:Import GPU profile from file' \
'capture:Capture the current live state into a saved profile bundle' \
'preview:Preview a profile bundle from disk' \
'diff:Diff two profile bundles' \
'apply:Apply a saved bundle or live snapshot-compatible bundle' \
'profiles:List available profiles' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__apply_commands] )) ||
_nvctl__subcmd__config__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__backup_commands] )) ||
_nvctl__subcmd__config__subcmd__backup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config backup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__capture_commands] )) ||
_nvctl__subcmd__config__subcmd__capture_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config capture commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__diff_commands] )) ||
_nvctl__subcmd__config__subcmd__diff_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config diff commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__edit_commands] )) ||
_nvctl__subcmd__config__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__export_commands] )) ||
_nvctl__subcmd__config__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help_commands] )) ||
_nvctl__subcmd__config__subcmd__help_commands() {
    local commands; commands=(
'show:Show current configuration' \
'edit:Edit configuration file' \
'reset:Reset configuration to defaults' \
'backup:Backup configuration' \
'restore:Restore configuration from backup' \
'export:Export GPU profile to file' \
'import:Import GPU profile from file' \
'capture:Capture the current live state into a saved profile bundle' \
'preview:Preview a profile bundle from disk' \
'diff:Diff two profile bundles' \
'apply:Apply a saved bundle or live snapshot-compatible bundle' \
'profiles:List available profiles' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl config help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__apply_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__backup_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__backup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help backup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__capture_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__capture_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help capture commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__diff_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__diff_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help diff commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__edit_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__export_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__import_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__import_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help import commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__preview_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__profiles_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__reset_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__restore_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__help__subcmd__show_commands] )) ||
_nvctl__subcmd__config__subcmd__help__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config help show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__import_commands] )) ||
_nvctl__subcmd__config__subcmd__import_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config import commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__preview_commands] )) ||
_nvctl__subcmd__config__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__profiles_commands] )) ||
_nvctl__subcmd__config__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__reset_commands] )) ||
_nvctl__subcmd__config__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__restore_commands] )) ||
_nvctl__subcmd__config__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__config__subcmd__show_commands] )) ||
_nvctl__subcmd__config__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl config show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container_commands] )) ||
_nvctl__subcmd__container_commands() {
    local commands; commands=(
'list:List GPU-enabled containers' \
'status:Show container GPU status' \
'monitor:Monitor container GPU usage' \
'launch:Launch container with GPU support' \
'phantom-link:Launch PhantomLink audio container' \
'profiles:Container profile management' \
'runtime:Runtime information and setup' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl container commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help_commands] )) ||
_nvctl__subcmd__container__subcmd__help_commands() {
    local commands; commands=(
'list:List GPU-enabled containers' \
'status:Show container GPU status' \
'monitor:Monitor container GPU usage' \
'launch:Launch container with GPU support' \
'phantom-link:Launch PhantomLink audio container' \
'profiles:Container profile management' \
'runtime:Runtime information and setup' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl container help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__launch_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__monitor_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__phantom-link_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__phantom-link_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help phantom-link commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__profiles_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__profiles_commands() {
    local commands; commands=(
'list:List available profiles' \
'apply:Apply profile to container' \
'create:Create new profile' \
    )
    _describe -t commands 'nvctl container help profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__apply_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help profiles apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__create_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help profiles create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__list_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__profiles__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help profiles list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__runtime_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__runtime_commands() {
    local commands; commands=(
'info:Show runtime information' \
'doctor:Diagnose NVIDIA container runtime health' \
'setup:Setup container runtime' \
'test:Test GPU passthrough' \
'configure:Configure NVIDIA Container Runtime' \
    )
    _describe -t commands 'nvctl container help runtime commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__configure_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help runtime configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__doctor_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help runtime doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__info_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help runtime info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__setup_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help runtime setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__test_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__runtime__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help runtime test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__container__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__launch_commands] )) ||
_nvctl__subcmd__container__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__list_commands] )) ||
_nvctl__subcmd__container__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__monitor_commands] )) ||
_nvctl__subcmd__container__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__phantom-link_commands] )) ||
_nvctl__subcmd__container__subcmd__phantom-link_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container phantom-link commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles_commands() {
    local commands; commands=(
'list:List available profiles' \
'apply:Apply profile to container' \
'create:Create new profile' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl container profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__apply_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__create_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__help_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__help_commands() {
    local commands; commands=(
'list:List available profiles' \
'apply:Apply profile to container' \
'create:Create new profile' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl container profiles help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__apply_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles help apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__create_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles help create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__profiles__subcmd__list_commands] )) ||
_nvctl__subcmd__container__subcmd__profiles__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container profiles list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime_commands() {
    local commands; commands=(
'info:Show runtime information' \
'doctor:Diagnose NVIDIA container runtime health' \
'setup:Setup container runtime' \
'test:Test GPU passthrough' \
'configure:Configure NVIDIA Container Runtime' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl container runtime commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__configure_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__doctor_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help_commands() {
    local commands; commands=(
'info:Show runtime information' \
'doctor:Diagnose NVIDIA container runtime health' \
'setup:Setup container runtime' \
'test:Test GPU passthrough' \
'configure:Configure NVIDIA Container Runtime' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl container runtime help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__configure_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime help configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__doctor_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime help doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__setup_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime help setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__test_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__help__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime help test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__info_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__setup_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__runtime__subcmd__test_commands] )) ||
_nvctl__subcmd__container__subcmd__runtime__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container runtime test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__container__subcmd__status_commands] )) ||
_nvctl__subcmd__container__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl container status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display_commands] )) ||
_nvctl__subcmd__display_commands() {
    local commands; commands=(
'info:' \
'ls:' \
'vibrance:' \
'hdr:' \
'gamma:' \
'sharpening:' \
'color-range:Color range control (Full vs Limited RGB)' \
'color-space:Color space control (RGB, YCbCr422, YCbCr444)' \
'dithering:Dithering control for color banding reduction' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range_commands() {
    local commands; commands=(
'get:Get current color range setting' \
'set:Set color range (full or limited)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display color-range commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-range get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range__subcmd__help_commands() {
    local commands; commands=(
'get:Get current color range setting' \
'set:Set color range (full or limited)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display color-range help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-range help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-range help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-range help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-range__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__color-range__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-range set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space_commands() {
    local commands; commands=(
'get:Get current color space' \
'set:Set color space' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display color-space commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-space get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space__subcmd__help_commands() {
    local commands; commands=(
'get:Get current color space' \
'set:Set color space' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display color-space help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-space help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-space help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-space help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__color-space__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__color-space__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display color-space set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering_commands() {
    local commands; commands=(
'get:Get current dithering settings' \
'enable:Enable dithering with specified mode and depth' \
'disable:Disable dithering' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display dithering commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__disable_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__enable_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__help_commands() {
    local commands; commands=(
'get:Get current dithering settings' \
'enable:Enable dithering with specified mode and depth' \
'disable:Disable dithering' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display dithering help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__dithering__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display dithering help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma_commands() {
    local commands; commands=(
'get:Get current gamma' \
'set:Set gamma (0.5-3.0, default 1.0)' \
'reset:Reset gamma to default (1.0)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display gamma commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__help_commands() {
    local commands; commands=(
'get:Get current gamma' \
'set:Set gamma (0.5-3.0, default 1.0)' \
'reset:Reset gamma to default (1.0)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display gamma help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma help reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__gamma__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__gamma__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display gamma set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'toggle:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display hdr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__disable_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__enable_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__help_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'toggle:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display hdr help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__toggle_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__help__subcmd__toggle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr help toggle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__status_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__hdr__subcmd__toggle_commands] )) ||
_nvctl__subcmd__display__subcmd__hdr__subcmd__toggle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display hdr toggle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__help_commands() {
    local commands; commands=(
'info:' \
'ls:' \
'vibrance:' \
'hdr:' \
'gamma:' \
'sharpening:' \
'color-range:Color range control (Full vs Limited RGB)' \
'color-space:Color space control (RGB, YCbCr422, YCbCr444)' \
'dithering:Dithering control for color banding reduction' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__color-range_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__color-range_commands() {
    local commands; commands=(
'get:Get current color range setting' \
'set:Set color range (full or limited)' \
    )
    _describe -t commands 'nvctl display help color-range commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__color-range__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__color-range__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help color-range get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__color-range__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__color-range__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help color-range set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__color-space_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__color-space_commands() {
    local commands; commands=(
'get:Get current color space' \
'set:Set color space' \
    )
    _describe -t commands 'nvctl display help color-space commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__color-space__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__color-space__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help color-space get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__color-space__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__color-space__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help color-space set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__dithering_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__dithering_commands() {
    local commands; commands=(
'get:Get current dithering settings' \
'enable:Enable dithering with specified mode and depth' \
'disable:Disable dithering' \
    )
    _describe -t commands 'nvctl display help dithering commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__disable_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help dithering disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__enable_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help dithering enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__dithering__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help dithering get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__gamma_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__gamma_commands() {
    local commands; commands=(
'get:Get current gamma' \
'set:Set gamma (0.5-3.0, default 1.0)' \
'reset:Reset gamma to default (1.0)' \
    )
    _describe -t commands 'nvctl display help gamma commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help gamma get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help gamma reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__gamma__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help gamma set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__hdr_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__hdr_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'toggle:' \
    )
    _describe -t commands 'nvctl display help hdr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__disable_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help hdr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__enable_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help hdr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__status_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help hdr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__toggle_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__hdr__subcmd__toggle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help hdr toggle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__ls_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__ls_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help ls commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__sharpening_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__sharpening_commands() {
    local commands; commands=(
'get:Get current image sharpening for a display' \
'set:Set image sharpening (0-100, default varies by display)' \
'reset:Reset image sharpening to default' \
'info:Show image sharpening info for a display' \
    )
    _describe -t commands 'nvctl display help sharpening commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help sharpening get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help sharpening info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help sharpening reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__sharpening__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help sharpening set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance_commands() {
    local commands; commands=(
'get:Get current vibrance for all displays' \
'set:Set vibrance for all displays (0-200%, where 100% is default)' \
'set-display:Set vibrance for specific display' \
'set-raw:Set vibrance using raw nvibrant values for multiple displays' \
'list:List all displays and their current vibrance' \
'reset:Reset all displays to default vibrance (100%)' \
'info:Show driver compatibility info' \
    )
    _describe -t commands 'nvctl display help vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__list_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set-display_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set-display_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance set-display commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set-raw_commands] )) ||
_nvctl__subcmd__display__subcmd__help__subcmd__vibrance__subcmd__set-raw_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display help vibrance set-raw commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__ls_commands] )) ||
_nvctl__subcmd__display__subcmd__ls_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display ls commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening_commands() {
    local commands; commands=(
'get:Get current image sharpening for a display' \
'set:Set image sharpening (0-100, default varies by display)' \
'reset:Reset image sharpening to default' \
'info:Show image sharpening info for a display' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display sharpening commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__help_commands() {
    local commands; commands=(
'get:Get current image sharpening for a display' \
'set:Set image sharpening (0-100, default varies by display)' \
'reset:Reset image sharpening to default' \
'info:Show image sharpening info for a display' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display sharpening help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening help reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__sharpening__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__sharpening__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display sharpening set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance_commands() {
    local commands; commands=(
'get:Get current vibrance for all displays' \
'set:Set vibrance for all displays (0-200%, where 100% is default)' \
'set-display:Set vibrance for specific display' \
'set-raw:Set vibrance using raw nvibrant values for multiple displays' \
'list:List all displays and their current vibrance' \
'reset:Reset all displays to default vibrance (100%)' \
'info:Show driver compatibility info' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help_commands() {
    local commands; commands=(
'get:Get current vibrance for all displays' \
'set:Set vibrance for all displays (0-200%, where 100% is default)' \
'set-display:Set vibrance for specific display' \
'set-raw:Set vibrance using raw nvibrant values for multiple displays' \
'list:List all displays and their current vibrance' \
'reset:Reset all displays to default vibrance (100%)' \
'info:Show driver compatibility info' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl display vibrance help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__get_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set-display_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set-display_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help set-display commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set-raw_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__help__subcmd__set-raw_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance help set-raw commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__info_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__list_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__reset_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__set_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__set-display_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__set-display_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance set-display commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__display__subcmd__vibrance__subcmd__set-raw_commands] )) ||
_nvctl__subcmd__display__subcmd__vibrance__subcmd__set-raw_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl display vibrance set-raw commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss_commands] )) ||
_nvctl__subcmd__dlss_commands() {
    local commands; commands=(
'status:Show DLSS capabilities and status' \
'enable:Enable DLSS 3 with Frame Generation' \
'disable:Disable DLSS' \
'profiles:Show game profiles' \
'auto:Auto-detect and apply game settings' \
'metrics:Show performance metrics' \
'doctor:Run DLSS diagnostics (GPU, driver, Proton compatibility)' \
'games:Scan game libraries for DLSS-enabled games' \
'versions:Show available DLSS versions' \
'launch-opts:Generate Proton launch options for a game' \
'info:Show info about a specific game'\''s DLSS installation' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl dlss commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__auto_commands] )) ||
_nvctl__subcmd__dlss__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__disable_commands] )) ||
_nvctl__subcmd__dlss__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__doctor_commands] )) ||
_nvctl__subcmd__dlss__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__enable_commands] )) ||
_nvctl__subcmd__dlss__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__games_commands] )) ||
_nvctl__subcmd__dlss__subcmd__games_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss games commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help_commands() {
    local commands; commands=(
'status:Show DLSS capabilities and status' \
'enable:Enable DLSS 3 with Frame Generation' \
'disable:Disable DLSS' \
'profiles:Show game profiles' \
'auto:Auto-detect and apply game settings' \
'metrics:Show performance metrics' \
'doctor:Run DLSS diagnostics (GPU, driver, Proton compatibility)' \
'games:Scan game libraries for DLSS-enabled games' \
'versions:Show available DLSS versions' \
'launch-opts:Generate Proton launch options for a game' \
'info:Show info about a specific game'\''s DLSS installation' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl dlss help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__auto_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__doctor_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__games_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__games_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help games commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__launch-opts_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__launch-opts_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help launch-opts commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__metrics_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__metrics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help metrics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__profiles_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__help__subcmd__versions_commands] )) ||
_nvctl__subcmd__dlss__subcmd__help__subcmd__versions_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss help versions commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__info_commands] )) ||
_nvctl__subcmd__dlss__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__launch-opts_commands] )) ||
_nvctl__subcmd__dlss__subcmd__launch-opts_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss launch-opts commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__metrics_commands] )) ||
_nvctl__subcmd__dlss__subcmd__metrics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss metrics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__profiles_commands] )) ||
_nvctl__subcmd__dlss__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__status_commands] )) ||
_nvctl__subcmd__dlss__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__dlss__subcmd__versions_commands] )) ||
_nvctl__subcmd__dlss__subcmd__versions_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl dlss versions commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__doctor_commands] )) ||
_nvctl__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver_commands] )) ||
_nvctl__subcmd__driver_commands() {
    local commands; commands=(
'info:Show comprehensive driver status (GPU, version, kernel, GSP, DKMS)' \
'check:Run driver health checks with opinionated warnings' \
'capabilities:Show driver capabilities and feature requirements' \
'validate:Validate system readiness for a target driver version' \
'diagnose-release:Diagnose kernel/userspace/GSP release alignment and firmware layout' \
'support-bundle:Write a support bundle with driver, GSP, DKMS, and log diagnostics' \
'install:Install a driver (proprietary, open, open-beta)' \
'update:Update driver to latest version' \
'rollback:Rollback to previous driver version (Arch Linux only)' \
'dkms:DKMS kernel module management' \
'gsp:GSP firmware management (nvidia-open)' \
'logs:View NVIDIA driver kernel logs' \
'source:Build nvidia-open from source (git clone workflow)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__driver__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__check_commands] )) ||
_nvctl__subcmd__driver__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver check commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__diagnose-release_commands] )) ||
_nvctl__subcmd__driver__subcmd__diagnose-release_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver diagnose-release commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms_commands() {
    local commands; commands=(
'status:Show detailed DKMS module status for all kernels' \
'doctor:Diagnose DKMS/header/source issues before rebuilding' \
'setup:Set up DKMS for nvidia-open (register source, create config)' \
'build:Build nvidia modules for all kernels (or specific with --kernel)' \
'logs:Show DKMS build logs (errors, warnings)' \
'unregister:Unregister nvidia from DKMS' \
'hook:Install pacman hooks for auto-rebuild on kernel updates (Arch)' \
'fix:Attempt to fix common DKMS issues' \
'cleanup:Remove old kernel modules (keeps running kernel + N most recent)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver dkms commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__build_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__build_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms build commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__cleanup_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__cleanup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms cleanup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__doctor_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__fix_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__fix_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms fix commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help_commands() {
    local commands; commands=(
'status:Show detailed DKMS module status for all kernels' \
'doctor:Diagnose DKMS/header/source issues before rebuilding' \
'setup:Set up DKMS for nvidia-open (register source, create config)' \
'build:Build nvidia modules for all kernels (or specific with --kernel)' \
'logs:Show DKMS build logs (errors, warnings)' \
'unregister:Unregister nvidia from DKMS' \
'hook:Install pacman hooks for auto-rebuild on kernel updates (Arch)' \
'fix:Attempt to fix common DKMS issues' \
'cleanup:Remove old kernel modules (keeps running kernel + N most recent)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver dkms help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__build_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__build_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help build commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__cleanup_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__cleanup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help cleanup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__doctor_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__fix_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__fix_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help fix commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__hook_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__hook_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help hook commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__logs_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__setup_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__unregister_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__help__subcmd__unregister_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms help unregister commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__hook_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__hook_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms hook commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__logs_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__setup_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__dkms__subcmd__unregister_commands] )) ||
_nvctl__subcmd__driver__subcmd__dkms__subcmd__unregister_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver dkms unregister commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp_commands() {
    local commands; commands=(
'status:Show GSP firmware status' \
'enable:Enable GSP firmware' \
'disable:Disable GSP firmware (fallback mode)' \
'diagnostics:Run GSP diagnostics' \
'explain:Explain what GSP is and common issues' \
'check-update:Check for firmware updates' \
'update:Update GSP firmware' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver gsp commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__check-update_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__check-update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp check-update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__diagnostics_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__diagnostics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp diagnostics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__disable_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__enable_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__explain_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__explain_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp explain commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help_commands() {
    local commands; commands=(
'status:Show GSP firmware status' \
'enable:Enable GSP firmware' \
'disable:Disable GSP firmware (fallback mode)' \
'diagnostics:Run GSP diagnostics' \
'explain:Explain what GSP is and common issues' \
'check-update:Check for firmware updates' \
'update:Update GSP firmware' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver gsp help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__check-update_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__check-update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help check-update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__diagnostics_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__diagnostics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help diagnostics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__explain_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__explain_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help explain commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__help__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp help update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__gsp__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__gsp__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver gsp update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__help_commands() {
    local commands; commands=(
'info:Show comprehensive driver status (GPU, version, kernel, GSP, DKMS)' \
'check:Run driver health checks with opinionated warnings' \
'capabilities:Show driver capabilities and feature requirements' \
'validate:Validate system readiness for a target driver version' \
'diagnose-release:Diagnose kernel/userspace/GSP release alignment and firmware layout' \
'support-bundle:Write a support bundle with driver, GSP, DKMS, and log diagnostics' \
'install:Install a driver (proprietary, open, open-beta)' \
'update:Update driver to latest version' \
'rollback:Rollback to previous driver version (Arch Linux only)' \
'dkms:DKMS kernel module management' \
'gsp:GSP firmware management (nvidia-open)' \
'logs:View NVIDIA driver kernel logs' \
'source:Build nvidia-open from source (git clone workflow)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__check_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help check commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__diagnose-release_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__diagnose-release_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help diagnose-release commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms_commands() {
    local commands; commands=(
'status:Show detailed DKMS module status for all kernels' \
'doctor:Diagnose DKMS/header/source issues before rebuilding' \
'setup:Set up DKMS for nvidia-open (register source, create config)' \
'build:Build nvidia modules for all kernels (or specific with --kernel)' \
'logs:Show DKMS build logs (errors, warnings)' \
'unregister:Unregister nvidia from DKMS' \
'hook:Install pacman hooks for auto-rebuild on kernel updates (Arch)' \
'fix:Attempt to fix common DKMS issues' \
'cleanup:Remove old kernel modules (keeps running kernel + N most recent)' \
    )
    _describe -t commands 'nvctl driver help dkms commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__build_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__build_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms build commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__cleanup_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__cleanup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms cleanup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__doctor_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__fix_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__fix_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms fix commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__hook_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__hook_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms hook commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__logs_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__setup_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__unregister_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__dkms__subcmd__unregister_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help dkms unregister commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp_commands() {
    local commands; commands=(
'status:Show GSP firmware status' \
'enable:Enable GSP firmware' \
'disable:Disable GSP firmware (fallback mode)' \
'diagnostics:Run GSP diagnostics' \
'explain:Explain what GSP is and common issues' \
'check-update:Check for firmware updates' \
'update:Update GSP firmware' \
    )
    _describe -t commands 'nvctl driver help gsp commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__check-update_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__check-update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp check-update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__diagnostics_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__diagnostics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp diagnostics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__disable_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__enable_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__explain_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__explain_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp explain commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__gsp__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help gsp update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__install_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__install_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help install commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__logs_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__rollback_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__rollback_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help rollback commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__source_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__source_commands() {
    local commands; commands=(
'status:Show source build status and info' \
'doctor:Diagnose source tree state, git pinning, and reproducibility info' \
'init:Initialize from a git clone of open-gpu-kernel-modules' \
'update:Update source\: fetch latest tag, checkout, and rebuild' \
'sync:Sync\: rebuild modules from current source without updating' \
    )
    _describe -t commands 'nvctl driver help source commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__doctor_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help source doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__init_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__init_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help source init commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help source status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__sync_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__sync_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help source sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__source__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help source update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__support-bundle_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__support-bundle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help support-bundle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__help__subcmd__validate_commands] )) ||
_nvctl__subcmd__driver__subcmd__help__subcmd__validate_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver help validate commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__info_commands] )) ||
_nvctl__subcmd__driver__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__install_commands] )) ||
_nvctl__subcmd__driver__subcmd__install_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver install commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__logs_commands] )) ||
_nvctl__subcmd__driver__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__rollback_commands] )) ||
_nvctl__subcmd__driver__subcmd__rollback_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver rollback commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source_commands] )) ||
_nvctl__subcmd__driver__subcmd__source_commands() {
    local commands; commands=(
'status:Show source build status and info' \
'doctor:Diagnose source tree state, git pinning, and reproducibility info' \
'init:Initialize from a git clone of open-gpu-kernel-modules' \
'update:Update source\: fetch latest tag, checkout, and rebuild' \
'sync:Sync\: rebuild modules from current source without updating' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver source commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__doctor_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help_commands() {
    local commands; commands=(
'status:Show source build status and info' \
'doctor:Diagnose source tree state, git pinning, and reproducibility info' \
'init:Initialize from a git clone of open-gpu-kernel-modules' \
'update:Update source\: fetch latest tag, checkout, and rebuild' \
'sync:Sync\: rebuild modules from current source without updating' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl driver source help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__doctor_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source help doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__init_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__init_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source help init commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__sync_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__sync_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source help sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__help__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source help update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__init_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__init_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source init commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__status_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__sync_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__sync_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__source__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__source__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver source update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__support-bundle_commands] )) ||
_nvctl__subcmd__driver__subcmd__support-bundle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver support-bundle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__update_commands] )) ||
_nvctl__subcmd__driver__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__driver__subcmd__validate_commands] )) ||
_nvctl__subcmd__driver__subcmd__validate_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl driver validate commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan_commands] )) ||
_nvctl__subcmd__fan_commands() {
    local commands; commands=(
'info:' \
'set:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl fan commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan__subcmd__help_commands] )) ||
_nvctl__subcmd__fan__subcmd__help_commands() {
    local commands; commands=(
'info:' \
'set:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl fan help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__fan__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl fan help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__fan__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl fan help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__fan__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl fan help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan__subcmd__info_commands] )) ||
_nvctl__subcmd__fan__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl fan info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__fan__subcmd__set_commands] )) ||
_nvctl__subcmd__fan__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl fan set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming_commands] )) ||
_nvctl__subcmd__gaming_commands() {
    local commands; commands=(
'enable:Enable gaming optimizations' \
'disable:Disable gaming optimizations' \
'status:Show gaming optimization status' \
'latency:Latency optimization controls' \
'gamescope:Gamescope controls' \
'launch:Game launch profiles' \
'auto:Automatic game profile application' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto_commands() {
    local commands; commands=(
'start:Start automatic profile application service' \
'stop:Stop automatic profile application service' \
'status:Show service status' \
'install-service:Install a user systemd service for auto-profile startup' \
'uninstall-service:Uninstall the user systemd service' \
'enable-service:Enable and start the user systemd service' \
'disable-service:Disable and stop the user systemd service' \
'daemon:Internal foreground daemon mode' \
'enable:Enable automatic profile application on boot' \
'disable:Disable automatic profile application on boot' \
'config:Configure auto-application settings' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__config_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__daemon_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__daemon_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto daemon commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__disable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__disable-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__disable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto disable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__enable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__enable-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__enable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto enable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help_commands() {
    local commands; commands=(
'start:Start automatic profile application service' \
'stop:Stop automatic profile application service' \
'status:Show service status' \
'install-service:Install a user systemd service for auto-profile startup' \
'uninstall-service:Uninstall the user systemd service' \
'enable-service:Enable and start the user systemd service' \
'disable-service:Disable and stop the user systemd service' \
'daemon:Internal foreground daemon mode' \
'enable:Enable automatic profile application on boot' \
'disable:Disable automatic profile application on boot' \
'config:Configure auto-application settings' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming auto help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__config_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__daemon_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__daemon_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help daemon commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__disable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help disable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__enable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help enable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__install-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__install-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help install-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__start_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__stop_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__uninstall-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__help__subcmd__uninstall-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto help uninstall-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__install-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__install-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto install-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__start_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__stop_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__auto__subcmd__uninstall-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__auto__subcmd__uninstall-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming auto uninstall-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__disable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__enable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope_commands() {
    local commands; commands=(
'launch:Launch application with Gamescope' \
'presets:List available presets' \
'create-preset:Create new preset' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming gamescope commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__create-preset_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__create-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope create-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help_commands() {
    local commands; commands=(
'launch:Launch application with Gamescope' \
'presets:List available presets' \
'create-preset:Create new preset' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming gamescope help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__create-preset_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__create-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope help create-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__launch_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope help launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__presets_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__help__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope help presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__launch_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__presets_commands] )) ||
_nvctl__subcmd__gaming__subcmd__gamescope__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming gamescope presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help_commands() {
    local commands; commands=(
'enable:Enable gaming optimizations' \
'disable:Disable gaming optimizations' \
'status:Show gaming optimization status' \
'latency:Latency optimization controls' \
'gamescope:Gamescope controls' \
'launch:Game launch profiles' \
'auto:Automatic game profile application' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto_commands() {
    local commands; commands=(
'start:Start automatic profile application service' \
'stop:Stop automatic profile application service' \
'status:Show service status' \
'install-service:Install a user systemd service for auto-profile startup' \
'uninstall-service:Uninstall the user systemd service' \
'enable-service:Enable and start the user systemd service' \
'disable-service:Disable and stop the user systemd service' \
'daemon:Internal foreground daemon mode' \
'enable:Enable automatic profile application on boot' \
'disable:Disable automatic profile application on boot' \
'config:Configure auto-application settings' \
    )
    _describe -t commands 'nvctl gaming help auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__config_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__daemon_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__daemon_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto daemon commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__disable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto disable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__enable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto enable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__install-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__install-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto install-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__start_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__stop_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__uninstall-service_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__auto__subcmd__uninstall-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help auto uninstall-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope_commands() {
    local commands; commands=(
'launch:Launch application with Gamescope' \
'presets:List available presets' \
'create-preset:Create new preset' \
    )
    _describe -t commands 'nvctl gaming help gamescope commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__create-preset_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__create-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help gamescope create-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__launch_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help gamescope launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__presets_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__gamescope__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help gamescope presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__latency_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__latency_commands() {
    local commands; commands=(
'optimize:Optimize for low latency' \
'status:Show latency status' \
'test:Test latency' \
    )
    _describe -t commands 'nvctl gaming help latency commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__optimize_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help latency optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help latency status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__test_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__latency__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help latency test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch_commands() {
    local commands; commands=(
'run:Launch a game with a profile' \
'list:List all game profiles' \
'show:Show profile details' \
'create:Create a new game profile' \
'delete:Delete a game profile' \
'hook-add:Add a pre-launch or post-exit hook' \
'hook-list:List hooks for a game profile' \
'hook-remove:Remove a hook from a profile by phase and index' \
'set-gamescope-preset:Set a named gamescope preset on a profile' \
'examples:Create example game profiles' \
    )
    _describe -t commands 'nvctl gaming help launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__create_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__delete_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__delete_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch delete commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__examples_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook-add_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook-add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch hook-add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook-list_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook-list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch hook-list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook-remove_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__hook-remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch hook-remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__list_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__run_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__run_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch run commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__set-gamescope-preset_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__set-gamescope-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch set-gamescope-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__show_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__launch__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help launch show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency_commands() {
    local commands; commands=(
'optimize:Optimize for low latency' \
'status:Show latency status' \
'test:Test latency' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming latency commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__help_commands() {
    local commands; commands=(
'optimize:Optimize for low latency' \
'status:Show latency status' \
'test:Test latency' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming latency help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__optimize_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency help optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__test_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__help__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency help test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__optimize_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__latency__subcmd__test_commands] )) ||
_nvctl__subcmd__gaming__subcmd__latency__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming latency test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch_commands() {
    local commands; commands=(
'run:Launch a game with a profile' \
'list:List all game profiles' \
'show:Show profile details' \
'create:Create a new game profile' \
'delete:Delete a game profile' \
'hook-add:Add a pre-launch or post-exit hook' \
'hook-list:List hooks for a game profile' \
'hook-remove:Remove a hook from a profile by phase and index' \
'set-gamescope-preset:Set a named gamescope preset on a profile' \
'examples:Create example game profiles' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__create_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__delete_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__delete_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch delete commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__examples_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help_commands() {
    local commands; commands=(
'run:Launch a game with a profile' \
'list:List all game profiles' \
'show:Show profile details' \
'create:Create a new game profile' \
'delete:Delete a game profile' \
'hook-add:Add a pre-launch or post-exit hook' \
'hook-list:List hooks for a game profile' \
'hook-remove:Remove a hook from a profile by phase and index' \
'set-gamescope-preset:Set a named gamescope preset on a profile' \
'examples:Create example game profiles' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gaming launch help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__create_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__delete_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__delete_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help delete commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__examples_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook-add_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook-add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help hook-add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook-list_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook-list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help hook-list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook-remove_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__hook-remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help hook-remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__run_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__run_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help run commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__set-gamescope-preset_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__set-gamescope-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help set-gamescope-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__show_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__help__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch help show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__hook-add_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__hook-add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch hook-add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__hook-list_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__hook-list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch hook-list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__hook-remove_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__hook-remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch hook-remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__list_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__run_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__run_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch run commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__set-gamescope-preset_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__set-gamescope-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch set-gamescope-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__launch__subcmd__show_commands] )) ||
_nvctl__subcmd__gaming__subcmd__launch__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming launch show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gaming__subcmd__status_commands] )) ||
_nvctl__subcmd__gaming__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gaming status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu_commands] )) ||
_nvctl__subcmd__gpu_commands() {
    local commands; commands=(
'info:Show comprehensive GPU information' \
'stat:Launch live TUI dashboard for GPU monitoring' \
'capabilities:Show detailed GPU overclocking capabilities' \
'list:List all detected GPUs with details' \
'select:Select active GPU for commands' \
'benchmark:Benchmark GPU performance' \
'watch:Live GPU utilization monitoring (text output)' \
'export:Export GPU metrics to JSON/CSV' \
'stress:Stress test GPU with monitoring' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gpu commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__benchmark_commands] )) ||
_nvctl__subcmd__gpu__subcmd__benchmark_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu benchmark commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__gpu__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__export_commands] )) ||
_nvctl__subcmd__gpu__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help_commands() {
    local commands; commands=(
'info:Show comprehensive GPU information' \
'stat:Launch live TUI dashboard for GPU monitoring' \
'capabilities:Show detailed GPU overclocking capabilities' \
'list:List all detected GPUs with details' \
'select:Select active GPU for commands' \
'benchmark:Benchmark GPU performance' \
'watch:Live GPU utilization monitoring (text output)' \
'export:Export GPU metrics to JSON/CSV' \
'stress:Stress test GPU with monitoring' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl gpu help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__benchmark_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__benchmark_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help benchmark commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__export_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__select_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__select_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help select commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__stat_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__stat_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help stat commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__stress_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__stress_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help stress commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__help__subcmd__watch_commands] )) ||
_nvctl__subcmd__gpu__subcmd__help__subcmd__watch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu help watch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__info_commands] )) ||
_nvctl__subcmd__gpu__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__list_commands] )) ||
_nvctl__subcmd__gpu__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__select_commands] )) ||
_nvctl__subcmd__gpu__subcmd__select_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu select commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__stat_commands] )) ||
_nvctl__subcmd__gpu__subcmd__stat_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu stat commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__stress_commands] )) ||
_nvctl__subcmd__gpu__subcmd__stress_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu stress commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__gpu__subcmd__watch_commands] )) ||
_nvctl__subcmd__gpu__subcmd__watch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl gpu watch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr_commands] )) ||
_nvctl__subcmd__hdr_commands() {
    local commands; commands=(
'status:Show HDR status across all displays' \
'enable:Enable HDR on all capable displays (use '\''nvctl display hdr enable <id>'\'' for specific display)' \
'disable:Disable HDR on all displays (use '\''nvctl display hdr disable <id>'\'' for specific display)' \
'config:Show HDR configuration' \
'set-brightness:Set peak brightness (nits)' \
'tools:Show HDR tools and recommendations for games' \
'capabilities:Show display HDR capabilities (EDID info)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl hdr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__hdr__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__config_commands] )) ||
_nvctl__subcmd__hdr__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__disable_commands] )) ||
_nvctl__subcmd__hdr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__enable_commands] )) ||
_nvctl__subcmd__hdr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help_commands() {
    local commands; commands=(
'status:Show HDR status across all displays' \
'enable:Enable HDR on all capable displays (use '\''nvctl display hdr enable <id>'\'' for specific display)' \
'disable:Disable HDR on all displays (use '\''nvctl display hdr disable <id>'\'' for specific display)' \
'config:Show HDR configuration' \
'set-brightness:Set peak brightness (nits)' \
'tools:Show HDR tools and recommendations for games' \
'capabilities:Show display HDR capabilities (EDID info)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl hdr help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__config_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__set-brightness_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__set-brightness_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help set-brightness commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__help__subcmd__tools_commands] )) ||
_nvctl__subcmd__hdr__subcmd__help__subcmd__tools_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr help tools commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__set-brightness_commands] )) ||
_nvctl__subcmd__hdr__subcmd__set-brightness_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr set-brightness commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__status_commands] )) ||
_nvctl__subcmd__hdr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__hdr__subcmd__tools_commands] )) ||
_nvctl__subcmd__hdr__subcmd__tools_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl hdr tools commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help_commands] )) ||
_nvctl__subcmd__help_commands() {
    local commands; commands=(
'gpu:🎮 GPU information and control' \
'display:🖥️ Display and monitor management' \
'vibrance:🌈 Digital vibrance control (0-200%)' \
'fan:🌀 Fan control and curves' \
'overclock:⚡ Overclocking and performance' \
'vrr:🔄 Variable Refresh Rate (VRR/G-Sync)' \
'monitor:📊 Real-time monitoring' \
'tui:📺 Terminal user interface' \
'nvtop:🖥️ GPU monitor (htop-style)' \
'gaming:🎯 Gaming optimization and latency' \
'recording:📹 Recording and streaming' \
'container:🐳 Container and virtualization workflows' \
'driver:🧠 Driver management, status, and kernel modules' \
'power:⚡ Power management' \
'color:🎨 Color and vibrance control' \
'config:⚙️ Configuration and profiles' \
'upscaling:📈 AI upscaling and enhancement' \
'dlss:🚀 DLSS and related features' \
'shaders:🎨 Shader cache management' \
'passthrough:🔌 GPU passthrough (VFIO/containers/VMs)' \
'wayland:🌊 Wayland NVIDIA optimization' \
'kde:🎨 KDE Plasma compositor optimization' \
'hdr:🌈 HDR control and configuration' \
'power-profile:⚡ Power profile management (AC/battery, activities)' \
'arch:🐧 Arch Linux integration (pacman hooks, DKMS)' \
'monitors:🖥️ Multi-monitor management' \
'osd:📊 On-screen display for gaming' \
'interactive:🎛️ Interactive menu mode' \
'system:💻 System information and platform detection' \
'doctor:🔍 Run system diagnostics' \
'completion:🧩 Generate shell completions' \
'version:📋 Show detailed version information' \
'asus:🎯 ASUS ROG GPU features (Power Detector+, Aura, etc.)' \
'companion:🔔 Lightweight desktop companion actions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch_commands] )) ||
_nvctl__subcmd__help__subcmd__arch_commands() {
    local commands; commands=(
'status:Show Arch Linux NVIDIA integration status' \
'install-hooks:Install all pacman hooks' \
'remove-hooks:Remove pacman hooks' \
'rebuild-dkms:Rebuild DKMS modules' \
'mkinitcpio:Regenerate initramfs' \
'check-updates:Check for pending NVIDIA/kernel updates' \
'aur-suggestions:List AUR optimization suggestions' \
    )
    _describe -t commands 'nvctl help arch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__aur-suggestions_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__aur-suggestions_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch aur-suggestions commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__check-updates_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__check-updates_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch check-updates commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__install-hooks_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__install-hooks_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch install-hooks commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__mkinitcpio_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__mkinitcpio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch mkinitcpio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__rebuild-dkms_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__rebuild-dkms_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch rebuild-dkms commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__remove-hooks_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__remove-hooks_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch remove-hooks commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__arch__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__arch__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help arch status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus_commands] )) ||
_nvctl__subcmd__help__subcmd__asus_commands() {
    local commands; commands=(
'detect:Detect ASUS ROG GPUs in system' \
'power:Show Power Detector+ status (12V-2x6 connector monitoring)' \
'status:Show ASUS GPU Tweak-style status' \
'aura:ASUS Aura RGB control' \
    )
    _describe -t commands 'nvctl help asus commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__aura_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__aura_commands() {
    local commands; commands=(
'status:Show Aura status' \
'mode:Set Aura mode (off, static, breathing, rainbow, cycle, cyberpunk, purple, performance, silent)' \
'color:Set Aura color (static mode)' \
'temp-reactive:Enable/disable temperature-reactive RGB (color changes with GPU temp)' \
'restore:Restore saved Aura configuration from config file' \
    )
    _describe -t commands 'nvctl help asus aura commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__color_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__color_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus aura color commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__mode_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__mode_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus aura mode commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__restore_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus aura restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus aura status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__temp-reactive_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__aura__subcmd__temp-reactive_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus aura temp-reactive commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__detect_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__detect_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus detect commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__power_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__power_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__asus__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__asus__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help asus status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color_commands] )) ||
_nvctl__subcmd__help__subcmd__color_commands() {
    local commands; commands=(
'vibrance:Vibrance control' \
'profiles:Color profile management' \
    )
    _describe -t commands 'nvctl help color commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__profiles_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__profiles_commands() {
    local commands; commands=(
'list:List available color profiles' \
'create:Create new color profile' \
'apply:Apply color profile' \
'schedule:Schedule color profile' \
    )
    _describe -t commands 'nvctl help color profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__apply_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color profiles apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__create_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color profiles create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color profiles list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__schedule_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__profiles__subcmd__schedule_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color profiles schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__vibrance_commands() {
    local commands; commands=(
'get:Get current vibrance for a display' \
'set:Set vibrance for a display' \
'apply:Apply vibrance profile' \
'preview:Preview vibrance changes' \
    )
    _describe -t commands 'nvctl help color vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__apply_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color vibrance apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color vibrance get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__preview_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color vibrance preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__color__subcmd__vibrance__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help color vibrance set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__companion_commands] )) ||
_nvctl__subcmd__help__subcmd__companion_commands() {
    local commands; commands=(
'notify-test:Send a desktop notification test' \
'open-docs:Open the project documentation in the default desktop handler' \
    )
    _describe -t commands 'nvctl help companion commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__companion__subcmd__notify-test_commands] )) ||
_nvctl__subcmd__help__subcmd__companion__subcmd__notify-test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help companion notify-test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__companion__subcmd__open-docs_commands] )) ||
_nvctl__subcmd__help__subcmd__companion__subcmd__open-docs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help companion open-docs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__completion_commands] )) ||
_nvctl__subcmd__help__subcmd__completion_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help completion commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config_commands] )) ||
_nvctl__subcmd__help__subcmd__config_commands() {
    local commands; commands=(
'show:Show current configuration' \
'edit:Edit configuration file' \
'reset:Reset configuration to defaults' \
'backup:Backup configuration' \
'restore:Restore configuration from backup' \
'export:Export GPU profile to file' \
'import:Import GPU profile from file' \
'capture:Capture the current live state into a saved profile bundle' \
'preview:Preview a profile bundle from disk' \
'diff:Diff two profile bundles' \
'apply:Apply a saved bundle or live snapshot-compatible bundle' \
'profiles:List available profiles' \
    )
    _describe -t commands 'nvctl help config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__apply_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__backup_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__backup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config backup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__capture_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__capture_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config capture commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__diff_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__diff_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config diff commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__edit_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__export_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__import_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__import_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config import commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__preview_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__profiles_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__reset_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__restore_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__restore_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config restore commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__config__subcmd__show_commands] )) ||
_nvctl__subcmd__help__subcmd__config__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help config show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container_commands] )) ||
_nvctl__subcmd__help__subcmd__container_commands() {
    local commands; commands=(
'list:List GPU-enabled containers' \
'status:Show container GPU status' \
'monitor:Monitor container GPU usage' \
'launch:Launch container with GPU support' \
'phantom-link:Launch PhantomLink audio container' \
'profiles:Container profile management' \
'runtime:Runtime information and setup' \
    )
    _describe -t commands 'nvctl help container commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__launch_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__monitor_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__phantom-link_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__phantom-link_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container phantom-link commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__profiles_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__profiles_commands() {
    local commands; commands=(
'list:List available profiles' \
'apply:Apply profile to container' \
'create:Create new profile' \
    )
    _describe -t commands 'nvctl help container profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__apply_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container profiles apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__create_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container profiles create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__profiles__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container profiles list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__runtime_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__runtime_commands() {
    local commands; commands=(
'info:Show runtime information' \
'doctor:Diagnose NVIDIA container runtime health' \
'setup:Setup container runtime' \
'test:Test GPU passthrough' \
'configure:Configure NVIDIA Container Runtime' \
    )
    _describe -t commands 'nvctl help container runtime commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__configure_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container runtime configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__doctor_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container runtime doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container runtime info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__setup_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container runtime setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__test_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__runtime__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container runtime test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__container__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__container__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help container status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display_commands] )) ||
_nvctl__subcmd__help__subcmd__display_commands() {
    local commands; commands=(
'info:' \
'ls:' \
'vibrance:' \
'hdr:' \
'gamma:' \
'sharpening:' \
'color-range:Color range control (Full vs Limited RGB)' \
'color-space:Color space control (RGB, YCbCr422, YCbCr444)' \
'dithering:Dithering control for color banding reduction' \
    )
    _describe -t commands 'nvctl help display commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__color-range_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__color-range_commands() {
    local commands; commands=(
'get:Get current color range setting' \
'set:Set color range (full or limited)' \
    )
    _describe -t commands 'nvctl help display color-range commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__color-range__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__color-range__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display color-range get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__color-range__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__color-range__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display color-range set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__color-space_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__color-space_commands() {
    local commands; commands=(
'get:Get current color space' \
'set:Set color space' \
    )
    _describe -t commands 'nvctl help display color-space commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__color-space__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__color-space__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display color-space get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__color-space__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__color-space__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display color-space set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__dithering_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__dithering_commands() {
    local commands; commands=(
'get:Get current dithering settings' \
'enable:Enable dithering with specified mode and depth' \
'disable:Disable dithering' \
    )
    _describe -t commands 'nvctl help display dithering commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display dithering disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display dithering enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__dithering__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display dithering get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__gamma_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__gamma_commands() {
    local commands; commands=(
'get:Get current gamma' \
'set:Set gamma (0.5-3.0, default 1.0)' \
'reset:Reset gamma to default (1.0)' \
    )
    _describe -t commands 'nvctl help display gamma commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display gamma get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__reset_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display gamma reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__gamma__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display gamma set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__hdr_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__hdr_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'toggle:' \
    )
    _describe -t commands 'nvctl help display hdr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display hdr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display hdr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display hdr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__toggle_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__hdr__subcmd__toggle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display hdr toggle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__ls_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__ls_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display ls commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__sharpening_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__sharpening_commands() {
    local commands; commands=(
'get:Get current image sharpening for a display' \
'set:Set image sharpening (0-100, default varies by display)' \
'reset:Reset image sharpening to default' \
'info:Show image sharpening info for a display' \
    )
    _describe -t commands 'nvctl help display sharpening commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display sharpening get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display sharpening info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__reset_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display sharpening reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__sharpening__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display sharpening set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance_commands() {
    local commands; commands=(
'get:Get current vibrance for all displays' \
'set:Set vibrance for all displays (0-200%, where 100% is default)' \
'set-display:Set vibrance for specific display' \
'set-raw:Set vibrance using raw nvibrant values for multiple displays' \
'list:List all displays and their current vibrance' \
'reset:Reset all displays to default vibrance (100%)' \
'info:Show driver compatibility info' \
    )
    _describe -t commands 'nvctl help display vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__get_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__get_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance get commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__reset_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set-display_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set-display_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance set-display commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set-raw_commands] )) ||
_nvctl__subcmd__help__subcmd__display__subcmd__vibrance__subcmd__set-raw_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help display vibrance set-raw commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss_commands() {
    local commands; commands=(
'status:Show DLSS capabilities and status' \
'enable:Enable DLSS 3 with Frame Generation' \
'disable:Disable DLSS' \
'profiles:Show game profiles' \
'auto:Auto-detect and apply game settings' \
'metrics:Show performance metrics' \
'doctor:Run DLSS diagnostics (GPU, driver, Proton compatibility)' \
'games:Scan game libraries for DLSS-enabled games' \
'versions:Show available DLSS versions' \
'launch-opts:Generate Proton launch options for a game' \
'info:Show info about a specific game'\''s DLSS installation' \
    )
    _describe -t commands 'nvctl help dlss commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__auto_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__doctor_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__games_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__games_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss games commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__launch-opts_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__launch-opts_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss launch-opts commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__metrics_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__metrics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss metrics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__profiles_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__dlss__subcmd__versions_commands] )) ||
_nvctl__subcmd__help__subcmd__dlss__subcmd__versions_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help dlss versions commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__doctor_commands] )) ||
_nvctl__subcmd__help__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver_commands] )) ||
_nvctl__subcmd__help__subcmd__driver_commands() {
    local commands; commands=(
'info:Show comprehensive driver status (GPU, version, kernel, GSP, DKMS)' \
'check:Run driver health checks with opinionated warnings' \
'capabilities:Show driver capabilities and feature requirements' \
'validate:Validate system readiness for a target driver version' \
'diagnose-release:Diagnose kernel/userspace/GSP release alignment and firmware layout' \
'support-bundle:Write a support bundle with driver, GSP, DKMS, and log diagnostics' \
'install:Install a driver (proprietary, open, open-beta)' \
'update:Update driver to latest version' \
'rollback:Rollback to previous driver version (Arch Linux only)' \
'dkms:DKMS kernel module management' \
'gsp:GSP firmware management (nvidia-open)' \
'logs:View NVIDIA driver kernel logs' \
'source:Build nvidia-open from source (git clone workflow)' \
    )
    _describe -t commands 'nvctl help driver commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__check_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver check commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__diagnose-release_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__diagnose-release_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver diagnose-release commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms_commands() {
    local commands; commands=(
'status:Show detailed DKMS module status for all kernels' \
'doctor:Diagnose DKMS/header/source issues before rebuilding' \
'setup:Set up DKMS for nvidia-open (register source, create config)' \
'build:Build nvidia modules for all kernels (or specific with --kernel)' \
'logs:Show DKMS build logs (errors, warnings)' \
'unregister:Unregister nvidia from DKMS' \
'hook:Install pacman hooks for auto-rebuild on kernel updates (Arch)' \
'fix:Attempt to fix common DKMS issues' \
'cleanup:Remove old kernel modules (keeps running kernel + N most recent)' \
    )
    _describe -t commands 'nvctl help driver dkms commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__build_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__build_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms build commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__cleanup_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__cleanup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms cleanup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__doctor_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__fix_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__fix_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms fix commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__hook_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__hook_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms hook commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__logs_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__setup_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__setup_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms setup commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__unregister_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__dkms__subcmd__unregister_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver dkms unregister commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp_commands() {
    local commands; commands=(
'status:Show GSP firmware status' \
'enable:Enable GSP firmware' \
'disable:Disable GSP firmware (fallback mode)' \
'diagnostics:Run GSP diagnostics' \
'explain:Explain what GSP is and common issues' \
'check-update:Check for firmware updates' \
'update:Update GSP firmware' \
    )
    _describe -t commands 'nvctl help driver gsp commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__check-update_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__check-update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp check-update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__diagnostics_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__diagnostics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp diagnostics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__explain_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__explain_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp explain commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__update_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__gsp__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver gsp update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__install_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__install_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver install commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__logs_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__logs_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver logs commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__rollback_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__rollback_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver rollback commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__source_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__source_commands() {
    local commands; commands=(
'status:Show source build status and info' \
'doctor:Diagnose source tree state, git pinning, and reproducibility info' \
'init:Initialize from a git clone of open-gpu-kernel-modules' \
'update:Update source\: fetch latest tag, checkout, and rebuild' \
'sync:Sync\: rebuild modules from current source without updating' \
    )
    _describe -t commands 'nvctl help driver source commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__doctor_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__doctor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver source doctor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__init_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__init_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver source init commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver source status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__sync_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__sync_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver source sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__update_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__source__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver source update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__support-bundle_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__support-bundle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver support-bundle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__update_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver update commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__driver__subcmd__validate_commands] )) ||
_nvctl__subcmd__help__subcmd__driver__subcmd__validate_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help driver validate commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__fan_commands] )) ||
_nvctl__subcmd__help__subcmd__fan_commands() {
    local commands; commands=(
'info:' \
'set:' \
    )
    _describe -t commands 'nvctl help fan commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__fan__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__fan__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help fan info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__fan__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__fan__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help fan set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming_commands() {
    local commands; commands=(
'enable:Enable gaming optimizations' \
'disable:Disable gaming optimizations' \
'status:Show gaming optimization status' \
'latency:Latency optimization controls' \
'gamescope:Gamescope controls' \
'launch:Game launch profiles' \
'auto:Automatic game profile application' \
    )
    _describe -t commands 'nvctl help gaming commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto_commands() {
    local commands; commands=(
'start:Start automatic profile application service' \
'stop:Stop automatic profile application service' \
'status:Show service status' \
'install-service:Install a user systemd service for auto-profile startup' \
'uninstall-service:Uninstall the user systemd service' \
'enable-service:Enable and start the user systemd service' \
'disable-service:Disable and stop the user systemd service' \
'daemon:Internal foreground daemon mode' \
'enable:Enable automatic profile application on boot' \
'disable:Disable automatic profile application on boot' \
'config:Configure auto-application settings' \
    )
    _describe -t commands 'nvctl help gaming auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__config_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__daemon_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__daemon_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto daemon commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable-service_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__disable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto disable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable-service_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__enable-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto enable-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__install-service_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__install-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto install-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__start_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__stop_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__uninstall-service_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__auto__subcmd__uninstall-service_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming auto uninstall-service commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope_commands() {
    local commands; commands=(
'launch:Launch application with Gamescope' \
'presets:List available presets' \
'create-preset:Create new preset' \
    )
    _describe -t commands 'nvctl help gaming gamescope commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__create-preset_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__create-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming gamescope create-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__launch_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__launch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming gamescope launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__presets_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__gamescope__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming gamescope presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__latency_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__latency_commands() {
    local commands; commands=(
'optimize:Optimize for low latency' \
'status:Show latency status' \
'test:Test latency' \
    )
    _describe -t commands 'nvctl help gaming latency commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__optimize_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming latency optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming latency status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__test_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__latency__subcmd__test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming latency test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch_commands() {
    local commands; commands=(
'run:Launch a game with a profile' \
'list:List all game profiles' \
'show:Show profile details' \
'create:Create a new game profile' \
'delete:Delete a game profile' \
'hook-add:Add a pre-launch or post-exit hook' \
'hook-list:List hooks for a game profile' \
'hook-remove:Remove a hook from a profile by phase and index' \
'set-gamescope-preset:Set a named gamescope preset on a profile' \
'examples:Create example game profiles' \
    )
    _describe -t commands 'nvctl help gaming launch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__create_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__create_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch create commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__delete_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__delete_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch delete commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__examples_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook-add_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook-add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch hook-add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook-list_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook-list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch hook-list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook-remove_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__hook-remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch hook-remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__run_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__run_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch run commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__set-gamescope-preset_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__set-gamescope-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch set-gamescope-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__show_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__launch__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming launch show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gaming__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__gaming__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gaming status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu_commands() {
    local commands; commands=(
'info:Show comprehensive GPU information' \
'stat:Launch live TUI dashboard for GPU monitoring' \
'capabilities:Show detailed GPU overclocking capabilities' \
'list:List all detected GPUs with details' \
'select:Select active GPU for commands' \
'benchmark:Benchmark GPU performance' \
'watch:Live GPU utilization monitoring (text output)' \
'export:Export GPU metrics to JSON/CSV' \
'stress:Stress test GPU with monitoring' \
    )
    _describe -t commands 'nvctl help gpu commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__benchmark_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__benchmark_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu benchmark commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__export_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__select_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__select_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu select commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__stat_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__stat_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu stat commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__stress_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__stress_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu stress commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__gpu__subcmd__watch_commands] )) ||
_nvctl__subcmd__help__subcmd__gpu__subcmd__watch_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help gpu watch commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr_commands() {
    local commands; commands=(
'status:Show HDR status across all displays' \
'enable:Enable HDR on all capable displays (use '\''nvctl display hdr enable <id>'\'' for specific display)' \
'disable:Disable HDR on all displays (use '\''nvctl display hdr disable <id>'\'' for specific display)' \
'config:Show HDR configuration' \
'set-brightness:Set peak brightness (nits)' \
'tools:Show HDR tools and recommendations for games' \
'capabilities:Show display HDR capabilities (EDID info)' \
    )
    _describe -t commands 'nvctl help hdr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__capabilities_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__capabilities_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr capabilities commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__config_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__set-brightness_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__set-brightness_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr set-brightness commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__hdr__subcmd__tools_commands] )) ||
_nvctl__subcmd__help__subcmd__hdr__subcmd__tools_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help hdr tools commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__interactive_commands] )) ||
_nvctl__subcmd__help__subcmd__interactive_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help interactive commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde_commands] )) ||
_nvctl__subcmd__help__subcmd__kde_commands() {
    local commands; commands=(
'status:Show KDE compositor status' \
'gaming:Apply gaming preset (low latency, VRR, minimal effects)' \
'productivity:Apply productivity preset (balanced, full effects)' \
'power-save:Apply power saving preset' \
'setup-env:Setup NVIDIA environment variables for KDE' \
'set-vrr:Set VRR per display' \
'restart:Restart KWin compositor' \
    )
    _describe -t commands 'nvctl help kde commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__gaming_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__gaming_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde gaming commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__power-save_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__power-save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde power-save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__productivity_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__productivity_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde productivity commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__restart_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__restart_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde restart commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__set-vrr_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__set-vrr_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde set-vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__setup-env_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__setup-env_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde setup-env commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__kde__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__kde__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help kde status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitor_commands] )) ||
_nvctl__subcmd__help__subcmd__monitor_commands() {
    local commands; commands=(
'start:Start monitoring' \
'stop:Stop monitoring' \
'status:Show monitoring status' \
'tui:Launch TUI monitoring interface' \
'export:Export monitoring data' \
    )
    _describe -t commands 'nvctl help monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitor__subcmd__export_commands] )) ||
_nvctl__subcmd__help__subcmd__monitor__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitor export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitor__subcmd__start_commands] )) ||
_nvctl__subcmd__help__subcmd__monitor__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitor start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitor__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__monitor__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitor status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitor__subcmd__stop_commands] )) ||
_nvctl__subcmd__help__subcmd__monitor__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitor stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitor__subcmd__tui_commands] )) ||
_nvctl__subcmd__help__subcmd__monitor__subcmd__tui_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitor tui commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors_commands() {
    local commands; commands=(
'status:Show current display configuration' \
'presets:List built-in monitor presets' \
'suggest:Suggest built-in monitor presets for the current setup' \
'preview:Preview a built-in monitor preset without applying it' \
'apply-preset:Apply a built-in monitor preset immediately' \
'save:Save current layout with a name' \
'load:Load and apply a saved layout' \
'list:List all saved layouts' \
'set-vrr:Set VRR for a specific display' \
'gamescope:Launch Gamescope on specific display' \
'auto:Auto-apply layout based on connected displays' \
'create-examples:Create example layouts' \
    )
    _describe -t commands 'nvctl help monitors commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__apply-preset_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__apply-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors apply-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__auto_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__create-examples_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__create-examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors create-examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__gamescope_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__gamescope_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors gamescope commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__load_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__load_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors load commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__presets_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__preview_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__save_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__set-vrr_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__set-vrr_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors set-vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__monitors__subcmd__suggest_commands] )) ||
_nvctl__subcmd__help__subcmd__monitors__subcmd__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help monitors suggest commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__nvtop_commands] )) ||
_nvctl__subcmd__help__subcmd__nvtop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help nvtop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd_commands] )) ||
_nvctl__subcmd__help__subcmd__osd_commands() {
    local commands; commands=(
'enable:Enable OSD overlay' \
'disable:Disable OSD overlay' \
'status:Show OSD status and configuration' \
'config:Configure OSD settings' \
'add:Add metric to OSD' \
'remove:Remove metric from OSD' \
'metrics:List available metrics' \
'check:Check MangoHud installation status' \
    )
    _describe -t commands 'nvctl help osd commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__add_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__check_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd check commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__config_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__metrics_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__metrics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd metrics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__remove_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__osd__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__osd__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help osd status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock_commands() {
    local commands; commands=(
'info:' \
'apply:' \
'profile:' \
'stress-test:' \
'auto:Automated overclocking wizard with safety features' \
'reset:' \
    )
    _describe -t commands 'nvctl help overclock commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock__subcmd__apply_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help overclock apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock__subcmd__auto_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help overclock auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help overclock info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock__subcmd__profile_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help overclock profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock__subcmd__reset_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help overclock reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__overclock__subcmd__stress-test_commands] )) ||
_nvctl__subcmd__help__subcmd__overclock__subcmd__stress-test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help overclock stress-test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough_commands() {
    local commands; commands=(
'status:Show GPU passthrough status' \
'list:List all NVIDIA GPUs and their PCI addresses' \
'iommu:Show IOMMU groups' \
'bind-vfio:Bind GPU to VFIO driver' \
'unbind-vfio:Unbind GPU from VFIO' \
'persistent:Setup persistent VFIO binding' \
'test-container:Test GPU passthrough to Docker container' \
'qemu-command:Generate QEMU command for GPU passthrough' \
'hugepages:Setup hugepages for VM performance' \
    )
    _describe -t commands 'nvctl help passthrough commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__bind-vfio_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__bind-vfio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough bind-vfio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__hugepages_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__hugepages_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough hugepages commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__iommu_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__iommu_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough iommu commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__persistent_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__persistent_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough persistent commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__qemu-command_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__qemu-command_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough qemu-command commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__test-container_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__test-container_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough test-container commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__passthrough__subcmd__unbind-vfio_commands] )) ||
_nvctl__subcmd__help__subcmd__passthrough__subcmd__unbind-vfio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help passthrough unbind-vfio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power_commands] )) ||
_nvctl__subcmd__help__subcmd__power_commands() {
    local commands; commands=(
'status:Show current power settings' \
'limit:Set GPU power limit (percentage)' \
'profile:Configure power profile' \
'persistence:Power persistence settings' \
'monitor:Monitor power usage' \
'automate:Automate power management' \
'curve:Manage power limit curves (temperature-based dynamic power)' \
'schedule:Schedule power profiles by time' \
    )
    _describe -t commands 'nvctl help power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__automate_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__automate_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power automate commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve_commands() {
    local commands; commands=(
'show:Show current power curve' \
'edit:Edit power curve interactively' \
'add:Add a curve point (temperature, power_limit)' \
'remove:Remove a curve point' \
'enable:Enable curve-based power management' \
'disable:Disable curve-based power management' \
'reset:Reset to default curve' \
    )
    _describe -t commands 'nvctl help power curve commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__add_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__edit_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__remove_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__reset_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__show_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__curve__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power curve show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__limit_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__limit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power limit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__monitor_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__persistence_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__persistence_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power persistence commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__profile_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__schedule_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__schedule_commands() {
    local commands; commands=(
'list:List all scheduled power profiles' \
'add:Add a scheduled power profile' \
'remove:Remove a schedule' \
'enable:Enable scheduled power management' \
'disable:Disable scheduled power management' \
    )
    _describe -t commands 'nvctl help power schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__add_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power schedule add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power schedule disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power schedule enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__list_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power schedule list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__remove_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__schedule__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power schedule remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__power__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile_commands() {
    local commands; commands=(
'status:Show current power profile status' \
'set:Set system power profile' \
'create-activity:Create activity-based profile' \
'apply:Apply profile for current activity' \
'monitor:Monitor and auto-switch on activity changes' \
'auto-power:Monitor and auto-switch on AC/Battery changes' \
'idle:Enable idle detection and power reduction' \
'create-defaults:Create default activity profiles' \
    )
    _describe -t commands 'nvctl help power-profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__apply_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__auto-power_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__auto-power_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile auto-power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__create-activity_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__create-activity_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile create-activity commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__create-defaults_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__create-defaults_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile create-defaults commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__idle_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__idle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile idle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__monitor_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__set_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__power-profile__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__power-profile__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help power-profile status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording_commands] )) ||
_nvctl__subcmd__help__subcmd__recording_commands() {
    local commands; commands=(
'start:Start recording' \
'stop:Stop recording' \
'status:Show recording status' \
'instant-replay:Start instant replay' \
'save:Save instant replay clip' \
'presets:List available presets' \
    )
    _describe -t commands 'nvctl help recording commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording__subcmd__instant-replay_commands] )) ||
_nvctl__subcmd__help__subcmd__recording__subcmd__instant-replay_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help recording instant-replay commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording__subcmd__presets_commands] )) ||
_nvctl__subcmd__help__subcmd__recording__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help recording presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording__subcmd__save_commands] )) ||
_nvctl__subcmd__help__subcmd__recording__subcmd__save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help recording save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording__subcmd__start_commands] )) ||
_nvctl__subcmd__help__subcmd__recording__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help recording start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__recording__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help recording status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__recording__subcmd__stop_commands] )) ||
_nvctl__subcmd__help__subcmd__recording__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help recording stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__shaders_commands] )) ||
_nvctl__subcmd__help__subcmd__shaders_commands() {
    local commands; commands=(
'stats:Show shader cache statistics' \
'clear:Clear all shader caches' \
'optimize:Optimize shader compilation settings' \
'precompile:Precompile shaders for a game' \
'open:Open shader cache folder' \
    )
    _describe -t commands 'nvctl help shaders commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__shaders__subcmd__clear_commands] )) ||
_nvctl__subcmd__help__subcmd__shaders__subcmd__clear_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help shaders clear commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__shaders__subcmd__open_commands] )) ||
_nvctl__subcmd__help__subcmd__shaders__subcmd__open_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help shaders open commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__shaders__subcmd__optimize_commands] )) ||
_nvctl__subcmd__help__subcmd__shaders__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help shaders optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__shaders__subcmd__precompile_commands] )) ||
_nvctl__subcmd__help__subcmd__shaders__subcmd__precompile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help shaders precompile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__shaders__subcmd__stats_commands] )) ||
_nvctl__subcmd__help__subcmd__shaders__subcmd__stats_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help shaders stats commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__system_commands] )) ||
_nvctl__subcmd__help__subcmd__system_commands() {
    local commands; commands=(
'info:Show system information (distro, compositor, driver)' \
'compositor:Show detected Wayland compositor and capabilities' \
'distro:Show detected Linux distribution' \
'optimize:Show platform optimization recommendations' \
    )
    _describe -t commands 'nvctl help system commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__system__subcmd__compositor_commands] )) ||
_nvctl__subcmd__help__subcmd__system__subcmd__compositor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help system compositor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__system__subcmd__distro_commands] )) ||
_nvctl__subcmd__help__subcmd__system__subcmd__distro_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help system distro commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__system__subcmd__info_commands] )) ||
_nvctl__subcmd__help__subcmd__system__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help system info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__system__subcmd__optimize_commands] )) ||
_nvctl__subcmd__help__subcmd__system__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help system optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__tui_commands] )) ||
_nvctl__subcmd__help__subcmd__tui_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help tui commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__upscaling_commands] )) ||
_nvctl__subcmd__help__subcmd__upscaling_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'profiles:' \
'auto-detect:' \
    )
    _describe -t commands 'nvctl help upscaling commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__upscaling__subcmd__auto-detect_commands] )) ||
_nvctl__subcmd__help__subcmd__upscaling__subcmd__auto-detect_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help upscaling auto-detect commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__upscaling__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__upscaling__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help upscaling disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__upscaling__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__upscaling__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help upscaling enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__upscaling__subcmd__profiles_commands] )) ||
_nvctl__subcmd__help__subcmd__upscaling__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help upscaling profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__upscaling__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__upscaling__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help upscaling status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__version_commands] )) ||
_nvctl__subcmd__help__subcmd__version_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help version commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__help__subcmd__vibrance_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__vrr_commands] )) ||
_nvctl__subcmd__help__subcmd__vrr_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'configure:' \
    )
    _describe -t commands 'nvctl help vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__vrr__subcmd__configure_commands] )) ||
_nvctl__subcmd__help__subcmd__vrr__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help vrr configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__vrr__subcmd__disable_commands] )) ||
_nvctl__subcmd__help__subcmd__vrr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help vrr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__vrr__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__vrr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help vrr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__vrr__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__vrr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help vrr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland_commands() {
    local commands; commands=(
'status:Show Wayland NVIDIA configuration status' \
'optimize:Apply optimal Wayland configuration' \
'export-env:Generate environment variables script' \
'switch-driver:Switch between nvidia-open and nvidia-dkms' \
'explicit-sync:Explicit sync management (reduces tearing on NVIDIA Wayland)' \
    )
    _describe -t commands 'nvctl help wayland commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync_commands() {
    local commands; commands=(
'status:Show explicit sync support status' \
'enable:Enable explicit sync in compositor' \
    )
    _describe -t commands 'nvctl help wayland explicit-sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync__subcmd__enable_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help wayland explicit-sync enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__explicit-sync__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help wayland explicit-sync status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__export-env_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__export-env_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help wayland export-env commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__optimize_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help wayland optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__status_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help wayland status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__help__subcmd__wayland__subcmd__switch-driver_commands] )) ||
_nvctl__subcmd__help__subcmd__wayland__subcmd__switch-driver_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl help wayland switch-driver commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__interactive_commands] )) ||
_nvctl__subcmd__interactive_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl interactive commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde_commands] )) ||
_nvctl__subcmd__kde_commands() {
    local commands; commands=(
'status:Show KDE compositor status' \
'gaming:Apply gaming preset (low latency, VRR, minimal effects)' \
'productivity:Apply productivity preset (balanced, full effects)' \
'power-save:Apply power saving preset' \
'setup-env:Setup NVIDIA environment variables for KDE' \
'set-vrr:Set VRR per display' \
'restart:Restart KWin compositor' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl kde commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__gaming_commands] )) ||
_nvctl__subcmd__kde__subcmd__gaming_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde gaming commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help_commands] )) ||
_nvctl__subcmd__kde__subcmd__help_commands() {
    local commands; commands=(
'status:Show KDE compositor status' \
'gaming:Apply gaming preset (low latency, VRR, minimal effects)' \
'productivity:Apply productivity preset (balanced, full effects)' \
'power-save:Apply power saving preset' \
'setup-env:Setup NVIDIA environment variables for KDE' \
'set-vrr:Set VRR per display' \
'restart:Restart KWin compositor' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl kde help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__gaming_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__gaming_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help gaming commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__power-save_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__power-save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help power-save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__productivity_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__productivity_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help productivity commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__restart_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__restart_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help restart commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__set-vrr_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__set-vrr_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help set-vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__setup-env_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__setup-env_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help setup-env commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__kde__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__power-save_commands] )) ||
_nvctl__subcmd__kde__subcmd__power-save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde power-save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__productivity_commands] )) ||
_nvctl__subcmd__kde__subcmd__productivity_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde productivity commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__restart_commands] )) ||
_nvctl__subcmd__kde__subcmd__restart_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde restart commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__set-vrr_commands] )) ||
_nvctl__subcmd__kde__subcmd__set-vrr_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde set-vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__setup-env_commands] )) ||
_nvctl__subcmd__kde__subcmd__setup-env_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde setup-env commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__kde__subcmd__status_commands] )) ||
_nvctl__subcmd__kde__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl kde status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor_commands] )) ||
_nvctl__subcmd__monitor_commands() {
    local commands; commands=(
'start:Start monitoring' \
'stop:Stop monitoring' \
'status:Show monitoring status' \
'tui:Launch TUI monitoring interface' \
'export:Export monitoring data' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__export_commands] )) ||
_nvctl__subcmd__monitor__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help_commands() {
    local commands; commands=(
'start:Start monitoring' \
'stop:Stop monitoring' \
'status:Show monitoring status' \
'tui:Launch TUI monitoring interface' \
'export:Export monitoring data' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl monitor help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help__subcmd__export_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help__subcmd__export_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor help export commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help__subcmd__start_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor help start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help__subcmd__stop_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor help stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__help__subcmd__tui_commands] )) ||
_nvctl__subcmd__monitor__subcmd__help__subcmd__tui_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor help tui commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__start_commands] )) ||
_nvctl__subcmd__monitor__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__status_commands] )) ||
_nvctl__subcmd__monitor__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__stop_commands] )) ||
_nvctl__subcmd__monitor__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitor__subcmd__tui_commands] )) ||
_nvctl__subcmd__monitor__subcmd__tui_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitor tui commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors_commands] )) ||
_nvctl__subcmd__monitors_commands() {
    local commands; commands=(
'status:Show current display configuration' \
'presets:List built-in monitor presets' \
'suggest:Suggest built-in monitor presets for the current setup' \
'preview:Preview a built-in monitor preset without applying it' \
'apply-preset:Apply a built-in monitor preset immediately' \
'save:Save current layout with a name' \
'load:Load and apply a saved layout' \
'list:List all saved layouts' \
'set-vrr:Set VRR for a specific display' \
'gamescope:Launch Gamescope on specific display' \
'auto:Auto-apply layout based on connected displays' \
'create-examples:Create example layouts' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl monitors commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__apply-preset_commands] )) ||
_nvctl__subcmd__monitors__subcmd__apply-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors apply-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__auto_commands] )) ||
_nvctl__subcmd__monitors__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__create-examples_commands] )) ||
_nvctl__subcmd__monitors__subcmd__create-examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors create-examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__gamescope_commands] )) ||
_nvctl__subcmd__monitors__subcmd__gamescope_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors gamescope commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help_commands() {
    local commands; commands=(
'status:Show current display configuration' \
'presets:List built-in monitor presets' \
'suggest:Suggest built-in monitor presets for the current setup' \
'preview:Preview a built-in monitor preset without applying it' \
'apply-preset:Apply a built-in monitor preset immediately' \
'save:Save current layout with a name' \
'load:Load and apply a saved layout' \
'list:List all saved layouts' \
'set-vrr:Set VRR for a specific display' \
'gamescope:Launch Gamescope on specific display' \
'auto:Auto-apply layout based on connected displays' \
'create-examples:Create example layouts' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl monitors help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__apply-preset_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__apply-preset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help apply-preset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__auto_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__create-examples_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__create-examples_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help create-examples commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__gamescope_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__gamescope_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help gamescope commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__load_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__load_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help load commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__presets_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__preview_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__save_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__set-vrr_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__set-vrr_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help set-vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__help__subcmd__suggest_commands] )) ||
_nvctl__subcmd__monitors__subcmd__help__subcmd__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors help suggest commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__list_commands] )) ||
_nvctl__subcmd__monitors__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__load_commands] )) ||
_nvctl__subcmd__monitors__subcmd__load_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors load commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__presets_commands] )) ||
_nvctl__subcmd__monitors__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__preview_commands] )) ||
_nvctl__subcmd__monitors__subcmd__preview_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors preview commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__save_commands] )) ||
_nvctl__subcmd__monitors__subcmd__save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__set-vrr_commands] )) ||
_nvctl__subcmd__monitors__subcmd__set-vrr_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors set-vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__status_commands] )) ||
_nvctl__subcmd__monitors__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__monitors__subcmd__suggest_commands] )) ||
_nvctl__subcmd__monitors__subcmd__suggest_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl monitors suggest commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__nvtop_commands] )) ||
_nvctl__subcmd__nvtop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl nvtop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd_commands] )) ||
_nvctl__subcmd__osd_commands() {
    local commands; commands=(
'enable:Enable OSD overlay' \
'disable:Disable OSD overlay' \
'status:Show OSD status and configuration' \
'config:Configure OSD settings' \
'add:Add metric to OSD' \
'remove:Remove metric from OSD' \
'metrics:List available metrics' \
'check:Check MangoHud installation status' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl osd commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__add_commands] )) ||
_nvctl__subcmd__osd__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__check_commands] )) ||
_nvctl__subcmd__osd__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd check commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__config_commands] )) ||
_nvctl__subcmd__osd__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__disable_commands] )) ||
_nvctl__subcmd__osd__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__enable_commands] )) ||
_nvctl__subcmd__osd__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help_commands] )) ||
_nvctl__subcmd__osd__subcmd__help_commands() {
    local commands; commands=(
'enable:Enable OSD overlay' \
'disable:Disable OSD overlay' \
'status:Show OSD status and configuration' \
'config:Configure OSD settings' \
'add:Add metric to OSD' \
'remove:Remove metric from OSD' \
'metrics:List available metrics' \
'check:Check MangoHud installation status' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl osd help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__add_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__check_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__check_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help check commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__config_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__config_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help config commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__metrics_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__metrics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help metrics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__remove_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__osd__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__metrics_commands] )) ||
_nvctl__subcmd__osd__subcmd__metrics_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd metrics commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__remove_commands] )) ||
_nvctl__subcmd__osd__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__osd__subcmd__status_commands] )) ||
_nvctl__subcmd__osd__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl osd status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock_commands] )) ||
_nvctl__subcmd__overclock_commands() {
    local commands; commands=(
'info:' \
'apply:' \
'profile:' \
'stress-test:' \
'auto:Automated overclocking wizard with safety features' \
'reset:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl overclock commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__apply_commands] )) ||
_nvctl__subcmd__overclock__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__auto_commands] )) ||
_nvctl__subcmd__overclock__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help_commands() {
    local commands; commands=(
'info:' \
'apply:' \
'profile:' \
'stress-test:' \
'auto:Automated overclocking wizard with safety features' \
'reset:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl overclock help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__apply_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__auto_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__auto_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help auto commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__profile_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__reset_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__help__subcmd__stress-test_commands] )) ||
_nvctl__subcmd__overclock__subcmd__help__subcmd__stress-test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock help stress-test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__info_commands] )) ||
_nvctl__subcmd__overclock__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__profile_commands] )) ||
_nvctl__subcmd__overclock__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__reset_commands] )) ||
_nvctl__subcmd__overclock__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__overclock__subcmd__stress-test_commands] )) ||
_nvctl__subcmd__overclock__subcmd__stress-test_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl overclock stress-test commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough_commands] )) ||
_nvctl__subcmd__passthrough_commands() {
    local commands; commands=(
'status:Show GPU passthrough status' \
'list:List all NVIDIA GPUs and their PCI addresses' \
'iommu:Show IOMMU groups' \
'bind-vfio:Bind GPU to VFIO driver' \
'unbind-vfio:Unbind GPU from VFIO' \
'persistent:Setup persistent VFIO binding' \
'test-container:Test GPU passthrough to Docker container' \
'qemu-command:Generate QEMU command for GPU passthrough' \
'hugepages:Setup hugepages for VM performance' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl passthrough commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__bind-vfio_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__bind-vfio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough bind-vfio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help_commands() {
    local commands; commands=(
'status:Show GPU passthrough status' \
'list:List all NVIDIA GPUs and their PCI addresses' \
'iommu:Show IOMMU groups' \
'bind-vfio:Bind GPU to VFIO driver' \
'unbind-vfio:Unbind GPU from VFIO' \
'persistent:Setup persistent VFIO binding' \
'test-container:Test GPU passthrough to Docker container' \
'qemu-command:Generate QEMU command for GPU passthrough' \
'hugepages:Setup hugepages for VM performance' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl passthrough help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__bind-vfio_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__bind-vfio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help bind-vfio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__hugepages_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__hugepages_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help hugepages commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__iommu_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__iommu_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help iommu commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__persistent_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__persistent_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help persistent commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__qemu-command_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__qemu-command_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help qemu-command commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__test-container_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__test-container_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help test-container commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__help__subcmd__unbind-vfio_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__help__subcmd__unbind-vfio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough help unbind-vfio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__hugepages_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__hugepages_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough hugepages commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__iommu_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__iommu_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough iommu commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__list_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__persistent_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__persistent_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough persistent commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__qemu-command_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__qemu-command_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough qemu-command commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__status_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__test-container_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__test-container_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough test-container commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__passthrough__subcmd__unbind-vfio_commands] )) ||
_nvctl__subcmd__passthrough__subcmd__unbind-vfio_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl passthrough unbind-vfio commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power_commands] )) ||
_nvctl__subcmd__power_commands() {
    local commands; commands=(
'status:Show current power settings' \
'limit:Set GPU power limit (percentage)' \
'profile:Configure power profile' \
'persistence:Power persistence settings' \
'monitor:Monitor power usage' \
'automate:Automate power management' \
'curve:Manage power limit curves (temperature-based dynamic power)' \
'schedule:Schedule power profiles by time' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__automate_commands] )) ||
_nvctl__subcmd__power__subcmd__automate_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power automate commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve_commands] )) ||
_nvctl__subcmd__power__subcmd__curve_commands() {
    local commands; commands=(
'show:Show current power curve' \
'edit:Edit power curve interactively' \
'add:Add a curve point (temperature, power_limit)' \
'remove:Remove a curve point' \
'enable:Enable curve-based power management' \
'disable:Disable curve-based power management' \
'reset:Reset to default curve' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power curve commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__add_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__disable_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__edit_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__enable_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help_commands() {
    local commands; commands=(
'show:Show current power curve' \
'edit:Edit power curve interactively' \
'add:Add a curve point (temperature, power_limit)' \
'remove:Remove a curve point' \
'enable:Enable curve-based power management' \
'disable:Disable curve-based power management' \
'reset:Reset to default curve' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power curve help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__add_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__edit_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__remove_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__reset_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__show_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__help__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve help show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__remove_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__reset_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__curve__subcmd__show_commands] )) ||
_nvctl__subcmd__power__subcmd__curve__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power curve show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help_commands] )) ||
_nvctl__subcmd__power__subcmd__help_commands() {
    local commands; commands=(
'status:Show current power settings' \
'limit:Set GPU power limit (percentage)' \
'profile:Configure power profile' \
'persistence:Power persistence settings' \
'monitor:Monitor power usage' \
'automate:Automate power management' \
'curve:Manage power limit curves (temperature-based dynamic power)' \
'schedule:Schedule power profiles by time' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__automate_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__automate_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help automate commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve_commands() {
    local commands; commands=(
'show:Show current power curve' \
'edit:Edit power curve interactively' \
'add:Add a curve point (temperature, power_limit)' \
'remove:Remove a curve point' \
'enable:Enable curve-based power management' \
'disable:Disable curve-based power management' \
'reset:Reset to default curve' \
    )
    _describe -t commands 'nvctl power help curve commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__add_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__disable_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__edit_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__edit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve edit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__enable_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__remove_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__reset_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__reset_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve reset commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__show_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__curve__subcmd__show_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help curve show commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__limit_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__limit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help limit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__monitor_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__persistence_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__persistence_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help persistence commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__profile_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__schedule_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__schedule_commands() {
    local commands; commands=(
'list:List all scheduled power profiles' \
'add:Add a scheduled power profile' \
'remove:Remove a schedule' \
'enable:Enable scheduled power management' \
'disable:Disable scheduled power management' \
    )
    _describe -t commands 'nvctl power help schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__add_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help schedule add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__disable_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help schedule disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__enable_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help schedule enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__list_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help schedule list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__remove_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__schedule__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help schedule remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__power__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__limit_commands] )) ||
_nvctl__subcmd__power__subcmd__limit_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power limit commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__monitor_commands] )) ||
_nvctl__subcmd__power__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__persistence_commands] )) ||
_nvctl__subcmd__power__subcmd__persistence_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power persistence commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__profile_commands] )) ||
_nvctl__subcmd__power__subcmd__profile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule_commands() {
    local commands; commands=(
'list:List all scheduled power profiles' \
'add:Add a scheduled power profile' \
'remove:Remove a schedule' \
'enable:Enable scheduled power management' \
'disable:Disable scheduled power management' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power schedule commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__add_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__disable_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__enable_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help_commands() {
    local commands; commands=(
'list:List all scheduled power profiles' \
'add:Add a scheduled power profile' \
'remove:Remove a schedule' \
'enable:Enable scheduled power management' \
'disable:Disable scheduled power management' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power schedule help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__add_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__add_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule help add commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__list_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule help list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__remove_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__help__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule help remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__list_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule list commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__schedule__subcmd__remove_commands] )) ||
_nvctl__subcmd__power__subcmd__schedule__subcmd__remove_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power schedule remove commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power__subcmd__status_commands] )) ||
_nvctl__subcmd__power__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile_commands] )) ||
_nvctl__subcmd__power-profile_commands() {
    local commands; commands=(
'status:Show current power profile status' \
'set:Set system power profile' \
'create-activity:Create activity-based profile' \
'apply:Apply profile for current activity' \
'monitor:Monitor and auto-switch on activity changes' \
'auto-power:Monitor and auto-switch on AC/Battery changes' \
'idle:Enable idle detection and power reduction' \
'create-defaults:Create default activity profiles' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power-profile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__apply_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__auto-power_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__auto-power_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile auto-power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__create-activity_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__create-activity_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile create-activity commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__create-defaults_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__create-defaults_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile create-defaults commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help_commands() {
    local commands; commands=(
'status:Show current power profile status' \
'set:Set system power profile' \
'create-activity:Create activity-based profile' \
'apply:Apply profile for current activity' \
'monitor:Monitor and auto-switch on activity changes' \
'auto-power:Monitor and auto-switch on AC/Battery changes' \
'idle:Enable idle detection and power reduction' \
'create-defaults:Create default activity profiles' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl power-profile help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__apply_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__apply_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help apply commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__auto-power_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__auto-power_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help auto-power commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__create-activity_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__create-activity_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help create-activity commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__create-defaults_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__create-defaults_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help create-defaults commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__idle_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__idle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help idle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__monitor_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__set_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__idle_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__idle_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile idle commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__monitor_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__monitor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile monitor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__set_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__set_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile set commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__power-profile__subcmd__status_commands] )) ||
_nvctl__subcmd__power-profile__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl power-profile status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording_commands] )) ||
_nvctl__subcmd__recording_commands() {
    local commands; commands=(
'start:Start recording' \
'stop:Stop recording' \
'status:Show recording status' \
'instant-replay:Start instant replay' \
'save:Save instant replay clip' \
'presets:List available presets' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl recording commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help_commands] )) ||
_nvctl__subcmd__recording__subcmd__help_commands() {
    local commands; commands=(
'start:Start recording' \
'stop:Stop recording' \
'status:Show recording status' \
'instant-replay:Start instant replay' \
'save:Save instant replay clip' \
'presets:List available presets' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl recording help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__instant-replay_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__instant-replay_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help instant-replay commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__presets_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__save_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__start_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__help__subcmd__stop_commands] )) ||
_nvctl__subcmd__recording__subcmd__help__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording help stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__instant-replay_commands] )) ||
_nvctl__subcmd__recording__subcmd__instant-replay_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording instant-replay commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__presets_commands] )) ||
_nvctl__subcmd__recording__subcmd__presets_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording presets commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__save_commands] )) ||
_nvctl__subcmd__recording__subcmd__save_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording save commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__start_commands] )) ||
_nvctl__subcmd__recording__subcmd__start_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording start commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__status_commands] )) ||
_nvctl__subcmd__recording__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__recording__subcmd__stop_commands] )) ||
_nvctl__subcmd__recording__subcmd__stop_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl recording stop commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders_commands] )) ||
_nvctl__subcmd__shaders_commands() {
    local commands; commands=(
'stats:Show shader cache statistics' \
'clear:Clear all shader caches' \
'optimize:Optimize shader compilation settings' \
'precompile:Precompile shaders for a game' \
'open:Open shader cache folder' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl shaders commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__clear_commands] )) ||
_nvctl__subcmd__shaders__subcmd__clear_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders clear commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help_commands() {
    local commands; commands=(
'stats:Show shader cache statistics' \
'clear:Clear all shader caches' \
'optimize:Optimize shader compilation settings' \
'precompile:Precompile shaders for a game' \
'open:Open shader cache folder' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl shaders help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help__subcmd__clear_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help__subcmd__clear_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders help clear commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help__subcmd__open_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help__subcmd__open_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders help open commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help__subcmd__optimize_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders help optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help__subcmd__precompile_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help__subcmd__precompile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders help precompile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__help__subcmd__stats_commands] )) ||
_nvctl__subcmd__shaders__subcmd__help__subcmd__stats_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders help stats commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__open_commands] )) ||
_nvctl__subcmd__shaders__subcmd__open_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders open commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__optimize_commands] )) ||
_nvctl__subcmd__shaders__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__precompile_commands] )) ||
_nvctl__subcmd__shaders__subcmd__precompile_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders precompile commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__shaders__subcmd__stats_commands] )) ||
_nvctl__subcmd__shaders__subcmd__stats_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl shaders stats commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system_commands] )) ||
_nvctl__subcmd__system_commands() {
    local commands; commands=(
'info:Show system information (distro, compositor, driver)' \
'compositor:Show detected Wayland compositor and capabilities' \
'distro:Show detected Linux distribution' \
'optimize:Show platform optimization recommendations' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl system commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__compositor_commands] )) ||
_nvctl__subcmd__system__subcmd__compositor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system compositor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__distro_commands] )) ||
_nvctl__subcmd__system__subcmd__distro_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system distro commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__help_commands] )) ||
_nvctl__subcmd__system__subcmd__help_commands() {
    local commands; commands=(
'info:Show system information (distro, compositor, driver)' \
'compositor:Show detected Wayland compositor and capabilities' \
'distro:Show detected Linux distribution' \
'optimize:Show platform optimization recommendations' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl system help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__help__subcmd__compositor_commands] )) ||
_nvctl__subcmd__system__subcmd__help__subcmd__compositor_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system help compositor commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__help__subcmd__distro_commands] )) ||
_nvctl__subcmd__system__subcmd__help__subcmd__distro_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system help distro commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__system__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__help__subcmd__info_commands] )) ||
_nvctl__subcmd__system__subcmd__help__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system help info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__help__subcmd__optimize_commands] )) ||
_nvctl__subcmd__system__subcmd__help__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system help optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__info_commands] )) ||
_nvctl__subcmd__system__subcmd__info_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system info commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__system__subcmd__optimize_commands] )) ||
_nvctl__subcmd__system__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl system optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__tui_commands] )) ||
_nvctl__subcmd__tui_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl tui commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling_commands] )) ||
_nvctl__subcmd__upscaling_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'profiles:' \
'auto-detect:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl upscaling commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__auto-detect_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__auto-detect_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling auto-detect commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__disable_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__enable_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'profiles:' \
'auto-detect:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl upscaling help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help__subcmd__auto-detect_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help__subcmd__auto-detect_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling help auto-detect commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help__subcmd__profiles_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling help profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__profiles_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__profiles_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling profiles commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__upscaling__subcmd__status_commands] )) ||
_nvctl__subcmd__upscaling__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl upscaling status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__version_commands] )) ||
_nvctl__subcmd__version_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl version commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vibrance_commands] )) ||
_nvctl__subcmd__vibrance_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vibrance commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr_commands] )) ||
_nvctl__subcmd__vrr_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'configure:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl vrr commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__configure_commands] )) ||
_nvctl__subcmd__vrr__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__disable_commands] )) ||
_nvctl__subcmd__vrr__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__enable_commands] )) ||
_nvctl__subcmd__vrr__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__help_commands] )) ||
_nvctl__subcmd__vrr__subcmd__help_commands() {
    local commands; commands=(
'status:' \
'enable:' \
'disable:' \
'configure:' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl vrr help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__help__subcmd__configure_commands] )) ||
_nvctl__subcmd__vrr__subcmd__help__subcmd__configure_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr help configure commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__help__subcmd__disable_commands] )) ||
_nvctl__subcmd__vrr__subcmd__help__subcmd__disable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr help disable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__vrr__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__vrr__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__vrr__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__vrr__subcmd__status_commands] )) ||
_nvctl__subcmd__vrr__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl vrr status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland_commands] )) ||
_nvctl__subcmd__wayland_commands() {
    local commands; commands=(
'status:Show Wayland NVIDIA configuration status' \
'optimize:Apply optimal Wayland configuration' \
'export-env:Generate environment variables script' \
'switch-driver:Switch between nvidia-open and nvidia-dkms' \
'explicit-sync:Explicit sync management (reduces tearing on NVIDIA Wayland)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl wayland commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync_commands() {
    local commands; commands=(
'status:Show explicit sync support status' \
'enable:Enable explicit sync in compositor' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl wayland explicit-sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__enable_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland explicit-sync enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help_commands() {
    local commands; commands=(
'status:Show explicit sync support status' \
'enable:Enable explicit sync in compositor' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl wayland explicit-sync help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help__subcmd__enable_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland explicit-sync help enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland explicit-sync help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland explicit-sync help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__status_commands] )) ||
_nvctl__subcmd__wayland__subcmd__explicit-sync__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland explicit-sync status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__export-env_commands] )) ||
_nvctl__subcmd__wayland__subcmd__export-env_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland export-env commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help_commands() {
    local commands; commands=(
'status:Show Wayland NVIDIA configuration status' \
'optimize:Apply optimal Wayland configuration' \
'export-env:Generate environment variables script' \
'switch-driver:Switch between nvidia-open and nvidia-dkms' \
'explicit-sync:Explicit sync management (reduces tearing on NVIDIA Wayland)' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'nvctl wayland help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync_commands() {
    local commands; commands=(
'status:Show explicit sync support status' \
'enable:Enable explicit sync in compositor' \
    )
    _describe -t commands 'nvctl wayland help explicit-sync commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync__subcmd__enable_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync__subcmd__enable_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help explicit-sync enable commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync__subcmd__status_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__explicit-sync__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help explicit-sync status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__export-env_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__export-env_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help export-env commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__help_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help help commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__optimize_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__status_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__help__subcmd__switch-driver_commands] )) ||
_nvctl__subcmd__wayland__subcmd__help__subcmd__switch-driver_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland help switch-driver commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__optimize_commands] )) ||
_nvctl__subcmd__wayland__subcmd__optimize_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland optimize commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__status_commands] )) ||
_nvctl__subcmd__wayland__subcmd__status_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland status commands' commands "$@"
}
(( $+functions[_nvctl__subcmd__wayland__subcmd__switch-driver_commands] )) ||
_nvctl__subcmd__wayland__subcmd__switch-driver_commands() {
    local commands; commands=()
    _describe -t commands 'nvctl wayland switch-driver commands' commands "$@"
}

if [ "$funcstack[1]" = "_nvctl" ]; then
    _nvctl "$@"
else
    compdef _nvctl nvctl
fi
