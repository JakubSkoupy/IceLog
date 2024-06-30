Work in progress, not working at all yet.
This is another project for me to learn on, and also to make logging my training easier for me.
I'm running into challenges structuring this project, which is made harder by a big break I had to take because of school. Hopefully I can get a lesson from this.

Rust just because I'm interested in this language, and want to know how strings work here, as they
seem very tricky.

TODOs
```
  Make it work
  Make it save environments with isolated templates
  Interactive mode as well as just piping logs using existing templates

  Expansion of simple expressions: '[expr ]3' => 'expr expr expr' [DONE]
  Tab completion [Done but very scuffed right now, maybe move this to a tab completion lib]
  Adding variables from range: '[expr 10 + (i * 2) i=(0..5, 2)' => 'expr 10 expr 14 expr 18'
  ...
```

The aim is to get a logger that can manage templates, so far looking for features seen here:

```

// Template:            "Edge Pull" \n:"size"mm \ikg x\i \s [\s..]3
// Template:            "Edge Pull" {num}mm {num:weight}kg x{num} {str} [{str}]5
// Template:            "Edge Pull" \nmm {num}_kg x_\i_ \s_ [\s..]*

// Template Display:    [Edge Pull] {num}mm {num}kg {string} {..string} {..string} {..string}
// Template Display:    [Edge Pull] {num}mm {num}kg {string} {..string 5x}
// Template Display:    [Edge Pull] {num}mm {num}kg {string} {..string}... 

// Template query:      [Edge Pull] 10 44 _         
// Template query:      [Edge Pull] 10 x 4x
// Template query:      [Edge Pull] 10 _ _0
// Template query:      [Edge Pull] 10 44 _ 3f|3b
// Template query:      [Edge Pull] 10 44 _ 3f&3b
// Template query:      [Edge Pull] 10 44 _ 3f 3b
// Template query:      [Edge Pull] 10 44 _ *5
// Template query:      [Edge Pull] 10 44+(i*2) _ i=(0..5, 2)

------------------------------------------------------------------------------

[Edge Pull :: 19.03.2024 13:45:12] 10mm 44kg 1x20 3f
[Edge Pull :: 19.03.2024 13:45:13] 10mm 34kg 1x20 3b
[Edge Pull :: 19.03.2024 13:45:52] 10mm 24kg 1x20 mono_mid
SESSION: 17.03.2024 #1 PULL

```

I want to try parsing this both by hand, and then learning BISON / YACC or something else,
learning about parsing and compilers seems like a good time for summer.

```

TEMPLATE GRAMMAR
TOKENS:

    entry: name [expression [multiplier]?]* [timestamp]?
    expression: [text | field]
    timestamp: -t time

    text: [a-zA-Z0-9 ]*
    name: "text"

    field: {num}
           {str}
           {num..}
           {str..}
           {num:text}
           {str:text}
           {num..:text}
           {num..:text}

    sub_expr_multiplier: {1, 2..} + {*}
    sub_expression: [ expression ] empty | [ expression ] sub_expr_multiplier
```
