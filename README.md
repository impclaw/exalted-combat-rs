# Exalted Combat Tracker
This is a tool for tracking Exalted 3rd edition combat. You need some familiarity with reading rust code to understand how to use is. The tracker uses a keyboard-controlled ncurses-based TUI to enable fast usage during combat. After some practice, using the tracker becomes second nature, and does not detract from the tabletop experience. There are still some usability issues, and I'm happy to accept any improvements. 

## Data Files
The combat tracker uses two data files, `chars.json` and `monsters.json`. The format of these files is described with the Character struct. The `chars.json` file contains startup characters, and should be filled with your party. The `monsters.json` file contains the monster database, which can be used to quickly import monsters to the combat encounter. The monsters database is not included with the program for copyright reasons. Also, mine contains a ton of my own custom monsters. 

## Usage
The program starts combat by performing a Join Battle action for all the participants. After that, you control combat by selecting a character using J/K, and performing one of these actions. 

| Key | Action |
| ------------- | ------------- |
| D | Toggles character ready/done |
| i | Modify character initiative |
| o | Modify character onslaught |
| h | Modify character health |
| n | New round |
| a | Add new character |
| m | Add monster from database |
| d | Perform decisive attack |
| w | Perform withering attack |
| r | Remove character |
| x | Reset combat |
| q | Quit program |
| ESC | Cancel action |

## Bugs
If you find any bugs, and I'm sure there are many (my understanding of the game rules is not that great), please report them on the issue tracker. Not that I expect anyone else to actually use this tool :)
