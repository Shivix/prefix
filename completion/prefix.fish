complete -c prefix -s c -l color -d 'Adds colour to the delimiter and = in for FIX fields, auto will colour only when printing directly into a tty' -r -f -a "{always\t'',auto\t'',never\t''}"
complete -c prefix -s d -l delimiter -d 'Set delimiter string to print after each FIX field' -r
complete -c prefix -s S -l summary -d 'Summarise each fix message based on an optional template' -r
complete -c prefix -s o -l only-fix -d 'Only print FIX messages'
complete -c prefix -s s -l strip -d 'Strip the whitespace around the = in each field'
complete -c prefix -s t -l tag -d 'Translate tag numbers on non FIX message lines'
complete -c prefix -s v -l value -d 'Translate the values of some tags (for Side: 1 -> Buy)'
complete -c prefix -s h -l help -d 'Print help'
complete -c prefix -s V -l version -d 'Print version'
