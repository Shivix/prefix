Output prefix.gif

Set FontSize 26
Set Width 1200
Set Height 600
Set CursorBlink false

# Not yet released
#Env PROMPT "$ "

Type@0ms `FIX_MSG="8=FIX.4.4|35=8|34=42|49=SENDER|56=TARGET|55=EURUSD|44=1.032|38=10000|54=1|150=F|10=123|"` Enter
Sleep 1s
Type@100ms "prefix $FIX_MSG -v" Sleep 1s  Enter

Sleep 10s
