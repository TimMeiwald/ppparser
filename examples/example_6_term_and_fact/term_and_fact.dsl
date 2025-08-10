<num> = [0x30..0x39];
<term> = (<term>, '+', <fact>)/(<term>, '-', <fact>)/<fact>;
<fact> = (<fact>, '*', <num>)/(<fact>, '/', <num>)/<num>;
<Grammar> = <term>; 