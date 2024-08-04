
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'prefix' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'prefix'
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
        'prefix' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Adds colour to the delimiter and = in for FIX fields, auto will colour only when printing directly into a tty')
            [CompletionResult]::new('--color', 'color', [CompletionResultType]::ParameterName, 'Adds colour to the delimiter and = in for FIX fields, auto will colour only when printing directly into a tty')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Set delimiter string to print after each FIX field')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'Set delimiter string to print after each FIX field')
            [CompletionResult]::new('-S', 'S ', [CompletionResultType]::ParameterName, 'Summarise each fix message based on an optional template')
            [CompletionResult]::new('--summary', 'summary', [CompletionResultType]::ParameterName, 'Summarise each fix message based on an optional template')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Only print FIX messages')
            [CompletionResult]::new('--only-fix', 'only-fix', [CompletionResultType]::ParameterName, 'Only print FIX messages')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Strip the whitespace around the = in each field')
            [CompletionResult]::new('--strip', 'strip', [CompletionResultType]::ParameterName, 'Strip the whitespace around the = in each field')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Translate tag numbers on non FIX message lines')
            [CompletionResult]::new('--tag', 'tag', [CompletionResultType]::ParameterName, 'Translate tag numbers on non FIX message lines')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Translate the values of some tags (for Side: 1 -> Buy)')
            [CompletionResult]::new('--value', 'value', [CompletionResultType]::ParameterName, 'Translate the values of some tags (for Side: 1 -> Buy)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
