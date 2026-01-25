<ascii> Inline = [0x00..0xFF]; # Any ASCII char # 
<s> Inline = ' '; # Some whitespace is necessary for differentiating words # 
<ws_kernel> Inline = (' '/'\t'/'\r'/'\n'); # Some whitespace are never relevant # 
<ws> Inline = <ws_kernel>*;
<wsc> = (<ws_kernel>/<comment>/<multiline_comment>)*; # Comments should be retained for e.g formatters #
<multiline_comment> = "/*", (!"*/", <ascii>)*, "*/";
<comment> = "//", (!'\n', <ascii>)*;

<keyword> = "one"/ "of"/ "auto"/ "break"/ "case"/ "char"/ "const"/ "continue"/ "default"/ "do"/ "double"/ "else"/ "enum"/ "extern"/ "float"/ "for"/ "goto"/ "if"/ "inline"/ "int"/ "long"/ "register"/ "restrict"/ "return"/ "short"/ "signed"/ "sizeof"/ "static"/ "struct"/ "switch"/ "typedef"/ "union"/ "unsigned"/ "void"/ "volatile"/ "while"/ "_Alignas"/ "_Alignof"/ "_Atomic"/ "_Bool"/ "_Complex"/ "_Generic"/ " _Imaginary"/ "_Noreturn"/ "_Static_assert"/ "_Thread_local";
<identifier> = <identifier_nondigit>, (<identifier_nondigit>/<digit>)*;
#<identifier> =  (<identifier>, <identifier_nondigit>)
                /(<identifier>, <digit>)
                /<identifier_nondigit>;#
<identifier_nondigit> Inline = <nondigit>/<universal_character_name>;
<nondigit> = ['A'..'Z']/['a'..'z']/'_';
<digit> = ['0'..'9'];
<universal_character_name> = ("\\u", <hex_quad>)/("\\U", <hex_quad>, <hex_quad>);
<hex_quad> = <hexadecimal_digit>, <hexadecimal_digit>, <hexadecimal_digit>, <hexadecimal_digit>;

<constant> = <floating_constant>/
             <integer_constant>/
             <enumeration_constant>/
             <character_constant>;

<integer_constant> = (<decimal_constant>, <integer_suffix>?)/
                     (<binary_constant>, <integer_suffix>?)/
                     (<octal_constant>, <integer_suffix>?)/
                     (<hexadecimal_constant>, <integer_suffix>?);

<decimal_constant> = (<decimal_constant>, <digit>)/(<nonzero_digit>);

<binary_constant> = (<binary_constant>, <binary_digit>)/(<binary_prefix>, <binary_digit>);

<binary_prefix> = "0b"/"0B";
<binary_digit> = '0'/'1';
<octal_constant> = (<octal_constant>, <octal_digit>)/'0';
<hexadecimal_constant> = (<hexadecimal_constant>, <hexadecimal_digit>)
                         /(<hexadecimal_prefix>, <hexadecimal_digit>);
<hexadecimal_prefix> = "0x"/"0X";
<nonzero_digit> = ['1'..'9'];
<octal_digit> = ['0'..'7'];
<hexadecimal_digit> = ['0'..'9']/['A'..'F']/['a'..'f'];
<integer_suffix> =  (<unsigned_suffix>, <long_suffix>?)/ 
                    (<unsigned_suffix>, <long_long_suffix>?)/
                    (<long_suffix>, <unsigned_suffix>?)/
                    (<long_long_suffix>, <unsigned_suffix>?);
<unsigned_suffix> = 'u'/'U';
<long_suffix> = 'l'/'L';
<long_long_suffix> = "ll"/"LL";
<floating_constant> =   <decimal_floating_constant>/
                        <hexadecimal_floating_constant>;

<decimal_floating_constant> = (<fractional_constant>, <exponent_part>?, <floating_suffix>?)/
                              (<digit_sequence>, <exponent_part>, <floating_suffix>?);

<hexadecimal_floating_constant> = (<hexadecimal_prefix>, <hexadecimal_fractional_constant>, <binary_exponent_part>?, <floating_suffix>?)/
                                  (<hexadecimal_prefix>, <hexadecimal_digit_sequence>, <binary_exponent_part>, <floating_suffix>?);

<fractional_constant> = (<digit_sequence>?, '.', <digit_sequence>)
                        /(<digit_sequence>, '.');
<exponent_part> = ('e', <sign>?, <digit_sequence>) / ('E', <sign>?, <digit_sequence>);
<sign> = '+'/'-';
<digit_sequence> = <digit>*;
<hexadecimal_fractional_constant> = (<hexadecimal_digit_sequence>?, '.', <hexadecimal_digit_sequence>)/
                                    (<hexadecimal_digit_sequence>, '.');
<binary_exponent_part> =    ('p', <sign>?, <digit_sequence>)/
                            ('P', <sign>?, <digit_sequence>);
<hexadecimal_digit_sequence> =  <hexadecimal_digit>/
                                (<hexadecimal_digit_sequence>, <hexadecimal_digit>);
<floating_suffix> = 'f'/'l'/'F'/'L';
<enumeration_constant> = <identifier>;
<character_constant> =  (''', <c_char_sequence>, ''')/
                        ('L', ''', <c_char_sequence>, ''');
<c_char_sequence> = (<c_char_sequence>, <c_char>)/<c_char>;
<c_char> =  (!''', !'\', !'\n', <ascii>)
            /<escape_sequence>;

<escape_sequence> = <simple_escape_sequence>
                    / <octal_escape_sequence>
                    / <universal_character_name>; # Removed /<hexadecimal_escape_sequence> temporarily as breaking stuff#
                    
<simple_escape_sequence> = "PLACEHOLDER"; # PLACEHOLDER: HAVENT ADDED EVERYTHING YET, "\a"/ "\b"/ "\f"/ "\n"/ "\r"/ "\t"/ "\v"/ "\'"/ "\"" /"\\" /"\?";#
<octal_escape_sequence> =   ('\', <octal_digit>)
                            / ('\', <octal_digit>, <octal_digit>)
                            / ('\', <octal_digit>, <octal_digit>, <octal_digit>);
<hexadecimal_escape_sequence> = ("\\x", <hexadecimal_digit>)
	                            / (<hexadecimal_escape_sequence>, <hexadecimal_digit>);
<string_literal> = <encoding_prefix>?, '"', <s_char_sequence>?, '"';
<encoding_prefix> = "u8"
                    / 'u'
                    / 'U'
                    / 'L';

<s_char_sequence> = (<s_char_sequence>, <s_char>)/<s_char>;
<s_char> =  (!'"', !'\', !'\n', <ascii>)
            /<escape_sequence>;
<punctuator> = '['
	/ ']'
	/ '('
	/ ')'
	/ '{'
	/ '}'
	/ '.'
	/ "->"
	/ "++"
	/ "--"
	/ '&'
	/ '*'
	/ '+'
	/ '-'
	/ '~'
	/ '!'
	/ '/'
	/ '%'
	/ "<<"
	/ ">>"
	/ '<'
	/ '>'
	/ "<="
	/ ">="
	/ "=="
	/ "!="
	/ '^'
	/ '|'
	/ "&&"
	/ "||"
	/ '?'
	/ ':'
	/ ';'
	/ "..."
	/ '='
	/ "*="
	/ "/="
	/ "%="
	/ "+="
	/ "-="
	/ "<<="
	/ ">>="
	/ "&="
	/ "^="
	/ "|="
	/ ','
	/ '#'
	/ "##"
	/ "<:"
	/ ":>"
	/ "<%"
	/ "%>"
	/ "%:"
	/ "%:%:";
<header_name> = ('<', <h_char_sequence>, '>')
	/ ('"', <q_char_sequence>, '"');
<h_char_sequence> = <h_char>
	/ (<h_char_sequence>, <h_char>);
<h_char> = (!'\n', !'<', <ascii>);
<q_char_sequence> = <q_char>
	                / (<q_char_sequence>, <q_char>);
<q_char> = (!'\n', !'"', <ascii>);

<pp_number> = <digit>
	/ ('.', <digit>)
	/ (<pp_number>, <digit>)
	/ (<pp_number>, <identifier_nondigit>)
	/ (<pp_number>, 'e', <sign>)
	/ (<pp_number>, 'E', <sign>)
	/ (<pp_number>, 'p', <sign>)
	/ (<pp_number>, 'P', <sign>)
	/ (<pp_number>, '.');


<primary_expression> = <ws>,    (<identifier>
                                / <constant>
                                / <string_literal>
                                / ('(', <ws>, <expression>, <ws>, ')')
                                / <generic_selection>), <ws>;

<generic_selection> = "_Generic", '(', <assignment_expression>, ',', <generic_assoc_list>, ')';

<generic_assoc_list> = <generic_association>
	/ (<generic_assoc_list>, ',', <generic_association>);

<generic_association> = (<type_name>, ':', <assignment_expression>)
	/ ("default", ':', <assignment_expression>);

<postfix_expression> = <primary_expression>
	/ (<postfix_expression>, '[', <expression>, ']')
	/ (<postfix_expression>, '(', <argument_expression_list>?, ')')
	/ (<postfix_expression>, '.', <identifier>)
	/ (<postfix_expression>, "->", <identifier>)
	/ (<postfix_expression>, "++")
	/ (<postfix_expression>, "--")
	/ ('(', <type_name>, ')', '{', <initializer_list>, '}')
	/ ('(', <type_name>, ')', '{', <initializer_list>, ',', '}');

<argument_expression_list> = <assignment_expression>
	/ (<argument_expression_list>, ',', <assignment_expression>);

<unary_expression> = <postfix_expression>
	/ ("++", <unary_expression>)
	/ ("--", <unary_expression>)
	/ (<unary_operator>, <cast_expression>)
	/ ("sizeof", <unary_expression>)
	/ ("sizeof", '(', <type_name>, ')')
	/ ("_Alignof", '(', <type_name>, ')');

<unary_operator> = '&'
	/ '*'
	/ '+'
	/ '-'
	/ '~'
	/ '!';

<cast_expression> = <unary_expression>
	/ ('(', <type_name>, ')', <cast_expression>);

<multiplicative_expression> = <cast_expression>
	/ (<multiplicative_expression>, '*', <cast_expression>)
	/ (<multiplicative_expression>, '/', <cast_expression>)
	/ (<multiplicative_expression>, '%', <cast_expression>);

<additive_expression> = <multiplicative_expression>
	/ (<additive_expression>, '+', <multiplicative_expression>)
	/ (<additive_expression>, '-', <multiplicative_expression>);

<shift_expression> = <additive_expression>
	/ (<shift_expression>, "<<", <additive_expression>)
	/ (<shift_expression>, ">>", <additive_expression>);

<relational_expression> = <shift_expression>
	/ (<relational_expression>, '<', <shift_expression>)
	/ (<relational_expression>, '>', <shift_expression>)
	/ (<relational_expression>, "<=", <shift_expression>)
	/ (<relational_expression>, ">=", <shift_expression>);

<equality_expression> = <relational_expression>
	/ (<equality_expression>, "==", <relational_expression>)
	/ (<equality_expression>, "!=", <relational_expression>);

<AND_expression> = <equality_expression>
	/ (<AND_expression>, '&', <equality_expression>);

<exclusive_OR_expression> = <AND_expression>
	/ (<exclusive_OR_expression>, '^', <AND_expression>);

<inclusive_OR_expression> = <exclusive_OR_expression>
	/ (<inclusive_OR_expression>, '|', <exclusive_OR_expression>);

<logical_AND_expression> = <inclusive_OR_expression>
	/ (<logical_AND_expression>, "&&", <inclusive_OR_expression>);

<logical_OR_expression> = <logical_AND_expression>
	/ (<logical_OR_expression>, "||", <logical_AND_expression>);

<conditional_expression> = <logical_OR_expression>
	/ (<logical_OR_expression>, '?', <expression>, ':', <conditional_expression>);

<assignment_expression> = <conditional_expression>
	/ (<unary_expression>, <assignment_operator>, <assignment_expression>);

<assignment_operator> = '='
	/ "*="
	/ "/="
	/ "%="
	/ "+="
	/ "-="
	/ "<<="
	/ ">>="
	/ "&="
	/ "^="
	/ "|=";

<expression> = <assignment_expression>
	/ (<expression>, ',', <assignment_expression>);

<constant_expression> = <conditional_expression>;

<declaration> = (<ws>, <declaration_specifiers>, <ws>, <attribute_seq>?, <ws>, <init_declarator_list>?, <ws>, ';')
	/ (<ws>, <static_assert_declaration>, <ws>);

<declaration_specifiers> = <ws>,  ((<storage_class_specifier>, <ws>, <declaration_specifiers>?)
                                / (<type_specifier>, <ws>, <declaration_specifiers>?)
                                / (<type_qualifier>, <ws>, <declaration_specifiers>?)
                                / (<function_specifier>, <ws>, <declaration_specifiers>?)
                                / (<alignment_specifier>, <ws>, <declaration_specifiers>?)), <ws>;

<attribute_seq> = <attribute>
	/ (<attribute>, <attribute_seq>?);

<attribute> = "__asm"
	/ "__based"
	/ "__cdecl"
	/ "__clrcall"
	/ "__fastcall"
	/ "__inline"
	/ "__stdcall"
	/ "__thiscall"
	/ "__vectorcall";

<init_declarator_list> = <init_declarator>, (<ws>, ',', <ws>, <init_declarator>)*, <ws>;
<init_declarator> = <ws>, (<declarator>/(<declarator>, <ws>, '=', <ws>, <initializer>)), <ws>;

<storage_class_specifier> = "auto"
	/ "extern"
	/ "register"
	/ "static"
	/ "_Thread_local"
	/ "typedef"
	/ ("__declspec", '(', <extended_decl_modifier_seq>, ')');

<extended_decl_modifier_seq> = <extended_decl_modifier>
	/ (<extended_decl_modifier_seq>, <extended_decl_modifier>);

<extended_decl_modifier> = "thread"
	/ "naked"
	/ "dllimport"
	/ "dllexport";

<type_specifier> = "void"
	/ "char"
	/ "short"
	/ "int"
	/ "__int8"
	/ "__int16"
	/ "__int32"
	/ "__int64"
	/ "long"
	/ "float"
	/ "double"
	/ "signed"
	/ "unsigned"
	/ "_Bool"
	/ "_Complex"
	/ <atomic_type_specifier>
	/ <struct_or_union_specifier>
	/ <enum_specifier>
	/ <typedef_name>;

<struct_or_union_specifier> = (<ws>, <struct_or_union>, <ws>, <identifier>?, <ws>, '{', <ws>, <struct_declaration_list>, <ws>, '}', <ws>)
	                        / (<ws>, <struct_or_union>, <ws>, <identifier>, <ws>);

<struct_or_union> = "struct"
	/ "union";

<struct_declaration_list> = (<struct_declaration_list>, <ws>, <struct_declaration>, <ws>)
                            /(<ws>, <struct_declaration>, <ws>);

<struct_declaration> =  (<ws>, <specifier_qualifier_list>, <ws>, <struct_declarator_list>?, <ws>, ';')
	                    /(<ws>, <static_assert_declaration>, <ws>);
                        
<specifier_qualifier_list> = (<ws>, <type_specifier>, <ws>, <specifier_qualifier_list>?, <ws>)
	/ (<ws>, <type_qualifier>, <ws>, <specifier_qualifier_list>?, <ws>)
	/ (<ws>, <alignment_specifier>, <ws>, <specifier_qualifier_list>?, <ws>);

<struct_declarator_list> =  (<struct_declarator_list>, <ws>, ',', <ws>, <struct_declarator>, <ws>)
                            /(<ws>, <struct_declarator>, <ws>);

<struct_declarator> =   (<declarator>?, ':', <constant_expression>)
                        /<declarator>;

<enum_specifier> = (<ws>, "enum", <ws>, <identifier>?, <ws>, '{', <ws>, <enumerator_list>, <ws>, '}', <ws>)
	/ (<ws>, "enum", <ws>, <identifier>?, <ws>, '{', <ws>, <enumerator_list>, <ws>, ',', <ws>, '}', <ws>)
	/ (<ws>, "enum", <ws>, <identifier>, <ws>);

<enumerator_list> = <ws>, (<enumerator>, <ws>)*;

<enumerator> = <ws>, (<enumeration_constant>/(<enumeration_constant>, <ws>, '=', <ws>, <constant_expression>)), <ws>;

<atomic_type_specifier> = <ws>, "_Atomic", <ws>, '(', <ws>, <type_name>, <ws>, ')', <ws>;

<type_qualifier> = "const"
	/ "restrict"
	/ "volatile"
	/ "_Atomic";

<function_specifier> = "inline"
	/ "_Noreturn";

<alignment_specifier> = ("_Alignas", '(', <type_name>, ')')
	/ ("_Alignas", '(', <constant_expression>, ')');

<declarator> = <ws>, <pointer>?, <direct_declarator>, <ws>;

<direct_declarator> = 		('(', <ws>, <declarator>, <ws>, ')')
							/ (<direct_declarator>, '[', <type_qualifier_list>?, <assignment_expression>?, ']')
							/ (<direct_declarator>, '[', "static", <type_qualifier_list>?, <assignment_expression>, ']')
							/ (<direct_declarator>, '[', <type_qualifier_list>, "static", <assignment_expression>, ']')
							/ (<direct_declarator>, '[', <type_qualifier_list>?, '*', ']')
							/ (<direct_declarator>, '(', <parameter_type_list>, ')')
							/ (<direct_declarator>, '(', <identifier_list>?, ')')
							/<identifier>;

<pointer> = ('*', <type_qualifier_list>?)
	/ ('*', <type_qualifier_list>?, <pointer>);

<type_qualifier_list> = (<type_qualifier_list>, <type_qualifier>)/<type_qualifier>;

<parameter_type_list> = <parameter_list>
	/ (<parameter_list>, ',', "...");

<parameter_list> = 	(<parameter_list>, ',', <parameter_declaration>)
					/<parameter_declaration>;

<parameter_declaration> = <ws>, ((<declaration_specifiers>, <ws>, <declarator>)
	/ (<declaration_specifiers>, <ws>, <abstract_declarator>?)), <ws>;

<identifier_list> = (<identifier_list>, ',', <identifier>)/<identifier>;

<type_name> = <specifier_qualifier_list>, <abstract_declarator>?;

<abstract_declarator> = <pointer>
	/ (<pointer>?, <direct_abstract_declarator>);

<direct_abstract_declarator> = ('(', <abstract_declarator>, ')')
	/ (<direct_abstract_declarator>, '[', <type_qualifier_list>?, <assignment_expression>?, ']')
	/ (<direct_abstract_declarator>, '[', "static", <type_qualifier_list>?, <assignment_expression>, ']')
	/ (<direct_abstract_declarator>, '[', <type_qualifier_list>, "static", <assignment_expression>, ']')
	/ (<direct_abstract_declarator>, '[', <type_qualifier_list>?, '*', ']')
	/ (<direct_abstract_declarator>?, '(', <parameter_type_list>?, ')');

<typedef_name> = <identifier>, &' ';

<initializer> = <assignment_expression>
	/ ('{', <initializer_list>, '}')
	/ ('{', <initializer_list>, ',', '}');

<initializer_list> = (<designation>?, <initializer>)
	/ (<initializer_list>, ',', <designation>?, <initializer>);

<designation> = <designator_list>, '=';

<designator_list> = <designator>
	/ (<designator_list>, <designator>);

<designator> = ('[', <constant_expression>, ']')
	/ ('.', <identifier>);

<static_assert_declaration> = "_Static_assert", '(', <constant_expression>, ',', <string_literal>, ')', ';';

<statement> = <ws>, (<labeled_statement>
	/ <compound_statement>
	/ <expression_statement>
	/ <selection_statement>
	/ <iteration_statement>
	/ <jump_statement>
	/ <try_except_statement>
	/ <try_finally_statement>), <ws>;

<jump_statement> = <ws>, (("goto", <identifier>, ';')
	/ ("continue", ';')
	/ ("break", ';')
	/ ("return", <expression>?, ';')
	/ ("__leave", ';')), <ws>;

<compound_statement> = <ws>, ('{', <ws>, <declaration_list>?, <ws>, <statement_list>?, <ws>, '}'), <ws>;

<declaration_list> = <ws>, (<declaration>, <ws>)+;

<statement_list> = (<statement_list>, <statement>)/<statement>;

<expression_statement> = <expression>?, ';';

<iteration_statement> = ("while", '(', <expression>, ')', <statement>)
	/ ("do", <statement>, "while", '(', <expression>, ')', ';')
	/ ("for", '(', <expression>?, ';', <expression>?, ';', <expression>?, ')', <statement>);

<selection_statement> = ("if", '(', <expression>, ')', <statement>)
	/ ("if", '(', <expression>, ')', <statement>, "else", <statement>)
	/ ("switch", '(', <expression>, ')', <statement>);

<labeled_statement> = (<identifier>, ':', <statement>)
	/ ("case", <constant_expression>, ':', <statement>)
	/ ("default", ':', <statement>);

<try_except_statement> = ("__try", <compound_statement>, "__except", '(', <expression>, ')', <compound_statement>);

<try_finally_statement> = ("__try", <compound_statement>, "__finally", <compound_statement>);

<translation_unit> = (<translation_unit>, <external_declaration>)/<external_declaration>;

<external_declaration> = <ws>, (<function_definition>/<declaration>), <ws>;

<function_definition> = <ws>, <declaration_specifiers>?, <ws>, <declarator>, <ws>, <declaration_list>?, <ws>, <compound_statement>, <ws>;

<Grammar> = <ws>, <translation_unit>, <ws>;

