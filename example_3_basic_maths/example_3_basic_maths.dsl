<onenine> Inline = ['1'..'9'];
<digit> Inline = ['0'..'9'];
<integer> = <sign>, ((<onenine>, <digit>+)/<digit>);

<number> = <integer>, <fraction>?, <exponent>?; 
<fraction> = ('.', <digit>+);
<exponent> = (('E'/'e'), <sign>, <digit>+);
<sign> = ('+'/'-')?;

<addition> = <expr>, '+', <term>;
<subtraction> = <expr>, '-', <term>;
<multiplication> = <term>, 'x', <factor>;
<division> = <term>, '/', <factor>;
<parentheses> = '(', <expr>, ')';

<expr>   =  <addition>/<subtraction>/<term>;
<term>   =   <multiplication>/<division>/<factor>;
<factor> =   <parentheses>/<number>;
<Grammar> = <expr>;