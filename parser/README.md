To regenerate the parser run the following 
cargo run -- -s ./parser/Grammar.dsl -t ./parser2/ -n parser2

Then copy over any changes. We cannot directly generate the parser to itself since it depends on the
text in parser to create the new parser. 