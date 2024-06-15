#include <iostream>

int g_max = 50;

static int arr_sum(int *list, int n)
{
    if (n == 0) return 0;
    int result = 0;
    for (int i = 0; i < n; i++) {
        result += list[i] <= g_max ? list[i] : g_max;
    }
    return result;
}

extern "C" int arr_sum_asm(int *list, int n);

int main(int argc, char **argv)
{
    int n = argc - 1;
    int num_list[n];
    for (int i = 1; i < argc; i++) num_list[i - 1] = atoi(argv[i]);
    std::cout << arr_sum(num_list, n) << std::endl;
    std::cout << arr_sum_asm(num_list, n) << std::endl;

    return 0;
}