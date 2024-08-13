<Alphabet_Upper> INLINE = ['A'..'Z'];
<Alphabet_Lower> INLINE = ['a'..'z'];
<onenine> INLINE = ['1'..'9'];
<digit> INLINE = ['0'..'9'];
<hex> INLINE = ['0'..'9']/['A'..'F'];
<Specials> INLINE = !(<Alphabet_Upper>/<Alphabet_Lower>/<onenine>/<ws>), <ASCII>;
<ASCII> INLINE = [0x00..0xFF];


<Grammar> = <json>; #Need a grammar rule for generated code to be correct# 
<json> = <element>;
<value> INLINE = <object>/<array>/<sstring>/<number>/<ttrue>/<ffalse>/<null>;
<ttrue> = "true";
<ffalse> = "false";
<null> = "null";

<object> = '{', ((<member>, (',', <member>)*)/<ws>), '}';
<members> INLINE = <member>, (',', <member>)*;
<member> = <ws>, <sstring>, <ws>, ':', <ws>, <element>, <ws>;
<array> = '[', (<elements>/<ws>), ']';
<elements> INLINE = <element>, (',', <element>)*;
<element> INLINE = <ws>, <value>, <ws>;
<sstring> = '"', (!('"'/('\', '\', <escape>)/'\'), <ASCII>)*, '"';
<character> = !('"'/('\', '\', <escape>)/'\'), <ASCII>; #Cannot handle UTF-8 yet so this isn't quite correct#
<escape> = ('\'/'b'/'f'/'n'/'r'/'t'/('u', <hex>, <hex>, <hex>, <hex>))?;
<integer> = '-'?, ((<onenine>, <digit>+)/<digit>);

<number> = <integer>, <fraction>?, <exponent>?;
<fraction> = ('.', <digit>+);
<exponent> = (('E'/'e'), <sign>, <digit>+);
<sign> = ('+'/'-')?;
<ws> INLINE = ('\n'/'\t'/'\r'/' ')*; #Can't use code points yet#

<wsa> = 0x0020;
<wsb> = 0x000A;
<wsc> = 0x000D;
<wsd> = 0x0009;
