struct myStruct
{
    int x;
};

int main()
{   
    struct myStruct b;
    b.y = 5;
    int x = 10;
    int y = 20;
    struct myStruct a = { x, 0 };
    int x = a.x;
    return 0;
}