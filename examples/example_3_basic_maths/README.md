# Example Grammar 
https://www.r-5.org/files/books/computers/compilers/writing/Keith_Cooper_Linda_Torczon-Engineering_a_Compiler-EN.pdf

We test an adapted grammar from the link(pg.94) in this example.
```
 Goal   =   <Expr>
 Expr   =   <Expr> + Term
            | <Expr> - Term
            | Term
 Term   =   Term * Factor
            | Term ÷ Factor
            | Factor
 Factor =   ( <Expr> )
            | num
            | name
```

cargo run -- -s ./example_3_basic_maths/example_3_basic_maths.dsl -t ./ -n example_3_basic_maths_parser



## Please note the parser is not automatically regenerated. This still needs to be done with build script ideally.