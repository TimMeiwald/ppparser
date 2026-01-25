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
<Ordered_Choice> = <Atom>, <Whitespace>,<Backslash>, <Whitespace>,<Atom>, (<Whitespace>, <Backslash>, <Whitespace>, <Atom>)*;
<One_Or_More> = <Nucleus>, <Whitespace>, <Plus>;
<Zero_Or_More> = <Nucleus>, <Whitespace>, <Star>;
<Optional> = <Nucleus>, <Whitespace>, <Question_Mark>;

<Whitespace> Inline = (' '/'\n'/'\r'/'\t')*;
<RHS> = <Whitespace>, (<Sequence>/<Ordered_Choice>/<Atom>);
<LHS> = <Var_Name_Decl>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;
<Grammar> = <Rule>+, <Whitespace>?;
#Lemme just check comments work# #Double up dem comments#
<Comment> = <Whitespace>, '#', (!'#',<ASCII>)*, '#', <Whitespace>;
<Semantic_Instructions> = <Inline>;
<Inline> = "Inline"; #Comment#