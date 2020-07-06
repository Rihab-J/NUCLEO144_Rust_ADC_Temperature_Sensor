MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 256K
}


/* NOTE: Do *NOT* modify `_stack_start` unless you know what you are doing. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);