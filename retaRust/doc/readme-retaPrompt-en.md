+++
title = "readme for retaPrompt in markdown"
author = ["tracehugo"]
date = 2022-12-02T00:00:00+01:00
tags = ["reta", "retaprompt", "readme"]
categories = ["development"]
draft = false
weight = -7
+++

<div class="ox-hugo-toc toc">

<div class="heading">Table of Contents</div>

- [commands](#befehle)
    - [output-comands](#ausgabe-befehle)
    - [mathematically output-comands](#mathematically-ausgabe-befehle)
    - [number ranges. lines specifications](#zahlenbereiche-zeilenangaben)
    - [the commands 15_.... and PROP.....](#die-befehle-15-dot-dot-dot-dot)
    - [other commands](#sonstige-befehle)
    - [save-commands](#speicher-befehle)
- [running retaPrompt](#retaprompt-starten)

</div>
<!--endtoc-->


## commands {#befehle}

-   "help" prints this help info here
-   "commands" prints the list of possible commands of ReTaPrompt
-   "shortcuts" prints the list of possible commands having just one letter of ReTaPrompt
-   "-h" oder "-help" not in retaPrompt as a command, but before it as a program-start argument for retaPrompt, outputs which program-start arguments are possible for ReTa-Prompt.

### output commands {#output commands}

-   the command reta is like not starting retaPrompt or rp or rpl but the CLI command reta.
    reta -h prints the help of reta. The command "reta", with its parameters, cannot be mixed with the other output commands.
-   Line specifications are e.g. a number or a range of numbers like 2-5 or these things separated with comma like 1,3-5,20
    For the command "u" and "i", i.e. "universe" and "intent", you can also specify fractions, like 2/3,4/5,1/2.
-   These output commands can be combined in one input line.
-   "i" or "intention" or "motive" or "motive" outputs the one column of intrinsic intentions of the star polygons, together with a line specification, (example a1 or a3/2)
-   "spheres" or "circles" displays infos about those some kind of things for Types n
-   "u" or "universum" outputs the one column of the universal structurals or transcendentals of the star polygons, together with a line specification (example u2 or u2/2,1/2,3/2)
-   "t" or "thomas" outputs the one column of the thomas vanglium which corresponds to the intrinsic intentions of the star polygons in coded form, together with a line specification
-   "M" resp. "mind" shows the universal mind (examples: M1/2 or M3)
-   "C" resp. "consciousness" shows consciousness (examples: C1/2 or C3)
-   "E" resp. "emotion" shows emotions n or 1/n or n/m (examples: E1/2 or E3)
-   "I" resp. "impulses" shows the impulses (examples: I1/2 or I3)
-   "T" resp. "truth" shows the reality truths (examples: T1/2 or T3)
-   "size" shows the structure size of n or 1/n or n/m
-   "complex" shows the complexity steps of n or 1/n
-   "ee" hides headlines
-   "equality" resp. "freedom" shows that information (examples: "freedom 9")
-   "m" resp. multiple" causes in output commands except "reta" that their line specifications e.g. 7 mean not only line 7, but all multiples of these line specifications as well, i.e. also 14,21, etc.
-   "single" is the standard for short commands: that line specifications do not mean multiples, but single lines.
-   "moon", together with a line specification, outputs information about celestial bodies: such as moons, planets, suns.
-   "all", together with a line specification, simply outputs all columns. This takes time.
-   "primecross" outputs, together with a row specification, the columns of the prime cross algorithm. These are related to the columns above ghost (15). That takes.
-   "r" or "direction" gives, together with a row specification, the columns which output how far a row works per outside, per inside, per sideways, against sideways.
-   Command "u" and "i", i.e. "universe" and "intention", also outputs the reciprocal of the given fractions, e.g. for 2/3,3/4 also 3/2,4/3.
-   Some of the short commands consisting of letters like "i" or "u" can also be used as commands without spaces in between. Example: instead of "a u 1,2" also "au1,2" works.
-   Short command "e" makes that lines with almost no information are not displayed, i.e. lines with only a minus or question mark.
-   The command "distance", together 2 other information separated by spaces, a number and two numbers between which there is immediately a hyphen, e.g. "distance 7 17-25": simply calculates the subtraction between the number 7 and the number range 17 to 25.
-   Command "clear" clears screen.
-   You could also use Python regexes: command '2 r"inten"' results into': '1 intent intentions'. Much more complicated stuff is possible! This regex syntax is also possible before and after an equal sign, so paramater and its value. r"" Does also work between commas.
-   Instead using a regex, you could set a star (*) before or after the eqal sign. This means everything is meant.

###mathematical-output-commands {#mathematical-output-commands}

-   "prime" or "prime factorization" outputs the prime factors of a given number.
-   "multis" outputs all multiplications of two numbers that result in the specified number.
-   "abc" or "abcd" outputs, together with an indication of letters, which number corresponds to which letter.
-   "p" or "mulpri" means that both commands are meant: "prim" and "multis".
-   "modulo" outputs the remainders for divisions for a number.


### number ranges, line specifications {#number-ranges-line-specifications}

Line specifications or number ranges for other specifications can be (without quotes):

-   "2" this would be line 2
-   "3-5" this would be line 3,4,5
-   "4,9" this would be line 4 and 9
-   "2,3-5" that would be line 2,3,4,5
-   "5+2+4" that would be the neighbors of 5 with distance 2 and 4, so 1,3,7,9
-   "10+2" As an indication, if the multiples were meant (There are then always multiples here at "m10+2" [and not only at inputs for explicit multiples, but then everywhere]), that would be not only 8 and 12, but also 18 and 22, and 28 and 32, and so on.
-   In the readme of reta, instead of retaPrompt, this is explained again
-   5,m20-22 means line 5 and also all multiples of 20,21,22, e.g. 40,42,44
-   20,m10 means all multiples of 10 without the 20 in it
-   with fractions this m syntax works exactly as well
-   "1/2,3" means line 3 for star polygons n and 1/2 means here line 2 for uniform polygons 1/n
-   "1/2-3/3" draws a square, so to speak, and means fractions 1/2,2/2,3/2,1/3,2/3,3/3, that is the area between the fractions.
-   "4/5+2/2" chooses the fraction 2/3,2/7,6/3,6/7: thus forms the distance 2 back and forth for the numerator and denominator
-   The syntax, where a minus and plus occurs, can also be combined together. For integer values as well as for fractions.
-   This syntax can be used several times one after the other, if it is separated with commas. Within these commas either fractions or integers may be contained as range specification and thus be specified together by many commas.
-   This syntax for fractions and integers works with the "m" or "multiple" command and with this minus syntax, which is there for taking out ranges.
-   Example
    - The 7 is the good and the 6 the value and both result in the good: 7-6+1 would then have to be the antagonists of it (This is not only 5 and 8, but also 6 and 7 again stupidly). Then 7-6 must be subtracted and the multiples are also searched for, so "m7-6+1,m-6-7" are all antagonists of "Wohl".
-   Command "R" or "range" makes setting your numbers for ranges instead of lines.

### the commands 15_.... and PROP..... {#die-befehle-15-dot-dot-dot-dot}

-   The commands beginning with 15\_ together form a tree structure and are output commands of the basic structures and mind (15), as "u" or "i" are output commands. A line specification is needed. Then something can be output.
-   The commands that start with EIG ... act together with a line specification concern the properties of star polygons and uniform polygons.
-   "invert" choses neighbor lines

### other commands {#other-commands}

-   "q" or ":q" or "exit" or "quit" or "end" terminates ReTaPrompt.
-   "shell", with statements after it, executes ordinary shell commands.
-   "python", with statements after it, executes ordinary Python commands.
-   "math", with instructions behind it, executes ordinary Python arithmetic commands, such as 1+1 or 2\*\*3.
-   "logging_yes" turns on logging of the ReTaPrompt command history, and "logging_no" turns it off. If logging is turned on, then you can fetch commands from the past, edit them if necessary, and then execute them.
-   "d" or "divider" means that for output commands not only the corresponding line should be output, but also the lines of the divider of the lines of the line specification. For example with the line specification 12, also the lines 2,3,4,6,12. Line 1 is then never output.
    If "m" is added as a command, then the multiples of all these divisors are also output as lines. This then becomes a lot.


### memory commands {#memory-commands}

-   Store
    - "S" or "CommandSaveAfter" saves the command entered next. "S" executed another time adds more commands to this save.
    - "s" or "CommandSaveBefore" saves the command entered before. So far only the last command can be saved as one thing.
        Command "s" or "S", executed several times, add command entries.
    - If these 2 or 4 save commands ("s" and "S") are entered together with other commands, the other commands are saved immediately without having to enter them before or after.
-   Output
    - If one then enters a command or parts of a command in the command input, then the stored command combines with this command input.
    - For example, one has stored the command a without line specification. If you then enter a line specification, e.g. 2, then this is the command "a 2". This way you can enter commands faster.
    Normally, without storing, one can only type a line specification in the command input and thus these are the commands w a t p.
    - "o" or "CommandSaveOutput" outputs the saved command. Simply typing Enter does the same.
    - It is possible to save a complete reta command (instead of a short command), e.g. `reta -columns --light` and then at the next command input you can enter a line, e.g. 4,7-10 and then from the columns over light lines 4,7,8,9,10 are output. So you can type everything faster. In addition, this can be combined with the short commands w and v, so you can type "3,7-10 v w".
        - The other way around is also possible: You save a line specification and after saving you can specify a reta command, without line specification, and automatically the line for the reta command will be output. This way you can specify several reta commands without having to specify the line for them each time.
-   Delete
    - "c" or "CommandClearSavese" opens a selection of what stored commands are to be deleted.
    Number ranges can be specified or the corresponding characters to be deleted.


## start retaPrompt {#retaprompt-start}

-   start retaPrompt with parameter -vi for ViMode (Otherwise Emacs shortcuts apply),
-   exit with q, exit, quit and
-   call help with h or help,
-   rp (instead of starting retaPrompt) is retaPrompt with vi mode, rpl is retaPrompt with vi mode and logging enabled at program start.
-   retaPrompt parameter -log activates logging at program start.
-   "-command" causes that up to the last program parameter retaPrompt command only one RetaPrompt command is executed.
-   "-e" causes all commands to display the 'e' command   respectively 'noOneCharacterLinePlusNoOutputWhichCommandItWas' is used every time - except if the first command was reta, because  this one works differently
