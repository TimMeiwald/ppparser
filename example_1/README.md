# Example Grammar 
https://medium.com/@eichenroth/the-importance-of-left-recursion-in-grammars-608f849447f6

We test the following grammar from the link in this example.
```
expr    := expr "+" integer |
           expr "-" integer |
           integer
integer := [1-9] [0-9]*
```

cargo run -- -s ./example_1/example_1.dsl -t example_1_parser -n example_1_parser


## Please note the parser is not automatically regenerated. This still needs to be done with build script ideally.