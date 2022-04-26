
use builtin;
use str;

set edit:completion:arg-completer[ea] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'ea'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'ea'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand run 'Run EXECUTABLE through `ea`. Expecting its output to be the format of STYLE. Arguments for EXECUTABLE must come after `--`. For example, `rg Vec src` becomes:'
            cand list 'List locations found from the latest `ea run` output. This is the default subcommand. Running `ea` is the same as running `ea list`'
            cand print 'Print the location info associated with NUMBER. Optionally, customize the output FORMAT. Also availble as the shorthand `ea p ...`'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ea;run'= {
            cand --debug 'Write debug info at <debug_files_base_name.*>'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ea;list'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ea;print'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'ea;help'= {
        }
    ]
    $completions[$command]
}
