<Alphabet_Upper> = ['A'..'Z'];
<Alphabet_Lower> PASSTHROUGH = ['a'..'z'];
<onenine> PASSTHROUGH = [0x31..0x39];
<digit> = [0x30..0x39];
<hex>= [0x30..0x39]/[0x65..0x70];
<Specials> PASSTHROUGH = !(<Alphabet_Upper>/<Alphabet_Lower>/<onenine>/<ws>),[0x0..0xFF] ;
<ASCII> PASSTHROUGH = [0x00..0xFF];


<Grammar> = <json>; #Need a grammar rule for generated code to be correct# 
<json> = <element>;
<value> = <object>/<array>/<sstring>/<number>/<ttrue>/<ffalse>/<null>;
<ttrue> = "true";
<ffalse> = "false";
<null> = "null";

<object> = '{', (<members>/<ws>), '}';
<members> = <member>, (',', <member>)*;
<member> = <ws>, <sstring>, <ws>, ':', <ws>, <element>, <ws>;
<array> = '[', (<elements>/<ws>), ']';
<elements> = <element>, (',', <element>)*;
<element> = <ws>, <value>, <ws>;
<sstring> = '"', <characters>, '"';
<characters> = <character>*;
<character> = !('"'/('\', '\', <escape>)/'\'), <Ascii>; #Cannot handle UTF-8 yet so this isn't quite correct#
<escape> = ('\'/'b'/'f'/'n'/'r'/'t'/('u', <hex>, <hex>, <hex>, <hex>))?;
<integer> = ('-', <onenine>, [0x30..0x39]+)/(<onenine>, [0x30..0x39]+)/('-', [0x30..0x39])/[0x30..0x39];
<number> = <integer>, <fraction>, <exponent>;
<fraction> = ('.', [0x30..0x39]+)?;
<exponent> = (('E'/'e'), <sign>, [0x30..0x39]+)?;
<sign> = ('+'/'-')?;
<ws> = ('\n'/'\t'/'\r'/' ')*; #Can't use code points yet#

<wsa> = 0x0020;
<wsb> = 0x000A;
<wsc> = 0x000D;
<wsd> = 0x0009;
