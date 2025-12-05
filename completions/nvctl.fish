# nvctl fish completion

# Main commands
complete -c nvctl -f -n '__fish_use_subcommand' -a 'gpu' -d 'GPU management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'display' -d 'Display management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'vibrance' -d 'Digital vibrance'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'fan' -d 'Fan control'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'overclock' -d 'Overclocking'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'vrr' -d 'VRR/G-SYNC'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'monitor' -d 'Live monitoring'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'gaming' -d 'Gaming optimizations'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'recording' -d 'Recording & streaming'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'container' -d 'Container runtime'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'drivers' -d 'Driver management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'driver' -d 'Driver capabilities & validation'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'power' -d 'Power management'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'system' -d 'System information'
complete -c nvctl -f -n '__fish_use_subcommand' -a 'osd' -d 'On-Screen Display'

# Driver subcommands
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'status' -d 'Driver status'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'install' -d 'Install drivers'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'update' -d 'Update drivers'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'rollback' -d 'Rollback drivers'
complete -c nvctl -f -n '__fish_seen_subcommand_from drivers' -a 'generate-completions' -d 'Shell completions'

# Driver capabilities subcommands
complete -c nvctl -f -n '__fish_seen_subcommand_from driver' -a 'info' -d 'Show driver capabilities'
complete -c nvctl -f -n '__fish_seen_subcommand_from driver' -a 'validate' -d 'Validate target branch'
complete -c nvctl -f -n '__fish_seen_subcommand_from driver validate' -l driver -d 'Driver major version (e.g. 590)'

# GPU subcommands
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'info' -d 'Show GPU information'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'stat' -d 'Live GPU monitoring'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'list' -d 'List GPUs'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'select' -d 'Select GPU'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'benchmark' -d 'GPU benchmark'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'watch' -d 'Watch metrics'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'export' -d 'Export metrics'
complete -c nvctl -f -n '__fish_seen_subcommand_from gpu' -a 'stress' -d 'Stress test'

# Global options
complete -c nvctl -s v -l verbose -d 'Verbose output'
complete -c nvctl -l format -d 'Output format'
complete -c nvctl -l no-color -d 'Disable colors'
complete -c nvctl -l driver -d 'Driver major version'
