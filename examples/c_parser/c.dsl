<s> Inline = ' '; # Some whitespace is necessary for differentiating words # 
<ws_kernel> Inline = (' '/'\t'/'\r'/'\n'); # Some whitespace are never relevant # 
<ws> Inline = <ws_kernel>*;
<wsc> = (<ws_kernel>/<comment>/<multiline_comment>)*; # Comments should be retained for e.g formatters #

<ascii> Inline = [0x00..0xFF]; # Any ASCII char # 
<multiline_comment> = "/*", (!"*/", <ascii>)*, "*/";
<comment> = "//", (!'\n', <ascii>)*;

<letters> Inline = ['a'..'z']/['A'..'Z'];
<digits> Inline = ['0'..'9'];

<int> = <digits>+;
<floating_constant> = (<digits>+, '.', <digits>*), ('f'/'F'/'l'/'L'/'')?;
<string> = '"', (!'"', <ascii>)*, '"';


<valid_thing_name> Inline = (<letters>/'_'), (<letters>/<digits>/'_')*; # Things can be ctypes or variables etc. #
<name> = !(<reserved_words>), <valid_thing_name>;
<identifier> = <name>, ('.', <name>)*;
<ctype> = <valid_thing_name>;

<reserved_words> = "char"/"void"/"int"/"float"/"double"/"return"; # Not all there yet # 

<structure> = <wsc>, "struct", <ws>, <name>, <ws>, '{', <wsc>, (<statement_variable_declaration>, <wsc>)* ,'}', <wsc>, ';';

<union> = <wsc>, "union", <ws>, <name>, <ws>, '{', <wsc>, (<statement_variable_declaration>, <wsc>)* ,'}', <wsc>, ';';
<enum_value> Inline = <name>, <ws>, ('=', <ws>, <int>)?;
<enumeration> = <wsc>, "enum", <ws>, <name>, <ws>, '{', <wsc>, (<enum_value>, ',', <wsc>)*, (<enum_value>, <wsc>)? ,'}', <wsc>, ';';

<function_definition> = <wsc>, <function_header>, <wsc>, <function_body>;
<function_declaration> = <function_header>, <wsc>, ';';
<function_header> = <ctype>, <ws>, <name>, <ws>, <function_parameters>; 
<function_parameters> = '(', <ws>, (<parameter>, <ws>, (',', <ws>, <parameter>)*)?, <ws>, ')', <wsc>;
<parameter> = <ctype>, <ws>, <identifier>;
<function_body> = '{', <wsc>, (<statement>, <wsc>)*, '}', (<ws>, ';')?, <wsc>;

<function_call> = <name>, <ws>, '(', <ws>, (<expression>, <ws>, (',', <ws>, <expression>)*)?, <ws>, ')';


<array> = '[', <ws>, <int>, <ws>, ']';
<statement_variable_declaration> = <ctype>, <ws>, <name>, <array>?, <ws>, ';';

<expression> = <floating_constant>/<int>/<string>/<function_call>/<identifier>;
<complex_initialization> = '{', <wsc>, (<expression>, ',', <wsc>)*, (<expression>, <wsc>)? ,'}';
<statement_return> = "return", <ws>, <expression>, <ws>, ';', <wsc>;
<statement_modifier> = "struct"/"enum"/"union";
<statement> =    (<statement_modifier>?, <ws>, ((<ctype>, <ws>, <identifier>)/<identifier>), <ws>, '=', <ws>, (<expression>/<complex_initialization>), <ws>, ';')
                /(<statement_modifier>?, <ws>, <ctype>, <ws>, <name>, <ws>, ';')
                /<statement_return>
                /(<expression>, <ws>, ';')
                /<function_definition>
                /<function_declaration>
                /<union>
                /<structure>
                /<enumeration>;

<Grammar> = (<wsc>, <statement>)*, <wsc>;