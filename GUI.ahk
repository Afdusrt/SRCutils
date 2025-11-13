;;;;;;;;;;;;;;;;;
Gui, Font, s12 Bold, Comic Sans MS
Gui, Add, Text,, EDL MODE:
Gui, Font, s8 Normal, Comic Sans MS

Gui, Add, Text,, Videolink:
Gui, Add, Edit, w280 vVideolink, https://www.youtube.com/watch?v=00000000000

Gui, Add, Text,, Path to edl file:
Gui, Add, Edit, w280 vEdlPath, edl.edl

Gui, Add, Button, w280 gRunEDLMODE, Run EDL MODE
;;;;;;;;;;;;;;;;;

;;;;;;;;;;;;;;;;;
Gui, Font, s12 Bold, Comic Sans MS
Gui, Add, Text,, SUB MODE:
Gui, Font, s8 Normal, Comic Sans MS

Gui, Add, Text,, Game abbreviation:
Gui, Add, Edit, w280 vGameAbbreviation

Gui, Add, Text,, Dsv file path:
Gui, Add, Edit, w280 vDsvPath, output.csv

Gui, Add, Text,, Example command file path:
Gui, Add, Edit, w280 vExampleCommandPath, example_command.txt

Gui, Add, Text,, Modifier OPTIONAL:
Gui, Add, Edit, w280 vModifier

Gui, Add, Button, w280 gRunSUBMODE, Run SUB MODE
;;;;;;;;;;;;;;;;;

Gui, Show, w300 h500, SRCutils GUI
Return

;;;;;;;;;;;;;;;;;
RunEDLMODE:
    Gui, Submit, NoHide
	Run, % "cmd.exe /c ""SRCutils-win-x86_64.exe edl " Videolink " " EdlPath " & pause"""
Return
;;;;;;;;;;;;;;;;;

;;;;;;;;;;;;;;;;;
RunSUBMODE:
    Gui, Submit, NoHide
	Run, % "cmd.exe /c ""SRCutils-win-x86_64.exe sub " GameAbbreviation " " DsvPath " " ExampleCommandPath " " Modifier "& pause"""
Return
;;;;;;;;;;;;;;;;;

GuiClose:
ExitApp
