# PackratParserGenerator 
            
## Introduction 
PackratParserGenerator is a packrat parsing expression grammar(PEG) parser generator.   

That is, this library can be used to read a PEG grammar and then generate a parser that conforms to that specification which is then able to return an Abstract Syntax Tree(AST) of your language when parsing your language as defined by the PEG grammar file.

Notably the parser generator generates it's own parser from a PEG grammar file describing it's own Domain specific language(DSL) used to describe PEG grammars. That is, the parser generator can generate it's own parser given the right grammar.

In order to use this library there are three components. How to write your grammar. How to compile your grammar and how to use your generated parser. 

Since this has infinite lookahead and backtracking capabilities it's capable of parsing recursive structures. This means that it can also be used if you simply need a more powerful Regex(It will likely be less performant though). 

## <u>How to write a grammar file</u>
Every grammar file is generated from rules described with the basic PEG rules. Used with the following syntax. This syntax differs slightly from that described in https://en.wikipedia.org/wiki/Parsing_expression_grammar because it made parsing easier initially. This may be subject to change in a major revision.

| Rule name      | Symbol        | Description                                                                                                                                                                                                                                                                               | Example                                                                                                                                                                                                                                             |
| -------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Terminal       | 'e'           | A terminal is any single character(or no character) between a pair apostrophes('). Currently only ASCII set in uint_8 tested. A terminal will match if it's content is the argument.                                                                                               | Terminal: 'A' will match if the argument is 'A' and return True as well as consume the token, otherwise it returns False. 
| Terminal Array       | ['A'..'Z']           |Will match any upper case character from A to Z. This only currently works for ASCII and will match any character who's numerical value is between the left and right hand side. You can also use hex if preferred e,g '[0x01..0x20]'| ['A'..'Z'] will match any uppercase character. 
| String Terminal       | "Hello World!"           | A String terminal is any characters between a pair of quotation marjs("). Currently only ASCII set in uint_8 tested. A terminal will match if it's content is the argument.                                                                                               | String Terminal: "Hello World!" will match if the argument is "Hello World!" and return True as well as consume the token, otherwise it returns False.           
| Sequence       | ,             | The expression to the left of the comma and to the right of the comma must both match or the sequence fails. If it matches it consumes those tokens                                                                                                                                       | Expression: 'A', 'B' matches the string 'AB' and nothing else.                                                                                                                                                                                      |
| Ordered Choice | /             | The expression to the left of the forward slash must match <u>or</u> the expression to the right must succeed or else the ordered choice fails. The first expression always matches preferentially. If it matches it consumes those tokens                                                | Expression: 'A'/'B' matches the string 'A' or the string 'B'.                                                                                                                                                                                       |
| And Predicate  | &             | The and predicate is applied before the expression and returns True if it matches the expression and False if it does not. It does not however ever consume tokens.                                                                                                                       | Expression: &'A' will return True if the argument is 'A' but False otherwise                                                                                                                                                                        |
| Not Predicate  | !             | The not predicate is also applied before the expression and acts exactly like the negated value of the And Predicate. Where the And Predicate returns True if it matches the expression the Not predicate returns False and where And Predicate returns False Not predicate returns True. | Expression: !'A' will return True if the argument is <u>not</u> 'A' and false otherwise                                                                                                                                                             |
| One or More    | +             | The one or more operator is applied after the expression and must match once in order to return True and consumes tokens. It may however consume multiple of the expression.                                                                                                              | Expression: 'A'+ will return True and consume tokens if the argument is 'A' or 'AA' or 'AAA' or any number of 'A'. Otherwise it will return False                                                                                                   |
| Zero or More   | *             | The zero or more operator is applied after the expression and must match zero times in order to return True but must match to consume tokens. It can also consume multiple expressions.                                                                                                   | Expression: 'A'* will return True if passed <u>not</u> 'A' and it will return True if passed 'A' or 'AA' or any number of 'A's. It will however only consume as many tokens as it has successfully matched                                          |                                                                                                                     |
| Rule           | \<Rule_Name\> | A Rule is simply a construct of the grammar. It is composed of the form A <- e where A is a \<Rule_Name\> and e is some valid parsing expression                                                                                                                                          | Rule: \<One\> = '1' will match the string '1' and return True otherwise it will return False. You can reference a rule from another rule allowing you to make reuseable grammar components that can be put together to construct the whole grammar. |


And the extension
| Extension name | Symbol | Description                                                                                                                                                                                                                                                                                                                                                                    | Example                                                    |
| -------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| Subexpression  | ()     | The subexpression essentially acts as an implicit Rule and allows you to define order of operations in a Rule because otherwise the limitations of the parser generator becomes tedious. The primary reason for this is because order of operations for PEG operations is not something I could find a exact example of and because brackets make things easier to read anyway | Expression: ('A', 'B')/('C', 'D') will match 'AB' or 'CD'. |
| Inline  | Inline     | The inline instruction can be added to a rule so that it does not generate nodes(in fact it does not cache either so left recursive rules will not work. It effectively turns the rule into a direct function call that has no impact on the cache. This is useful to prevent e.g whitespace from adding to the cache size despite being instantly discarded). It can also be used where you want to seperate a complex rule into named subcomponents but not have them actually match individually and therefore use more memory during parsing. | \<ExampleRule\> Inline = ' '; Will match a whitespace and increment but will not create a ExampleRule node type. |

Whilst these rules do not seem particularly powerful they allow you to construct sophisticated and complex grammars and compile a parser from it in a very short period of time. Unfortunately it's somewhat difficult to add meaningful error messages in due to it's backtracking nature(suggestions for solutions are certainly welcome). 

### <u>More complex examples</u>

The following rules matches all capital letters.
```
<Alphabet_Upper> ='A'/'B'/'C'/'D'/'E'/'F'/'G'/'H'/'I'/'J'/'K'/'L'/'M'/'N'/'O'/'P'/'Q'/'R'/'S'/'T'/'U'/'V'/'W'/'X'/'Y'/'Z';   
```
However so does this.
```
<Alphabet_Upper> = ['A'..'Z'];
```
The following matches all lowercase letters  
```
<Alphabet_Lower>  = ['a'..'z']
```
The following rule matches all lowercase and uppercase letters  
```
<Alphabet> = <Alphabet_Upper>/<Alphabet_Lower>;  
```
You can even add comments to your grammar.   
The comments will automatically be added to the docstring of the previous rule. So if you append a single comment to the end of each rule that comment will be in it's docstring in the generated parser.
```     
<Alphabet> = <Alphabet_Upper>/<Alphabet_Lower>; #This matches both upper and lower case letters, Comments are contained by one hash on either side. #
```

This matches any digit    
```
<Num> = ['0'..'9'];
```
This matches any valid python variable name. It first matches one letter or underscore and then matches as many zero or more underscores, letters or numbers. Since python restricts digits to not being the first character of the variable name(probably to ease the interpreters lexing.)  
```
<Variable> = (<Alphabet_Lower>/<Alphabet_Upper>/'_'), (<Alphabet_Lower>/<Alphabet_Upper>/'_'/<Num>)*;
```
As one can imagine because it's possible to combine rules together one can make very complex parsers. 

An example is the following. The following rules are triggered using the Grammar rule. It is the grammar for the DSL described in this section of the text.
<font size = '1'>

```
<Alphabet_Upper> Inline = ['A'..'Z']; #We all love commments#
<Alphabet_Lower> Inline =['a'..'z'];
<Num> = [0x30..0x39];
<NumNoZero> = [0x31..0x39];
<HexVal>= [0x30..0x39]/[0x41..0x46];
<Spaces> Inline = '\n'/'\t'/'\r'/' ';
<Specials> Inline = !(<Alphabet_Upper>/<Alphabet_Lower>/<Num>/<Spaces>),[0x0..0xFF] ;
<ASCII> = [0x00..0xFF];
<Apostrophe> Inline = '"';
<QuotationMark> Inline = ''';
<Left_Angle_Bracket> Inline = '<';
<Right_Angle_Bracket> Inline = '>';
<Left_Bracket> Inline = '(';
<Right_Bracket> Inline = ')';
<Assignment> Inline = '=';
<End_Rule> Inline = ';';
<Ampersand> Inline = '&';
<Exclamation_Mark> Inline = '!';
<Plus> Inline = '+';
<Star> Inline = '*';
<Question_Mark> Inline = '?';
<Comma> Inline = ',';
<Newline> = '\n';
<Backslash> Inline = '/';

<Var_Name> Inline = (<Alphabet_Lower>/<Alphabet_Upper>),(<Alphabet_Lower>/<Alphabet_Upper>/'_')*;
<Var_Name_Decl> = <Left_Angle_Bracket>, <Var_Name>, <Right_Angle_Bracket>;
<Var_Name_Ref>= <Left_Angle_Bracket>, <Var_Name>, <Right_Angle_Bracket>;
<Hex> = '0', 'x', <HexVal>+; #Replace with custom code#
<Integer> = <NumNoZero>, <Num>*; #No negative values since that is meaningless in this context#
<OrderedChoiceMatchRange> = <Whitespace>, '[', <Whitespace>,(<Terminal>/<Integer>/<Hex>),'.', '.',(<Terminal>/<Integer>/<Hex>) ,<Whitespace>, ']', <Whitespace>;


<Subexpression> = <Left_Bracket>,<RHS>,<Right_Bracket>;
<Epsilon> = <QuotationMark>, <QuotationMark>;
<StringTerminal> = (<Apostrophe>, !<Apostrophe>, (<ASCII>, (!<Apostrophe>,<ASCII>)+), <Apostrophe>)/<Hex>/<Integer>; #Multibyte matches essentially#

<Terminal> = (<QuotationMark>,<ASCII>,<QuotationMark>)/(<QuotationMark>,'\',('n'/'r'/'t'),<QuotationMark>)/<Epsilon>;
<Nucleus> = (<OrderedChoiceMatchRange>/<Subexpression>/<Terminal>/<StringTerminal>/<Var_Name_Ref>), <Whitespace>;
<Atom> = (<And_Predicate>/<Not_Predicate>/<One_Or_More>/<Zero_Or_More>/<Optional>/<Nucleus>), <Whitespace>;

<And_Predicate> = <Ampersand>, <Nucleus>;
<Not_Predicate> = <Exclamation_Mark>, <Nucleus>;
<Sequence> = <Atom>, <Whitespace>, <Comma>, <Whitespace>, <Atom>, (<Comma>, <Whitespace>, <Atom>)*;
<Ordered_Choice> = <Atom>, <Whitespace>,<Backslash>, <Whitespace>,<Atom>, (<Backslash>, <Whitespace>, <Atom>)*;
<One_Or_More> = <Nucleus>, <Whitespace>, <Plus>;
<Zero_Or_More> = <Nucleus>, <Whitespace>, <Star>;
<Optional> = <Nucleus>, <Whitespace>, <Question_Mark>;

<Whitespace> Inline = (' '/'\n'/'\r'/'\t')*;
<RHS> = <Sequence>/<Ordered_Choice>/<Atom>;
<LHS> = <Var_Name_Decl>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;
<Grammar> = <Rule>+, <Whitespace>?;
#Lemme just check comments work# #Double up dem comments#
<Comment> = <Whitespace>, '#', (!'#',<ASCII>)*, '#', <Whitespace>;
<Semantic_Instructions> = <Inline>;
<Inline> = "Inline"; #Comment#
```
</font>

## <u>How to compile your grammar</u>
## <u>How to use your parser</u> 