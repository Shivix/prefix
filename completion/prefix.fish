complete -c prefix -s c -l color -d 'Adds colour to the delimiter and = in for FIX fields, auto will colour only when printing directly into a tty' -r -f -a "always\t''
auto\t''
never\t''"
complete -c prefix -s d -l delimiter -d 'Set delimiter string to print after each FIX field' -r
complete -c prefix -s S -l summary -d 'Summarise each fix message based on a template, if summary is provided with no template then it uses \'35\'' -r
complete -c prefix -s o -l only-fix -d 'Only print FIX messages'
complete -c prefix -l porcelain -d 'print FIX messages closer to standard format, same as --delimiter \\x01 --strip'
complete -c prefix -s r -l repeating -d 'Combine any repeating groups into a single field with a comma delimited value'
complete -c prefix -s f -l strict -d 'Only consider full FIX messages containing both BeginString and Checksum'
complete -c prefix -s s -l strip -d 'Strip the whitespace around the = in each field'
complete -c prefix -s t -l tag -d 'Translate tag numbers on non FIX message lines, if the entire line matches a tag name it will print it\'s number'
complete -c prefix -s v -l value -d 'Translate the values of some tags (for Side: 1 -> Buy)'
complete -c prefix -s h -l help -d 'Print help'
complete -c prefix -s V -l version -d 'Print version'
