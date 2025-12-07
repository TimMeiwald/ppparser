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

<valid_thing_name> Inline = (<letters>/'_'), (<letters>/<digits>/'_')*; # Things can be ctypes or variables etc. #
<name> = !(<reserved_words>), <valid_thing_name>;
<ctype> = <valid_thing_name>;

<reserved_words> = "void"/"int"/"float"/"double"; # Not all there yet # 


<function_declaration> = <function_header>, <wsc>, ';';
<function_definition> = <function_header>, <wsc>, <function_body>;
<function_header> = <ctype>, <ws>, <name>, <ws>, <function_parameters>; 
<function_parameters> = '(', <ws>, (<parameter>, <ws>, (',', <ws>, <parameter>)*)?, <ws>, ')', <wsc>;
<parameter> = <ctype>, <ws>, <name>;
<function_body> = '{', <wsc>, (<statement>, <wsc>)*, <wsc>, '}';

<function_call> = <name>, <ws>, '(', <ws>, (<name>, <ws>, (',', <ws>, <name>)*)?, <ws>, ')';
<statement> = (<function_call>/<return_statement>), <wsc>, ';'; #WIP#

<return_statement> = "return", <ws>, <expression>;
<expression> = <name>/<int>;

<Grammar> = <wsc>, (<function_definition>/<function_declaration>)*, <wsc>;