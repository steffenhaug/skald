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
- Add interning of identifiers.
  - Remove all the (currently necessary) copying of strings.
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

Medium-long term
----------------
- Small standard library.
- Add a good REPL.

Far down the line
-----------------
- Bytecode VM
- Memory management
- EZ PZ Rust integration



Note on syntax
==============
Syntax is very much not finalized.

# Pattern matching on constants and variables
(fun (fib 0) 1
     (fib 1) 1
     (fib n) (+ (fib (- n 1))
                (fib (- n 2))))

# ML-style (ish)
fib(0) = 1
fib(1) = 1
fib(n) = fib(n - 1) + fib (n - 2)

# Proper ML style
fib 0 = 1
fib 1 = 1
fib n = (fib n - 1) + (fib n - 2)

# Pattern matching on tuples.
# colon prefix = atom

# Explicit match
(fun (tree-print T)
       (match T
         {:leaf X} (print X)
         {:branch L R}
           (do (tree-print L)
               (tree-print R))))

# Match baked into signature similar to ML languages.
(fun (tree-print {:leaf x})
       (print X)

     (tree-print {:branch l r})
       (do
         (tree-print l)
         (tree-print r)))

# ML-style.
fun tree_print({:leaf x}) =
  print(x)

fun tree_print({:branch l r}) = do
  tree_print(l)
  tree_print(r)
end
