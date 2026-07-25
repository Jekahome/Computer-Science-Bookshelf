/* linker.ld */
MEMORY
{
  rom (rx) : ORIGIN = 0x00000000, LENGTH = 2M
  ram (rwx): ORIGIN = 0x80000000, LENGTH = 8M
}

SECTIONS
{
  . = 0x00000000;

  .text :
  {
    *(.text*)
    *(.text.*)
  } > rom

  .rodata :
  {
    *(.rodata*)
    *(.rodata.*)
  } > rom

  __data_load = . ;     

  .data :
  {
    __data_start = . ;
    *(.data*)
    *(.sdata*)
    . = ALIGN(4);
    __data_end = . ;
  } > ram AT > rom

  .bss :
  {
    __bss_start = .;
    *(.bss*)
    *(.sbss*)
    . = ALIGN(4);
    __bss_end = .;
  } > ram

  __stack_top = ORIGIN(ram) + LENGTH(ram);
}