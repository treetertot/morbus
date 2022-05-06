# morbus
Morbus means disease or something. I nabbed the name from a meme about a certian movie that will not age well.

Morbus isn't designed to be the worst language ever;
some of the operations aren't necessary and are provided for convenience and a stack is provided for isolating numbers being operated on.
It was instead designed to maximize goto nonsense by making the goto the most prominent thing on each executed line.

Morbus has two sections of memory: program memory and stack.

Program memory is the indexable memory generated from your file and gets executed from the line in line 0.
Every line in program memory contains a number. If it's empty or contains something that is not a number it will be treated as 0.
When a line is entered from another line (so this will not happen to line zero unless you reach it inside your own program),
it will execute the operation numbered the current line % 10. It will then go to the line that is the number on the current line.

The stack is what most of the operations modify.

The operations are:

0 - Push Next

1 - Push Address

2 - Pop Address

3 - Duplicate

4 - Swap

5 - Add

6 - Subtract

7 - Multiply

8 - Divide

9 - Quit


Push Next pushes the number on the next line onto the stack.

Push Address pops a number off the stack, goes to that line, and then pushes the number there onto the stack.

Pop address removes the two numbers on top of the stack, writing the second to the line in the first number.

Duplicate makes a copy of the top stack item and pushes it onto the stack.

Swap flips the top two items on the stack.

Add - Divide each apply the operation on the second item from the top of the stack by the top item on the stack.

Quit ends the program and determines what is in the output.
The program will spit out all the numbers in the program section afer the number on the quit line.
This program just prints the elements as a rust debug statement.
Future versions may allow more options for output. They also might not. Who cares?!

For some examples take a look at the .morb files in this repository.

hello.morb returns with the numbers that would make "Hello, world!" if they were bytes.

average.morb averages three numbers in the file.
