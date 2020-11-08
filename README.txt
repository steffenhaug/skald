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
- Make functions use pattern matching to bind arguments.
- Add interning of identifiers.
  - Remove all the (currently necessary) copying of strings.
- Add pattern matching funciton definitions
  by transforming the AST.
- Add a module system.
- Add a parser so the language can be used.

Medium-long term
----------------
- Small standard library.
- Add a good REPL.

Far down the line
-----------------
- Bytecode VM
- Memory management
- EZ PZ Rust integration
