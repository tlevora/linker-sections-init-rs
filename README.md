This crate provides couple of macros for linker section memory initialization on bare-metal devices.

# Example

Simple example defines a section `.custom_data` with start at 4-byte aligned `__scustom_data`
and 4-byte aligned end at `__ecustom_data`. The initialization data goes at `__sicustom_data`.

```text
MEMORY
{
    FLASH   : ORIGIN = 0x08000000, LENGTH = 32K
    RAM     : ORIGIN = 0x20000000, LENGTH = 16K
    DATA    : ORIGIN = 0x20004000, LENGTH = 16K
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
    } > DATA AT>FLASH

    __sicustom_data = LOADADDR(.custom_data);
} INSERT BEFORE .uninit;
```

In rust code it is needed to call macro `init_sections` with proper argument. The `init_sections`
macro shall be usually called before or at the beginning of `main` function.

```rust
#![no_std]
#![no_main]

use linker_sections::init_sections;
use {defmt_rtt as _, panic_probe as _};

const INITIAL_VALUE: u32 = 0xDEAD_BEEF;

#[unsafe(link_section = ".custom_data")]
static mut STATIC_VARIABLE: u32 = INITIAL_VALUE;

#[cortex_m_rt::pre_init]
unsafe fn pre_init() {
    init_sections!(custom_data);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // print initialized variable value
    unsafe {
        defmt::info!("STATIC_VARIABLE = 0x{:08X}", STATIC_VARIABLE);
    }

    loop {}
}
```
