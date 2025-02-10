MEMORY
{
    FLASH       : ORIGIN = 0x08000000, LENGTH = 30K
    CONSTS      : ORIGIN = 0x08007800, LENGTH =  2K
    STACK       : ORIGIN = 0x20000000, LENGTH =  4K
    RAM         : ORIGIN = 0x20001000, LENGTH =  4K
    CUSTOM_RAM1 : ORIGIN = 0x20002000, LENGTH =  1K
    CUSTOM_RAM2 : ORIGIN = 0x20002400, LENGTH =  1K
}

SECTIONS
{
    .custom_data_a : ALIGN(4)
    {
        . = ALIGN(4);
        __scustom_data_a = .;
        *(.custom_data_a .custom_data_a.*);
        . = ALIGN(4);
        __ecustom_data_a = .;
    } > CUSTOM_RAM1 AT>CONSTS
    __sicustom_data_a = LOADADDR(.custom_data_a);

    .custom_data_b : ALIGN(4)
    {
        . = ALIGN(4);
        __scustom_data_b = .;
        *(.custom_data_b .custom_data_b.*);
        . = ALIGN(4);
        __ecustom_data_b = .;
    } > CUSTOM_RAM2 AT>CONSTS
    __sicustom_data_b = LOADADDR(.custom_data_b);
} INSERT AFTER .uninit;

_stack_start = ORIGIN(STACK) + LENGTH(STACK);
_stack_end = ORIGIN(STACK);
