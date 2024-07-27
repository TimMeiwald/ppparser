<Alphabet_Upper> PASSTHROUGH = ['A'..'Z']; #We all love commments#
<Alphabet_Lower> PASSTHROUGH =['a'..'z'];
<Num> PASSTHROUGH = '0'/'1'/'2'/'3'/'4'/'5'/'6'/'7'/'8'/'9';
<Spaces> PASSTHROUGH = '\n'/'\t'/'\r'/' ';
<Specials> PASSTHROUGH = '+'/'*'/'-'/'&'/'!'/'?'/'<'/'>'/'"'/'('/')'/'_'/','/'/'/';'/'='/'\'/'#'/':'/'|'/'.'/'{'/'}'/'['/']'/'%'/'''/'^'/'~'/'@';
<ASCII> PASSTHROUGH = <Alphabet_Lower>/<Alphabet_Upper>/<Num>/<Spaces>/<Specials>;
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

<Var_Name> COLLECT =<Left_Angle_Bracket>,
                    (<Alphabet_Lower>/<Alphabet_Upper>),
                    (<Alphabet_Lower>/<Alphabet_Upper>/'_')*,
                    <Right_Angle_Bracket>; 
                    #Not whitespace dependent, feel free to use multiple lines for readability#


<Var_Name_Decl> = <Var_Name>;
<Hex> = '0', 'x'; #Replace with custom code#
<Integer> = '0'; #Replace with custom code#
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

<Whitespace> DELETE = (' '/<Newline>/'\r'/'\t')*;
<RHS> PASSTHROUGH = <Sequence>/<Ordered_Choice>/<Atom>;
<LHS> = <Var_Name_Decl>, (<Whitespace>, <Semantic_Instructions>, <Whitespace>)?;
<Rule> = <LHS>, <Whitespace>, <Assignment>, <Whitespace>, <RHS>, <Whitespace>, <End_Rule>, <Whitespace>, <Comment>*;
<Grammar> = <Rule>+, <Whitespace>?;
#Lemme just check comments work# #Double up dem comments#
<Comment> COLLECT = <Whitespace>, '#', (!'#',<ASCII>)*, '#', <Whitespace>;
<Semantic_Instructions> PASSTHROUGH = <Delete>/<Passthrough>/<Collect>;
<Delete> = "DELETE";
<Passthrough> COLLECT = "PASSTHROUGH";
<Collect> COLLECT = "COLLECT"; #Comment#