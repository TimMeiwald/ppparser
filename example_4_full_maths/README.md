# Example Grammar 
https://www.r-5.org/files/books/computers/compilers/writing/Keith_Cooper_Linda_Torczon-Engineering_a_Compiler-EN.pdf

https://medium.com/@mbednarski/operator-priority-and-associativity-in-ebnf-grammar-3a9f23dd9daf

Extends example 3 to do maths including exponents etc. Integer maths only though because supporting floats etc isn't interesting as a test of parsing. 
cargo run -- -s ./example_4_full_maths/example_4_full_maths.dsl -t ./ -n example_4_full_maths_parser



## Please note the parser is not automatically regenerated. This still needs to be done with build script ideally.