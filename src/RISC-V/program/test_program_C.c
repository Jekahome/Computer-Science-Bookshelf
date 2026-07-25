volatile unsigned char *uart = (unsigned char *)0x10000000;


void custom_putchar(char c)
{
    *uart = c;
}


void print_array(const char *arr, int size)
{
    for (int i = 0; i < size; i++)
    {
        custom_putchar(arr[i]);
    }
}


int main()
{
    static char message[] = "Hello C!\n";

    print_array(message, 9);

    while (1)
    {
    }

    return 0;
}