string = '''<Alphabet_Upper> PASSTHROUGH = "A"/"B"/"C"/"D"/"E"/"F"/"G"/"H"/"I"/"J"/"K"/"L"/"M"/"N"/"O"/"P"/"Q"/"R"/"S"/"T"/"U"/"V"/"W"/"X"/"Y"/"Z"; #We all love commments#
<Alphabet_Lower> PASSTHROUGH ="a"/"b"/"c"/"d"/"e"/"f"/"g"/"h"/"i"/"j"/"k"/"l"/"m"/"n"/"o"/"p"/"q"/"r"/"s"/"t"/"u"/"v"/"w"/"x"/"y"/"z";
<Num> PASSTHROUGH = "0"/"1"/"2"/"3"/"4"/"5"/"6"/"7"/"8"/"9";
<Spaces> PASSTHROUGH = "\n"/"\t"/"\r"/" ";
<Specials> PASSTHROUGH = "+"/"*"/"-"/"&"/"!"/"?"/"<"/">"/"""/"("/")"/"_"/","/"/"/";"/"="/"\"/"#"/":"/"|"/"."/"{"/"}"/"["/"]"/"%"/"'"/"^"/"~";
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

<Whitespace> DELETE = (" "/"\n"/"\r")*;
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

<lr> = <lr>, "A";
<A> = (<B>,"a"),"b";
<B> = (<A>,"b"),"a";'''

x = string.split(";")
sum = 0
for j in x:
    print(len(j), j)
    sum += len(j)
    print(sum)