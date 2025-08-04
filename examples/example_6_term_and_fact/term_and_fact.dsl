<num> = [0x30..0x39];
<test_term> = (<test_term>, '+', <test_fact>)/(<test_term>, '-', <test_fact>)/<test_fact>;
<test_fact> = (<test_fact>, '*', <num>)/(<test_fact>, '/', <num>)/<num>;
<Grammar> = <test_term>; 