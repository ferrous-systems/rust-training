/* Memory Configuration Linker Script

This file is imported by cortex-m-rt's link.x script, and should be placed
somewhere in the linker's search path.
*/

/*
Settings for AN385 and AN386 on MPS2
*/
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 4M
    RAM : ORIGIN = 0x20000000, LENGTH = 4M
  PSRAM : ORIGIN = 0x21000000, LENGTH = 16M
}

/*
Settings for AN500 on MPS2
*/
/*
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 4M
    RAM : ORIGIN = 0x20000000, LENGTH = 4M
  PSRAM : ORIGIN = 0x60000000, LENGTH = 16M
}
*/
