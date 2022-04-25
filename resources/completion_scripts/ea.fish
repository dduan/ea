complete -c ea -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c ea -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ea -n "__fish_use_subcommand" -f -a "run"
complete -c ea -n "__fish_use_subcommand" -f -a "list"
complete -c ea -n "__fish_use_subcommand" -f -a "print"
complete -c ea -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ea -n "__fish_seen_subcommand_from run" -l debug -r -F
complete -c ea -n "__fish_seen_subcommand_from run" -s h -l help -d 'Print help information'
complete -c ea -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c ea -n "__fish_seen_subcommand_from print" -s h -l help -d 'Print help information'
