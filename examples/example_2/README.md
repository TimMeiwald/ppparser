# Example Grammar 
https://medium.com/@eichenroth/the-importance-of-left-recursion-in-grammars-608f849447f6

We test the following grammar from the link in this example.
```
A := B "a" | "a"
B := A "b" | "b"
```

This grammar can parse any string of alternating “a”s and “b”s that ends with an “a”. Indirect left-recursion is a little bit harder to detect, and most parsers cannot handle it.

cargo run -- -s ./example_2/example_2.dsl -t ./ -n example_2_parser


## Please note the parser is not automatically regenerated. This still needs to be done with build script ideally.