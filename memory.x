/* memory.x - Linker script for the STM32f411CEU6 */
MEMORY
{
  /* Flash memory begins at 0x80000000 and has a size of 512kB*/
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  /* RAM begins at 0x20000000 and has a size of 128kB*/
  RAM :   ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
