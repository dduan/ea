
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'ea' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'ea'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'ea' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Run EXECUTABLE through `ea`. Expecting its output to be the format of STYLE. Arguments for EXECUTABLE must come after `--`. For example, `rg Vec src` becomes:')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List locations found from the latest `ea run` output. This is the default subcommand. Running `ea` is the same as running `ea list`')
            [CompletionResult]::new('print', 'print', [CompletionResultType]::ParameterValue, 'Print the location info associated with NUMBER. Optionally, customize the output FORMAT. Also availble as the shorthand `ea p ...`')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'ea;run' {
            [CompletionResult]::new('--debug', 'debug', [CompletionResultType]::ParameterName, 'Write debug info at <debug_files_base_name.*>')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ea;list' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ea;print' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'ea;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
