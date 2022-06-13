
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
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Set delimiter string to print after each FIX field.')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'Set delimiter string to print after each FIX field.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Translate the values of some tags. (for Side: 1 -> Buy)')
            [CompletionResult]::new('--value', 'value', [CompletionResultType]::ParameterName, 'Translate the values of some tags. (for Side: 1 -> Buy)')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Strip the whitespace around the = in each field. Less human readable but closer to real FIX.')
            [CompletionResult]::new('--strip', 'strip', [CompletionResultType]::ParameterName, 'Strip the whitespace around the = in each field. Less human readable but closer to real FIX.')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
