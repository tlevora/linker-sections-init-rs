MEMORY
{
    FLASH       : ORIGIN = 0x08000000, LENGTH = 30K
    CONSTS      : ORIGIN = 0x08007800, LENGTH =  2K
    STACK       : ORIGIN = 0x20000000, LENGTH =  4K
    RAM         : ORIGIN = 0x20001000, LENGTH =  4K
    CUSTOM_RAM  : ORIGIN = 0x20002000, LENGTH =  2K
}

SECTIONS
{
    .custom_data : ALIGN(4)
    {
        . = ALIGN(4);
        __scustom_data = .;
        *(.custom_data .custom_data.*);
        . = ALIGN(4);
        __ecustom_data = .;
    } > CUSTOM_RAM AT>CONSTS
    __sicustom_data = LOADADDR(.custom_data);
} INSERT AFTER .uninit;

_stack_start = ORIGIN(STACK) + LENGTH(STACK);
_stack_end = ORIGIN(STACK);
