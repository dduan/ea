
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
            cand run 'run'
            cand list 'list'
            cand print 'print'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'ea;run'= {
            cand --debug 'debug'
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
