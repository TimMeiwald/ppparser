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

<valid_thing_name> Inline = (<letters>/'_'), (<letters>/<digits>/'_')*; # Things can be ctypes or variables etc. #
<name> = !(<reserved_words>), <valid_thing_name>;
<ctype> = <valid_thing_name>;

<reserved_words> = "char"/"void"/"int"/"float"/"double"/"return"; # Not all there yet # 


<structure> = <wsc>, "struct", <ws>, <name>, <ws>, '{', <wsc>, (<statement_variable_declaration>, <wsc>)* ,'}', <wsc>, ';';
<enum_value> Inline = <name>, <ws>, ('=', <ws>, <int>)?;
<enumeration> = <wsc>, "enum", <ws>, <name>, <ws>, '{', <wsc>, (<enum_value>, ',', <wsc>)*, (<enum_value>, <wsc>)? ,'}', <wsc>, ';';

<function_declaration> = <function_header>, <wsc>, ';';
<function_definition> = <wsc>, <function_header>, <wsc>, <function_body>;
<function_header> = <ctype>, <ws>, <name>, <ws>, <function_parameters>; 
<function_parameters> = '(', <ws>, (<parameter>, <ws>, (',', <ws>, <parameter>)*)?, <ws>, ')', <wsc>;
<parameter> = <ctype>, <ws>, <name>;
<function_body> = '{', <wsc>, (<statement>, <wsc>)*, '}', (<ws>, ';')?, <wsc>;

<function_call> = <name>, <ws>, '(', <ws>, (<expression>, <ws>, (',', <ws>, <expression>)*)?, <ws>, ')', <ws>, ';', <wsc>;

<statement> = (<structure>/<enumeration>/<function_call>/<statement_return>/<statement_variable_assignment>);
<statement_return> = "return", <ws>, <expression>, <ws>, ';', <wsc>;

<array> = '[', <ws>, <int>, <ws>, ']';
<statement_variable_assignment> = <ctype>, <ws>, <name>, <array>?, <ws>, '=', <ws>, <expression>, <ws>, ';', <wsc>;
<statement_variable_declaration> = <ctype>, <ws>, <name>, <array>?, <ws>, ';';

<expression> = <name>/<floating_constant>/<int>;



<Grammar> = <wsc>, (<function_definition>/<function_declaration>/<structure>/<enumeration>)*, <wsc>;