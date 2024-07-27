<Alphabet_Upper> = "A"/"B"/"C"/"D"/"E"/"F"/"G"/"H"/"I"/"J"/"K"/"L"/"M"/"N"/"O"/"P"/"Q"/"R"/"S"/"T"/"U"/"V"/"W"/"X"/"Y"/"Z"; #We all love commments#
<Alphabet_Lower> ="a"/"b"/"c"/"d"/"e"/"f"/"g"/"h"/"i"/"j"/"k"/"l"/"m"/"n"/"o"/"p"/"q"/"r"/"s"/"t"/"u"/"v"/"w"/"x"/"y"/"z";
<Alphabet> = <Alphabet_Upper>/<Alphabet_Lower>;
<Num> = "0"/"1"/"2"/"3"/"4"/"5"/"6"/"7"/"8"/"9";
<Specials> = "+"/"*"/"-"/"&"/"!"/"?"/"<"/">"/"""/"("/")"/"_"/","/"/"/";"/"="/"\"/"#"/":"/"|"/"."/"{"/"}"/"["/"]"/"%"/"'"/"^"/"~"/"@"/"%";
<Spaces> = "\n"/"\t"/"\r"/" ";
<Ascii> = <Alphabet_Lower>/<Alphabet_Upper>/<Num>/<Spaces>/<Specials>;

<Grammar> = <json>; #Need a grammar rule for generated code to be correct# 
<json> = <element>;
<value> = <object>/<array>/<string>/<number>/<ttrue>/<ffalse>/<null>;
<ttrue> = "t", "r", "u", "e";
<ffalse> = "f", "a", "l", "s", "e";
<null> = "n", "u", "l", "l";

<object> = "{", (<members>/<ws>), "}";
<members> = <member>, (",", <member>)*;
<member> = <ws>, <string>, <ws>, ":", <ws>, <element>, <ws>;
<array> = "[", (<elements>/<ws>), "]";
<elements> = <element>, (",", <element>)*;
<element> = <ws>, <value>, <ws>;
<string> = """, <characters>, """;
<characters> = <character>*;
<character> = !("""/("\", "\", <escape>)/"\"), <Ascii>; #Cannot handle UTF-8 yet so this isn't quite correct#
<escape> = ("\"/"b"/"f"/"n"/"r"/"t"/("u", <hex>, <hex>, <hex>, <hex>))?;
<hex> = <digit>/<Alphabet_Upper>/<Alphabet_Lower>; #Needs to be A.F and a.f instead#
<integer> = ("-", <onenine>, <digits>)/(<onenine>, <digit>+)/("-", <digit>)/<digit>;
<number> = <integer>, <fraction>, <exponent>;
<digits> = <digit>+;
<digit> = "0"/<onenine>;
<onenine> = "1"/"2"/"3"/"4"/"5"/"6"/"7"/"8"/"9";
<fraction> = (".", <digit>+)?;
<exponent> = (("E", "e"), <sign>, <digits>)?;
<sign> = ("+"/"-")?;
<ws> = <Spaces>*; #Can't use code points yet#

<wsa> = "0", "0", "2", "0";
<wsb> = "0", "0", "0", "A";
<wsc> = "0", "0", "0", "D";
<wsd> = "0", "0", "0", "9";
