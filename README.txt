Rune
====
Another silly little interpreter!
Tree-walking with no memory management until
a minimal set of features is complete. 

Roadmap
-------
x Env module with lazy static global scope.
x Add tuples.
x Add pattern matching.
x Add interning of identifiers.
  x Remove all the (currently necessary) copying of strings.
- Add a parser so the language can be used.
- Add pattern matching funciton definitions
  by transforming the AST.
  - This might be catastophic with regards to error reporting.
     Might be better to modify the interpretation of functions
     to always be pattern matching.
- Add a module system.
- Add more built-in types.
  - strings
  - numbers
  - collections

Code Quality
------------
- Make function application consist of function and args
  instead of a list.
- Implement useful traits (such as Display)

Medium-long term
----------------
- Small standard library.
- Add a good REPL.

Far down the line
-----------------
- Bytecode VM
- Memory management
- EZ PZ Rust integration
