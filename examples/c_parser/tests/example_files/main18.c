

int main()
{
    float y = 20.5;
    union myUnion
    {
        int x;
        float y; // Should be just fine
        char z[30];
    };
    union myUnion x = { 0, 20, "Thing"};
}