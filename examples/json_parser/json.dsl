<Alphabet_Upper> Inline = ['A'..'Z'];
<Alphabet_Lower> Inline = ['a'..'z'];
<onenine> Inline = ['1'..'9'];
<digit> Inline = ['0'..'9'];
<hex> Inline = ['0'..'9']/['A'..'F'];
<Specials> Inline = !(<Alphabet_Upper>/<Alphabet_Lower>/<onenine>/<ws>), <ASCII>;
<ASCII> Inline = [0x00..0xFF];


<Grammar> = <json>; #Need a grammar rule for generated code to be correct# 
<json> = <element>;
<value> Inline = <object>/<array>/<sstring>/<number>/<ttrue>/<ffalse>/<null>;
<ttrue> = "true";
<ffalse> = "false";
<null> = "null";

<object> = '{', ((<member>, (',', <member>)*)/<ws>), '}';
<members> Inline = <member>, (',', <member>)*;
<member> = <ws>, <sstring>, <ws>, ':', <ws>, <element>, <ws>;
<array> = '[', (<elements>/<ws>), ']';
<elements> Inline = <element>, (',', <element>)*;
<element> Inline = <ws>, <value>, <ws>;
<sstring> = '"', (!('"'/('\', '\', <escape>)/'\'), <ASCII>)*, '"';
<character> = !('"'/('\', '\', <escape>)/'\'), <ASCII>; #Cannot handle UTF-8 yet so this isn't quite correct#
<escape> = ('\'/'b'/'f'/'n'/'r'/'t'/('u', <hex>, <hex>, <hex>, <hex>))?;
<integer> = '-'?, ((<onenine>, <digit>+)/<digit>);

<number> = <integer>, <fraction>?, <exponent>?;
<fraction> = ('.', <digit>+);
<exponent> = (('E'/'e'), <sign>, <digit>+);
<sign> = ('+'/'-')?;
<ws> Inline = ('\n'/'\t'/'\r'/' ')*; #Can't use code points yet#

<wsa> = 0x0020;
<wsb> = 0x000A;
<wsc> = 0x000D;
<wsd> = 0x0009;
