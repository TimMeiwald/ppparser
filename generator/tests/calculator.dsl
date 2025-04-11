<onenine> Inline = ['1'..'9'];
<digit> Inline = ['0'..'9'];
<integer> = '-'?, ((<onenine>, <digit>+)/<digit>);

<number> = <integer>, <fraction>?, <exponent>?; 
<fraction> = ('.', <digit>+);
<exponent> = (('E'/'e'), <sign>, <digit>+);
<sign> = ('+'/'-')?;

<div_expr> = <expr_divmul>, '/', <expr_exponentiation>;
<mult_expr> = <expr_divmul>, '*', <expr_exponentiation>;
<add_expr> = <expr_addsub>, '+', <expr_divmul>;
<sub_expr> = <expr_addsub>, '-', <expr_divmul>;
<exponent_expr> = <expr_parentheses>, '^', <expr_parentheses>;
<parentheses_expr> = '(', <expr_addsub>, ')';
<term> = <number>; #For now we just use a json number and don't differentiate between integers and floats since it doesn't affect the left recursion test value#

<expr_parentheses> = <parentheses_expr>/<term>;
<expr_exponentiation> = <exponent_expr>/<expr_parentheses>;

<expr_divmul> = <div_expr>/<mult_expr>/<expr_exponentiation>;
<expr_addsub> = <add_expr>/<sub_expr>/<expr_divmul>;
<grammar> = <expr_addsub>;