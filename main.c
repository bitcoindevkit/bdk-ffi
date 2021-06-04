#include <stdlib.h>

#include "bdk_ffi.h"

int main (int argc, char const * const argv[])
{
    Point_t * a = new_point(84,45);
    Point_t * b = new_point(0.0,39.0);
    Point_t * m = mid_point(a, b);
    print_point(m);
    print_point(NULL);
    return EXIT_SUCCESS;
}
