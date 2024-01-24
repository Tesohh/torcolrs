# Torcol

![torcol logo](assets/torcolcircle.png)

Torcol is an esoteric, interpreted programming language (based on the Ladino language spoken in northern Italy) that can be described as something in the middle of a shell language and lisp.
It is implemented in rust and the parser is fully custom and uses no libraries (the only dependency is anyhow)

# Features

- Memory danger
- Definitely better than java
- (Maybe) Turing complete
- Ladino
- No null type
- Type inference when declaring variables

# Syntax

```bash
# define a variable
lasa x 2 # lasa means let in ladino

# print to screen
drucher x
drucher (jonta 2 4) # commands in parentheses are evaluated and their return values are inserted
drucher (jonta (jonta 5 8) (sotra 4 2)) # prints (5+8) + (4-2)
```

# Additional info

Special thanks to @Ander69420 for translations
