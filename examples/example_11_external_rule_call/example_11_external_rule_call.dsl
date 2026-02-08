<Alphabet_Lower> Inline =['a'..'z'];
<unique_line_matcher> = unique_line_cache((<Alphabet_Lower>/' ')+, '\n'); # Trivial context sensitive grammar, any lower case line will be matched but only once. #
<Grammar> = (<unique_line_matcher>)+;
