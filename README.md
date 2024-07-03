# Torcol

![torcol logo](assets/torcolcircle.png)

Torcol is an esoteric, interpreted programming language (based on the Ladino language spoken in northern Italy) that can be described as something in the middle of a shell language and lisp. Torcol comes from "Torcol dal vent" which means Helicopter.
It is implemented in rust and the parser is fully custom and uses no libraries (the only dependency is anyhow)

# Features

- Memory danger
- Definitely better than java
- (Maybe) Turing complete
- Ladino
- No null type
- Type inference when declaring variables
- EVERYTHING IS AN EXPRESSION.

# Syntax

```bash
# define a variable
lasa x 2 # lasa means let in ladino

# print to screen
stampa x
stampa (jonta 2 4) # commands in parentheses are evaluated and their return values are inserted
stampa (jonta (jonta 5 8) (sotra 4 2)) # prints (5+8) + (4-2)

# conditions
se (eq x 2) {
	stampa "x == 2" 
}
se (no (eq x 2)) {
	stampa "x != 2"
}

cmd miocomando {
	lasa y 3 		   # any variables defined in the block are freed automatically after reaching the closing brace
	stampa (jonta x y) # you can still use variables from outside
}

```

# Additional info

Special thanks to @Ander69420 for translations
