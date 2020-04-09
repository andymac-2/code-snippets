#include <stdio.h>
int main(void)
{
    int a[5];
    printf("%p\n", &a);
    printf("%p\n", &a+1);
    printf("%p\n", &a[0]);
    printf("%p\n", &a[1]);
    printf("%lu  hello\n", sizeof a);
    return 0;
}