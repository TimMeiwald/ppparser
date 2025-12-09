<digit> Inline = ['0'..'9'];
<character> Inline = [0x00..0xFF];
<letter> Inline = ['a'..'z']/['A'..'Z'];

<integer> = <digit>+;
<float> = <integer>, '.', (<integer>/'');
<char> = ''', <character>, ''';
<string> = '"', (!'"', <character>)*, '"';

<comment> = "//", (!'\n', <character>)*, '\n';

<initialize_type>

<type_expression> = <name>, '=', (<structure>/<enumeration>/<union>/<function>/);
<expression> = "WIP";