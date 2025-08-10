Packrat Parsers Can Support Left Recursion - Alessandro Warth et al
rr ::= "1" <rr> / "1"
lr ::= <lr> "1" / "1"