<s> Inline = ' '; # Some whitespace is necessary for differentiating words # 
<ws_kernel> Inline = (' '/'\t'/'\r'/'\n'); # Some whitespace are never relevant # 
<ws> Inline = <ws_kernel>*;
<wsc> = (<ws_kernel>/<comment>/<multiline_comment>)*; # Comments should be retained for e.g formatters #

<digits> Inline = ['0'..'9'];
<character> Inline = [0x00..0xFF];
<letters> Inline = ['a'..'z']/['A'..'Z'];

<integer> = <digits>+;
<float> = <integer>, '.', (<integer>/'');
<char> = ''', <character>, ''';
<string> = '"', (!'"', <character>)*, '"';
<name_kernel> Inline = (<letters>/'_'), (<letters>/<digits>/'_')*; # Things can be ctypes or variables etc. #
<type_name> = <name_kernel>;
<variable_name> = <name_kernel>;

<comment> = "//", (!'\n', <character>)*, '\n';

<core_types> = ('f'/'u'/'i'), ('8'/"16"/"32"/"64");
<array> = '[', <s>*, <core_types>, <s>*, ';', <s>*, <integer>, <s>*, ']'; # Fixed size array # 
<type_expression> Inline = (<core_types>/<array>/<structure_definition>/<enumeration_definition>/<union_definition>/<type_name>);

<function_definition> = "fn"; # Function is very much still a work in progress # 
<function_type_parameters> = "<>"/'';
<function_parameters> = "()"/'';
<function_body> = '{', <expression>* ,'}';

<structure_definition> = "struct", '{', <variable_name>, ':', <type_expression>, ';', '}';
<enumeration_definition> = "enum", '{', <variable_name>, ':', <type_expression>, ';', '}';
<union_definition> = "union", '{', <variable_name>, ':', <type_expression>, ';', '}';
<complex_assignment> = '{', <variable_name>, '=', <expression>, ';', '}';


<type_definition> = "type", <type_name>, ':', <type_expression>, ';';
<variable_assignment> = <variable_name>, ':', <type_name>, '=', <expression>;
<statement> = <type_definition>/<variable_assignment>;

<expression> = <float>/<integer>/<char>/<string>/<variable_name>/<complex_assignment>;

<grammar> = (<type_definition>)*;