<Alphabet_Upper> PASSTHROUGH = ['A'..'Z']; #We all love commments#
<Alphabet_Lower> PASSTHROUGH =['a'..'z'];
<Num> PASSTHROUGH = [0x30..0x39];
<NumNoZero> = [0x31..0x39];
<HexVal>= [0x30..0x39]/[0x41..0x46];
<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';
<Specials> PASSTHROUGH = !(<Alphabet_Upper>/<Alphabet_Lower>/<Num>/<Spaces>),[0x0..0xFF] ;
<ASCII> PASSTHROUGH = [0x00..0xFF];
<Apostrophe> DELETE = '"';
<QuotationMark> = ''';
<Left_Angle_Bracket> DELETE = '<';
<Right_Angle_Bracket> DELETE = '>';
<Left_Bracket> DELETE = '(';
<Right_Bracket> DELETE = ')';
<Assignment> DELETE = '=';
<End_Rule> DELETE = ';';
<Ampersand> DELETE = '&';
<Exclamation_Mark> DELETE = '!';
<Plus> DELETE = '+';
<Star> DELETE = '*';
<Question_Mark> DELETE = '?';
<Comma> DELETE = ',';
<Newline> = '\n';
<Backslash> DELETE = '/';

<Var_Name> =<Left_Angle_Bracket>,
                    (<Alphabet_Lower>/<Alphabet_Upper>),
                    (<Alphabet_Lower>/<Alphabet_Upper>/'_')*,
                    <Right_Angle_Bracket>; 
                    #Not whitespace dependent, feel free to use multiple lines for readability#


<Var_Name_Decl> = <Var_Name>;
<Hex> = '0', 'x', <HexVal>+; #Replace with custom code#
<Integer> = <NumNoZero>, <Num>*; #No negative values since that is meaningless in this context#
<OrderedChoiceMatchRange> = <Whitespace>, '[', <Whitespace>,(<Terminal>/<Integer>/<Hex>),'.', '.',(<Terminal>/<Integer>/<Hex>) ,<Whitespace>, ']', <Whitespace>;


<Subexpression> = <Left_Bracket>,<RHS>,<Right_Bracket>;
<Epsilon> = <QuotationMark>, <QuotationMark>;
<StringTerminal> = (<Apostrophe>, !<Apostrophe>, (<ASCII>, (!<Apostrophe>,<ASCII>)+), <Apostrophe>)/<Hex>/<Integer>; #Multibyte matches essentially#

<Terminal> = (<QuotationMark>,<ASCII>,<QuotationMark>)/(<QuotationMark>,'\',('n'/'r'/'t'),<QuotationMark>)/<Epsilon>;
<Nucleus> PASSTHROUGH = (<OrderedChoiceMatchRange>/<Subexpression>/<Terminal>/<StringTerminal>/<Var_Name>), <Whitespace>;
<Atom> PASSTHROUGH = (<And_Predicate>/<Not_Predicate>/<One_Or_More>/<Zero_Or_More>/<Optional>/<Nucleus>), <Whitespace>;

<And_Predicate> = <Ampersand>, <Nucleus>;
<Not_Predicate> = <Exclamation_Mark>, <Nucleus>;
<Sequence> = <Atom>, <Whitespace>, <Comma>, <Whitespace>, <Atom>, (<Comma>, <Whitespace>, <Atom>)*;
<Ordered_Choice> = <Atom>, <Whitespace>,<Backslash>, <Whitespace>,<Atom>, (<Backslash>, <Whitespace>, <Atom>)*;
<One_Or_More> = <Nucleus>, <Whitespace>, <Plus>;
<Zero_Or_More> = <Nucleus>, <Whitespace>, <Star>;
<Optional> = <Nucleus>, <Whitespace>, <Question_Mark>;

<Whitespace> Inline = (' '/'\n'/'\r'/'\t')*;
<RHS> PASSTHROUGH = <Sequence>/<Ordered_Choice>/<Atom>;
<LHS> = <Var_Name_Decl>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;
<Grammar> = <Rule>+, <Whitespace>?;
#Lemme just check comments work# #Double up dem comments#
<Comment> = <Whitespace>, '#', (!'#',<ASCII>)*, '#', <Whitespace>;
<Semantic_Instructions> PASSTHROUGH = <Delete>/<Passthrough>/<Inline>;
<Delete> = "DELETE";
<Passthrough> = "PASSTHROUGH";
<Inline> = "Inline"; #Comment#

<test_LR_num> = <Num>;
<test_LR_expr> = (<test_LR_expr>, '-', <test_LR_num>) / <test_LR_num>; # Should match 0-0-0-0-0-0-0-0 etc #