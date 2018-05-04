#include <stdint.h>

void cpuid(uint32_t* a, uint32_t* b, uint32_t* c, uint32_t* d) {
#ifdef _MSC_VER
   uint32_t regs[4];
   __cpuidex((int*)regs, *a, *c);
   *a = regs[0], *b = regs[1], *c = regs[2], *d = regs[3];
#else
   asm volatile ("cpuid"
      : "+a"(*a), "=b"(*b), "+c"(*c), "=d"(*d)
   );
#endif
}
