<ascii> Inline = [0x00..0xFF]; # Any ASCII char # 

<keyword> = "one"/ "of"/ "auto"/ "break"/ "case"/ "char"/ "const"/ "continue"/ "default"/ "do"/ "double"/ "else"/ "enum"/ "extern"/ "float"/ "for"/ "goto"/ "if"/ "inline"/ "int"/ "long"/ "register"/ "restrict"/ "return"/ "short"/ "signed"/ "sizeof"/ "static"/ "struct"/ "switch"/ "typedef"/ "union"/ "unsigned"/ "void"/ "volatile"/ "while"/ "_Alignas"/ "_Alignof"/ "_Atomic"/ "_Bool"/ "_Complex"/ "_Generic"/ " _Imaginary"/ "_Noreturn"/ "_Static_assert"/ "_Thread_local";
<identifier> =  <identifier_nondigit>/
                (<identifier>, <identifier_nondigit>)/
                (<identifier>, <digit>);
<identifier_nondigit> = <nondigit>/<universal_character_name>;
<nondigit> = ['A'..'Z']/['a'..'z']/'_';
<digit> = ['0'..'9'];
<universal_character_name> = ("\\u", <hex_quad>)/("\\U", <hex_quad>, <hex_quad>);
<hex_quad> = <hexadecimal_digit>, <hexadecimal_digit>, <hexadecimal_digit>, <hexadecimal_digit>;

<constant> = <integer_constant>/
             <floating_constant>/
             <enumeration_constant>/
             <character_constant>;

<integer_constant> = (<decimal_constant>, <integer_suffix>?)/
                     (<binary_constant>, <integer_suffix>?)/
                     (<octal_constant>, <integer_suffix>?)/
                     (<hexadecimal_constant>, <integer_suffix>?);

<decimal_constant> = (<nonzero_digit>)/
                     (<decimal_constant>, <digit>);

<binary_constant> = (<binary_prefix>, <binary_digit>)/
                    (<binary_constant>, <binary_digit>);

<binary_prefix> = "0b"/"0B";
<binary_digit> = '0'/'1';
<octal_constant> = '0'/(<octal_constant>, <octal_digit>);
<hexadecimal_constant> = (<hexadecimal_prefix>, <hexadecimal_digit>)/
                         (<hexadecimal_constant>, <hexadecimal_digit>);
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

<fractional_constant> = (<digit_sequence>?, '.', <digit_sequence>)/
                        (<digit_sequence>, '.');
<exponent_part> = ('e', <sign>?, <digit_sequence>) / ('E', <sign>?, <digit_sequence>);
<sign> = '+'/'-';
<digit_sequence> = <digit> / (<digit_sequence>, <digit>);
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
<c_char_sequence> = <c_char> / (<c_char_sequence>, <c_char>);
<c_char> = (!''', !'\', !'\n', !<escape_sequence>, <ascii>)*;
<escape_sequence> = <simple_escape_sequence>
                    / <octal_escape_sequence>
                    / <hexadecimal_escape_sequence>
                    / <universal_character_name>;
                    
<simple_escape_sequence> = "PLACEHOLDER"; #"\a"/ "\b"/ "\f"/ "\n"/ "\r"/ "\t"/ "\v"/ "\'"/ "\"" /"\\" /"\?";#
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

<s_char_sequence> = <s_char>/(<s_char_sequence>, <s_char>);
<s_char> = (!''', !'\', !'\n', !<escape_sequence>, <ascii>)*;
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


<primary_expression> = <identifier>
	/ <constant>
	/ <string_literal>
	/ ('(', <expression>, ')')
	/ <generic_selection>;

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

<declaration> = (<declaration_specifiers>, <attribute_seq>?, <init_declarator_list>?, ';')
	/ <static_assert_declaration>;

<declaration_specifiers> = (<storage_class_specifier>, <declaration_specifiers>?)
	/ (<type_specifier>, <declaration_specifiers>?)
	/ (<type_qualifier>, <declaration_specifiers>?)
	/ (<function_specifier>, <declaration_specifiers>?)
	/ (<alignment_specifier>, <declaration_specifiers>?);

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

<init_declarator_list> = <init_declarator>
	/ (<init_declarator_list>, ',', <init_declarator>);

<init_declarator> = <declarator>
	/ (<declarator>, '=', <initializer>);

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

<struct_or_union_specifier> = (<struct_or_union>, <identifier>?, '{', <struct_declaration_list>, '}')
	/ (<struct_or_union>, <identifier>);

<struct_or_union> = "struct"
	/ "union";

<struct_declaration_list> = <struct_declaration>
	/ (<struct_declaration_list>, <struct_declaration>);

<struct_declaration> = (<specifier_qualifier_list>, <struct_declarator_list>?, ';')
	/ <static_assert_declaration>;

<specifier_qualifier_list> = (<type_specifier>, <specifier_qualifier_list>?)
	/ (<type_qualifier>, <specifier_qualifier_list>?)
	/ (<alignment_specifier>, <specifier_qualifier_list>?);

<struct_declarator_list> = <struct_declarator>
	/ (<struct_declarator_list>, ',', <struct_declarator>);

<struct_declarator> = <declarator>
	/ (<declarator>?, ':', <constant_expression>);

<enum_specifier> = ("enum", <identifier>?, '{', <enumerator_list>, '}')
	/ ("enum", <identifier>?, '{', <enumerator_list>, ',', '}')
	/ ("enum", <identifier>);

<enumerator_list> = <enumerator>
	/ (<enumerator_list>, ',', <enumerator>);

<enumerator> = <enumeration_constant>
	/ (<enumeration_constant>, '=', <constant_expression>);

<atomic_type_specifier> = "_Atomic", '(', <type_name>, ')';

<type_qualifier> = "const"
	/ "restrict"
	/ "volatile"
	/ "_Atomic";

<function_specifier> = "inline"
	/ "_Noreturn";

<alignment_specifier> = ("_Alignas", '(', <type_name>, ')')
	/ ("_Alignas", '(', <constant_expression>, ')');

<declarator> = <pointer>?, <direct_declarator>;

<direct_declarator> = <identifier>
	/ ('(', <declarator>, ')')
	/ (<direct_declarator>, '[', <type_qualifier_list>?, <assignment_expression>?, ']')
	/ (<direct_declarator>, '[', "static", <type_qualifier_list>?, <assignment_expression>, ']')
	/ (<direct_declarator>, '[', <type_qualifier_list>, "static", <assignment_expression>, ']')
	/ (<direct_declarator>, '[', <type_qualifier_list>?, '*', ']')
	/ (<direct_declarator>, '(', <parameter_type_list>, ')')
	/ (<direct_declarator>, '(', <identifier_list>?, ')');

<pointer> = ('*', <type_qualifier_list>?)
	/ ('*', <type_qualifier_list>?, <pointer>);

<type_qualifier_list> = <type_qualifier>
	/ (<type_qualifier_list>, <type_qualifier>);

<parameter_type_list> = <parameter_list>
	/ (<parameter_list>, ',', "...");

<parameter_list> = <parameter_declaration>
	/ (<parameter_list>, ',', <parameter_declaration>);

<parameter_declaration> = (<declaration_specifiers>, <declarator>)
	/ (<declaration_specifiers>, <abstract_declarator>?);

<identifier_list> = <identifier>
	/ (<identifier_list>, ',', <identifier>);

<type_name> = <specifier_qualifier_list>, <abstract_declarator>?;

<abstract_declarator> = <pointer>
	/ (<pointer>?, <direct_abstract_declarator>);

<direct_abstract_declarator> = ('(', <abstract_declarator>, ')')
	/ (<direct_abstract_declarator>, '[', <type_qualifier_list>?, <assignment_expression>?, ']')
	/ (<direct_abstract_declarator>, '[', "static", <type_qualifier_list>?, <assignment_expression>, ']')
	/ (<direct_abstract_declarator>, '[', <type_qualifier_list>, "static", <assignment_expression>, ']')
	/ (<direct_abstract_declarator>, '[', <type_qualifier_list>?, '*', ']')
	/ (<direct_abstract_declarator>?, '(', <parameter_type_list>?, ')');

<typedef_name> = <identifier>;

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

<statement> = <labeled_statement>
	/ <compound_statement>
	/ <expression_statement>
	/ <selection_statement>
	/ <iteration_statement>
	/ <jump_statement>
	/ <try_except_statement>
	/ <try_finally_statement>;

<jump_statement> = ("goto", <identifier>, ';')
	/ ("continue", ';')
	/ ("break", ';')
	/ ("return", <expression>?, ';')
	/ ("__leave", ';');

<compound_statement> = ('{', <declaration_list>?, <statement_list>?, '}');

<declaration_list> = <declaration>
	/ (<declaration_list>, <declaration>);

<statement_list> = <statement>
	/ (<statement_list>, <statement>);

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

<translation_unit> = <external_declaration>
	/ (<translation_unit>, <external_declaration>);

<external_declaration> = <function_definition>
	/ <declaration>;

<function_definition> = <declaration_specifiers>?, <declarator>, <declaration_list>?, <compound_statement>;

<Grammar> = <translation_unit>;

