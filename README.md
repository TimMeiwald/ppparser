# PackratParserGenerator 

## Introduction 
PackratParserGenerator is a packrat parsing expression grammar(PEG) parser generator.   

That is, this library can be used to read a PEG grammar and then generate a parser that conforms to that specification which is then able to return an Abstract Syntax Tree(AST) of your language when parsing your language as defined by the PEG grammar file.

Notably the parser generator generates it's own parser from a PEG grammar file describing it's own Domain specific language(DSL) used to describe PEG grammars. That is, the parser generator can generate it's own parser given the right grammar.

In addition, the parser generator has the ability to simply implement a second pass of the AST that can drastically reduce the complexity of the AST or allow for rules that are merely syntactic sugar to make rules easier to read. 

In order to use this library there are three components. How to write your grammar. How to compile your grammar and how to use your generated parser. 

Since this has infinite lookahead and backtracking capabilities it's capable of parsing recursive structures. This means that it can also be used if you simply need a more powerful Regex(It will be less performant though). 

## <u>How to write a grammar file</u>
Every grammar file is generated from rules described with the basic PEG rules. Used with the following syntax. This syntax differs slightly from that described in https://en.wikipedia.org/wiki/Parsing_expression_grammar because it made parsing easier initially. This may be subject to change in a major revision.

| Rule name      | Symbol        | Description                                                                                                                                                                                                                                                                               | Example                                                                                                                                                                                                                                             |
| -------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Sequence       | ,             | The expression to the left of the comma and to the right of the comma must both match or the sequence fails. If it matches it consumes those tokens                                                                                                                                       | Expression: "A", "B" matches the string "AB" and nothing else.                                                                                                                                                                                      |
| Ordered Choice | /             | The expression to the left of the forward slash must match <u>or</u> the expression to the right must succeed or else the ordered choice fails. The first expression always matches preferentially. If it matches it consumes those tokens                                                | Expression: "A"/"B" matches the string "A" or the string "B".                                                                                                                                                                                       |
| And Predicate  | &             | The and predicate is applied before the expression and returns True if it matches the expression and False if it does not. It does not however ever consume tokens.                                                                                                                       | Expression: &"A" will return True if the argument is "A" but False otherwise                                                                                                                                                                        |
| Not Predicate  | !             | The not predicate is also applied before the expression and acts exactly like the negated value of the And Predicate. Where the And Predicate returns True if it matches the expression the Not predicate returns False and where And Predicate returns False Not predicate returns True. | Expression: !"A" will return True if the argument is <u>not</u> "A" and false otherwise                                                                                                                                                             |
| One or More    | +             | The one or more operator is applied after the expression and must match once in order to return True and consumes tokens. It may however consume multiple of the expression.                                                                                                              | Expression: "A"+ will return True and consume tokens if the argument is "A" or "AA" or "AAA" or any number of "A". Otherwise it will return False                                                                                                   |
| Zero or More   | *             | The zero or more operator is applied after the expression and must match zero times in order to return True but must match to consume tokens. It can also consume multiple expressions.                                                                                                   | Expression: "A"* will return True if passed <u>not</u> "A" and it will return True if passed "A" or "AA" or any number of "A"s. It will however only consume as many tokens as it has successfully matched                                          |
| Terminal       | "e"           | A terminal is any single character(or no character) between a pair of quotation marks("). Currently only ASCII set in uint_8 tested. A terminal will match if it's content is the argument.                                                                                               | Terminal: "A" will match if the argument is "A" and return True as well as consume the token, otherwise it returns False.                                                                                                                           |
| Rule           | \<Rule_Name\> | A Rule is simply a construct of the grammar. It is composed of the form A <- e where A is a \<Rule_Name\> and e is some valid parsing expression                                                                                                                                          | Rule: \<One\> = "1" will match the string "1" and return True otherwise it will return False. You can reference a rule from another rule allowing you to make reuseable grammar components that can be put together to construct the whole grammar. |


And the extension
| Extension name | Symbol | Description                                                                                                                                                                                                                                                                                                                                                                    | Example                                                    |
| -------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| Subexpression  | ()     | The subexpression essentially acts as an implicit Rule and allows you to define order of operations in a Rule because otherwise the limitations of the parser generator becomes tedious. The primary reason for this is because order of operations for PEG operations is not something I could find a exact example of and because brackets make things easier to read anyway | Expression: ("A", "B")/("C", "D") will match "AB" or "CD". |

Whilst these rules do not seem particularly powerful they allow you to construct sophisticated and complex grammars and compile a parser from it in a very short period of time. Unfortunately it's somewhat difficult to add meaningful error messages in due to it's backtracking nature(suggestions for solutions are certainly welcome). 

### <u>More complex examples</u>

The following rules matches all capital letters.
```
<Alphabet_Upper> ="A"/"B"/"C"/"D"/"E"/"F"/"G"/"H"/"I"/"J"/"K"/"L"/"M"/"N"/"O"/"P"/"Q"/"R"/"S"/"T"/"U"/"V"/"W"/"X"/"Y"/"Z";   
```
The following matches all lowercase letters  
```
<Alphabet_Lower>  ="a"/"b"/"c"/"d"/"e"/"f"/"g"/"h"/"i"/"j"/"k"/"l"/"m"/"n"/"o"/"p"/"q"/"r"/"s"/"t"/"u"/"v"/"w"/"x"/"y"/"z";
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
<Num> = "0"/"1"/"2"/"3"/"4"/"5"/"6"/"7"/"8"/"9";
```
This matches any valid python variable name. It first matches one letter or underscore and then matches as many zero or more underscores, letters or numbers. Since python restricts digits to not being the first character of the variable name(probably to ease the interpreters lexing.)  
```
<Variable> = (<Alphabet_Lower>/<Alphabet_Upper>/"_"), 
            (<Alphabet_Lower>/<Alphabet_Upper>/"_"/<Num>)*;
```
As one can imagine because it's possible to combine rules together one can make very complex parsers. 

An example is the following. The following rules are triggered using the Grammar rule. It is the grammar for the DSL described in this section of the text.
<font size = "1">

```
<Alphabet_Upper > PASSTHROUGH ="A"/"B"/"C"/"D"/"E"/"F"/"G"/"H"/"I"/"J"/"K"/"L"/"M"/"N"/"O"/"P"/"Q"/"R"/"S"/"T"/"U"/"V"/"W"/"X"/"Y"/"Z"; #We all love commments#
<Alphabet_Lower> PASSTHROUGH ="a"/"b"/"c"/"d"/"e"/"f"/"g"/"h"/"i"/"j"/"k"/"l"/"m"/"n"/"o"/"p"/"q"/"r"/"s"/"t"/"u"/"v"/"w"/"x"/"y"/"z";
<Num> PASSTHROUGH = "0"/"1"/"2"/"3"/"4"/"5"/"6"/"7"/"8"/"9";
<Spaces> PASSTHROUGH = "\n"/"\t"/"\r"/" ";
<Specials> PASSTHROUGH = "+"/"*"/"-"/"&"/"!"/"?"/"<"/">"/"""/"("/")"/"_"/","/"/"/";"/"="/"\"/"#"/":"/"|"/"."/"{"/"}"/"["/"]"/"%"/"'";
<ASCII> PASSTHROUGH = <Alphabet_Lower>/<Alphabet_Upper>/<Num>/<Spaces>/<Specials>;
<Apostrophe> DELETE = """;
<Left_Angle_Bracket> DELETE = "<";
<Right_Angle_Bracket> DELETE = ">";
<Left_Bracket> DELETE = "(";
<Right_Bracket> DELETE = ")";
<Assignment> DELETE = "=";
<End_Rule> DELETE = ";";
<Ampersand> DELETE = "&";
<Exclamation_Mark> DELETE = "!";
<Plus> DELETE = "+";
<Star> DELETE = "*";
<Question_Mark> DELETE = "?";
<Comma> DELETE = ",";
<Backslash> DELETE = "/";

<Var_Name> COLLECT =<Left_Angle_Bracket>,
                    (<Alphabet_Lower>/<Alphabet_Upper>),
                    (<Alphabet_Lower>/<Alphabet_Upper>/"_")*,
                    <Right_Angle_Bracket>; 
                    #Not whitespace dependent, feel free to use multiple lines for readability#

<Subexpression> = <Left_Bracket>,<RHS>,<Right_Bracket>;
<Epsilon> = <Apostrophe>, <Apostrophe>;
<Terminal> = (<Apostrophe>,<ASCII>,<Apostrophe>)/(<Apostrophe>,"\",("n"/"r"/"t"),<Apostrophe>)/<Epsilon>;
<Nucleus> PASSTHROUGH = (<Subexpression>/<Terminal>/<Var_Name>), <Whitespace>;
<Atom> PASSTHROUGH = (<And_Predicate>/<Not_Predicate>/<One_Or_More>/<Zero_Or_More>/<Optional>/<Nucleus>), <Whitespace>;

<And_Predicate> = <Ampersand>, <Nucleus>;
<Not_Predicate> = <Exclamation_Mark>, <Nucleus>;
<Sequence> = <Atom>, <Whitespace>, <Comma>, <Whitespace>, <Atom>, (<Comma>, <Whitespace>, <Atom>)*;
<Ordered_Choice> = <Atom>, <Whitespace>,<Backslash>, <Whitespace>,<Atom>, (<Backslash>, <Whitespace>, <Atom>)*;
<One_Or_More> = <Nucleus>, <Whitespace>, <Plus>;
<Zero_Or_More> = <Nucleus>, <Whitespace>, <Star>;
<Optional> = <Nucleus>, <Whitespace>, <Question_Mark>;

<Whitespace> DELETE = (" "/"\n")*;
<RHS> PASSTHROUGH = <Sequence>/<Ordered_Choice>/<Atom>;
<LHS> = <Var_Name>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;
<Grammar> = <Rule>+, <Whitespace>;
#Lemme just check comments work# #Double up dem comments#
<Comment> COLLECT = <Whitespace>, "#", (!"#",<ASCII>)*, "#", <Whitespace>;
<Semantic_Instructions> PASSTHROUGH = <Delete>/<Passthrough>/<Collect>;
<Delete> COLLECT = "D","E","L","E","T","E";
<Passthrough> COLLECT = "P", "A", "S", "S", "T", "H", "R", "O", "U", "G", "H";
<Collect> COLLECT = "C", "O", "L", "L", "E", "C", "T"; #Comment#
```
</font>

You may have taken a look at the above grammar file and wondered what COLLECT, PASSTHROUGH and DELETE mean. Good question. 

These are used to generate the 2nd pass of the parser that is used to simplify the generated AST when parsing your grammar. 

If you tag a rule with DELETE then all of the nodes that matched that rule will be deleted including their children. This is particularly useful for getting rid of useless tokens that still need to be matched for the parser to work like whitespace(in non-whitespace indented languages).

If you tag a rule with PASSTHROUGH then the tagged node will be collapsed with all of it's children being appended to it's parent node. This allows you to use syntactic sugar rules like \<Alphabet_Upper\> without cluttering the AST that is outputted with lots of nodes that are useless since capitalized letters aren't meaningful language constructs as such. They just save typing. 

If you tag a rule with COLLECT then the tagged node will collect the contents of all of the _TERMINAL nodes and cocatenate the result and assign it to the tagged nodes content. This <u>only</u> works for nodes that have only _TERMINAL nodes. These are generated by anything matching a raw terminal like "A". Note: Since \<Alphabet_Upper\> uses passthrough you can use this in other rules and still collect the _TERMINAL nodes generated by it because of the order in which these extra "semantic instructions" are called. 

<u>Important Note</u>   
The 2nd pass is not clever. It simply does DELETE on all  DELETE tagged nodes first. Then does PASSTHROUGH on all PASSTHROUGH tagged nodes. Then COLLECT on all COLLECT tagged nodes. It's not possible to conditionally call them. At that point it becomes language specific enough that it should be handled by you. This is merely some convenience for simple things like removing all whitespace.   

However, If you wish some whitespace to be meaningful and other whitespace to be meaningless. Then you absolutely can defin two rules as follows   
<meaningful_space> = " ";  
<meaingless_space> DELETE = " ";  
And that way you can simply choose whichever rule you want and conditionally choose which spaces to delete since the DELETE is by node type which in this case is the rule not the fact that they represent a space. 

## <u>How to compile your grammar</u>
## <u>How to use your parser</u>






## <u>Known Defect List</u>
DEFECT: Known issue between string inputs and string from file read. Unsure how to solve or what even the issue is.     
## <u>To do List</u>
TODO: Add DROPTERM to semantic instructions so that you can keep a node but remove it's terminals e.g where the nodes existence matters but not the characters themselves.  
TODO: QOL Feature Use uint[0-255] to get all ASCII chars as opposed to current solution              
TODO: Implement Pylint/MyPy/Black         
TODO: Error handling, halt start at top level rule and step in find longest match and then return associated error. Generate error framework       
TODO: Add Commenter Tests    
TODO: Add Parse_Call_Maker Tests     
TODO: Have tests rerun with new parser too so that I can CI/CD any updates to the parser too.     
TODO: Also implement automated tests for the entire compile process.    
TODO: Basically more Tests.    
  

PERF=/usr/lib/linux-tools/6.8.0-39-generic/perf cargo flamegraph --root -- -s ../json_samples/canada.json