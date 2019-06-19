#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main()
{
    srand(time(NULL));
    for (int i = 0; i < 10000; ++i)
    {
        if (rand() % 2 == 1)
        {
            printf("\u2571");
        }
        else
        {
            printf("\u2572");
        }
    }
}