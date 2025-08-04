<num> = [0x30..0x39];
<expr> = (<expr>, '-', <num>) / <num>; # Should match 0-0-0-0-0-0-0-0 etc #
<Grammar> = <expr>; 