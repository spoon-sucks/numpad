# numpad
## a very practical language
numpad is an interpreted esoteric programming language developed in rust only using the numbers 0-9. the numpad flow works by manipulating a single stack of 32 bit integars using various instructions.

## guide
whilst there are a couple of examples in the repo, they don't actually explain anything, so i'm going to explain what all the numbers do.
| key | name | information |
| --- | --- | --- |
| 0 | push | add the number 1 to the top of the stack. |
| 1 | print | print out the number at the top of the stack. |
| 2 | pop | pops out the number at the top of the stack. |
| 3 | swap | swaps the two numbers at the top of the stack. |
| 4 | add | adds the two numbers at the top of the stack. |
| 5 | subtract | subtracts the two numbers at the top of the stack. (if the stack is: [5, 2], it will add 5-2 which is 3) |
| 6 | empty | checks if the stack is empty and pushes the result to the top. (1 if the stack is empty, 0 if not) |
| 7 | compare | checks if the two numbers at the top of the stack are equal and pushes the result to the top. (1 if they are equal, 0 if not) |
| 8 | if-goto | jumps to the most recent label if the top number is not 0, and then pops the top number. |
| 9 | label | acts as a marker for if-goto (does nothing on it's own) |
