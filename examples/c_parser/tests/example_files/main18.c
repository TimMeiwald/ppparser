

int main()
{
    float y = 20.5;
    union myStruct
    {
        int x;
        float y; // Should be just fine
        char z[30];
    };
}