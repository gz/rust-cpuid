#include <stdint.h>

void c_cpuid(uint32_t* a, uint32_t* b, uint32_t* c, uint32_t* d) {
   asm volatile ("cpuid"
      : "+a"(*a), "=b"(*b), "+c"(*c), "=d"(*d)
   );
}
