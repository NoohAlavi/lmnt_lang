# ELEMENT PROGRAMMING LANGUAGE

<h2>What is it?</h2>

Element is just a stupid little programming language i made for fun.

<h2>How Do I Use It?</h2>

If you really want to use it, for whatever reason, just create a file with the extension `.lmnt`, clone the repo, and run `cargo r <pathToLmntFile>`.

<h2>Documentation</h2>

`println <textToPrint>` -> Prints text to the console 

`getln {<variableName>}` -> Gets user input from the console, and stores it in a new given variable

`let {<variableName>} <value>` -> Creates a new variable with given name and gives it a given value

`throw <customErrorMessage>` -> Prints an error to the console and stops the program

<h3>Example Program</h3>

```
// Prints "Hello, World!" and "What is your name?" to the console
println Hello, World!
println What is your name?

// Gets user input and stores it in 'name' variable
getln {name}

// Prints "Nice to meet you" along with the name the user entered
println Nice to meet you, {name} !
```
