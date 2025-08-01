<onenine> Inline = ['1'..'9'];
<digit> Inline = ['0'..'9'];
<integer> = <sign>, ((<onenine>, <digit>+)/<digit>);

<number> = <integer>, <fraction>?, <exponent>?; 
<fraction> = ('.', <digit>+);
<exponent> = (('E'/'e'), <sign>, <digit>+);
<sign> = ('+'/'-')?;

<addition> = <expr>, '+', <term>;
<subtraction> = <expr>, '-', <term>;
<multiplication> = <term>, '*', <power_expr>;
<division> = <term>, '/', <power_expr>;
<parentheses> = '(', <expr>, ')';
<power> = <factor>, '^', <power_expr>;

<expr>   =  <addition>/<subtraction>/<term>;
<term>   =   <multiplication>/<division>/<power_expr>;
<power_expr> = <power>/<factor>;
<factor> =   <parentheses>/<number>;
<Grammar> = <expr>;