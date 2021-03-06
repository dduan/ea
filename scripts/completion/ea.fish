complete -c ea -n "__fish_use_subcommand" -s h -l help -d 'Print help information'
complete -c ea -n "__fish_use_subcommand" -s V -l version -d 'Print version information'
complete -c ea -n "__fish_use_subcommand" -f -a "run" -d 'Run EXECUTABLE through `ea`. Expecting its output to be the format of STYLE. Arguments for EXECUTABLE must come after `--`. For example, `rg Vec src` becomes:'
complete -c ea -n "__fish_use_subcommand" -f -a "list" -d 'List locations found from the latest `ea run` output. This is the default subcommand. Running `ea` is the same as running `ea list`'
complete -c ea -n "__fish_use_subcommand" -f -a "print" -d 'Print the location info associated with NUMBER. Optionally, customize the output FORMAT. Also availble as the shorthand `ea p ...`'
complete -c ea -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c ea -n "__fish_seen_subcommand_from run" -l debug -d 'Write debug info at <debug_files_base_name.*>' -r -F
complete -c ea -n "__fish_seen_subcommand_from run" -s h -l help -d 'Print help information'
complete -c ea -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help information'
complete -c ea -n "__fish_seen_subcommand_from print" -s h -l help -d 'Print help information'
