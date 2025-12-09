struct myStruct
{
    int x;
    float y; // Should be just fine
};

int main()
{
    struct myStruct this_is_a_struct = {20, 10, "Hello there"};
}