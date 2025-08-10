<num> = [0x30..0x39];
<test_LR_num> = <num>;
<test_indirect_three_level_A> = (<test_indirect_three_level_B>, '-', <test_LR_num>) / <test_LR_num>;
<test_indirect_three_level_B> = <test_indirect_three_level_C>;
<test_indirect_three_level_C> = <test_indirect_three_level_D>;
<test_indirect_three_level_D> = <test_indirect_three_level_E>;
<test_indirect_three_level_E> = <test_indirect_three_level_A>;
<Grammar> = <test_indirect_three_level_A>;