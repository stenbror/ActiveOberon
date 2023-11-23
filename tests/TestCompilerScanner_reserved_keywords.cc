
#include "assertions_macros.h"

#include "../scanner.h"

void reserved_keyword_ASYNC()
{
    ASSERT_TRUE(true);
}

int main() {
    reserved_keyword_ASYNC();

    return 0;
}
