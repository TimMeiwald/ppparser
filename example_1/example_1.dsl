<expr> = <addition>/<subtraction>/<integer>;
<addition> = <expr>, '+', <integer>;
<subtraction> = <expr>, '-', <integer>;
<onenine> Inline = ['1'..'9'];
<digit> Inline = ['0'..'9'];
<integer> = <onenine>, <digit>*;

<Grammar> = <expr>;