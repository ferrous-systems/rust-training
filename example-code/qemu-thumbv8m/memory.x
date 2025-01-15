/* Memory Configuration Linker Script

This file is imported by cortex-m-rt's link.x script, and should be placed
somwhere in the linker's search path.
*/

/*
Settings for AN505 and AN521 on MPS2

TODO: Add Non-Secure memory regions
*/
MEMORY
{
  FLASH : ORIGIN = 0x10000000, LENGTH = 4M
    RAM : ORIGIN = 0x28000000, LENGTH = 4M
  PSRAM : ORIGIN = 0x80000000, LENGTH = 16M
}

/*
Settings for AN524 on MPS3
*/
/* MEMORY
{
  FLASH_NS : ORIGIN = 0x00000000, LENGTH = 128K
     FLASH : ORIGIN = 0x10000000, LENGTH = 128K
    RAM_NS : ORIGIN = 0x20000000, LENGTH = 128K
       RAM : ORIGIN = 0x30000000, LENGTH = 128K
  QSPI_ROM : ORIGIN = 0x28000000, LENGTH = 16M
      DDR4 : ORIGIN = 0x60000000, LENGTH = 2048M
} */

/*
Settings for AN547 on MPS3
*/
/* MEMORY
{
     FLASH_NS : ORIGIN = 0x00000000, LENGTH = 512K
      CODE_NS : ORIGIN = 0x01000000, LENGTH = 2M
        FLASH : ORIGIN = 0x10000000, LENGTH = 512K
         CODE : ORIGIN = 0x11000000, LENGTH = 2M
      DTCM_NS : ORIGIN = 0x20000000, LENGTH = 512K
       RAM_NS : ORIGIN = 0x21000000, LENGTH = 4M
  QSPI_ROM_NS : ORIGIN = 0x28000000, LENGTH = 8M
         DTCM : ORIGIN = 0x30000000, LENGTH = 512K
          RAM : ORIGIN = 0x31000000, LENGTH = 4M
     QSPI_ROM : ORIGIN = 0x38000000, LENGTH = 8M
         DDR4 : ORIGIN = 0x60000000, LENGTH = 2048M
} */
