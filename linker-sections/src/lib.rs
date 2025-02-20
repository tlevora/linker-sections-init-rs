//! Couple of macros for linker section memory initialization. This crate is
//! designed for use on platforms with 32-bit aligned memory and 32-bit memory
//! access, but was tested with cortex-m cores only.
//!
//! This crate provides section memory initialization macro in a couple of
//! following variants.
//!  - [`init_sections`]
//!
//!     Use this macro if your section is defined using symbols
//!     - `__s<section>` for section VMA start,
//!     - `__e<section>` for section VMA end,
//!     - `__si<section>` for section LMA start.
//!
//!     ```
//!     init_sections!(buffers, sram2, sram3);
//!     ```
//!
//!  - [`init_sections_with_prefixes`]
//!
//!     Use if you want to specify your section boundary symbols manually.
//!
//!     ```
//!     init_sections_with_prefixes!(
//!         buffers(__s, __e, __si),
//!         sram2(__s, __e, __si),
//!         sram3(__s, __e, __si)
//!     );
//!     ```
//!
//! # Example
//!
//! Simple example defines a section `.custom_data` with start at 4-byte aligned `__scustom_data`
//! and 4-byte aligned end at `__ecustom_data`. The initialization data goes at `__sicustom_data`.
//!
//! ```text
//! MEMORY
//! {
//!     FLASH   : ORIGIN = 0x08000000, LENGTH = 32K
//!     RAM     : ORIGIN = 0x20000000, LENGTH = 16K
//!     DATA    : ORIGIN = 0x20004000, LENGTH = 16K
//! }
//!
//! SECTIONS
//! {
//!     .custom_data : ALIGN(4)
//!     {
//!         . = ALIGN(4);
//!         __scustom_data = .;
//!         *(.custom_data .custom_data.*);
//!
//!         . = ALIGN(4);
//!         __ecustom_data = .;
//!     } > DATA AT>FLASH
//!
//!     __sicustom_data = LOADADDR(.custom_data);
//! } INSERT BEFORE .uninit;
//! ```
//!
//! In rust code it is needed to call macro [`init_sections`] with proper argument. the
//! [`init_sections`] macro shall be usually called before or at the beginning of `main` function.
//!
//! ```
//! #![no_std]
//! #![no_main]
//!
//! use linker_sections::init_sections;
//! use {defmt_rtt as _, panic_probe as _};
//!
//! const INITIAL_VALUE: u32 = 0xDEAD_BEEF;
//!
//! #[unsafe(link_section = ".custom_data")]
//! static mut STATIC_VARIABLE: u32 = INITIAL_VALUE;
//! #[cortex_m_rt::pre_init]
//! unsafe fn pre_init() {
//!     init_sections!(custom_data);
//! }
//!
//! #[cortex_m_rt::entry]
//! fn main() -> ! {
//!     // print initialized variable value
//!     unsafe {
//!         defmt::info!("STATIC_VARIABLE = 0x{:08X}", STATIC_VARIABLE);
//!     }
//!
//!     loop {}
//! }
//! ```
//!
//! # Safety
//!
//! - The symbols must be 4-byte aligned.
//! - The symbols must point to memory with required access (read, write).
//! - The symbols must represent continuos memory.
//!
//! # Limitations
//!
//! - Each section's name shall be a valid rust function name. but it does not have to be snake_case.
//! - Only one macro can be called and it can be called at most once.

#![no_std]

#[doc(hidden)]
pub extern crate with_builtin_macros;
#[doc(hidden)]
pub use with_builtin_macros::{with_builtin, with_eager_expansions};

#[macro_export]
/// Defines pre-init function initializing linker section memory.
///
/// This macro accepts linker section names as arguments and assumes the linker symbols are named
/// after given section names prefixed with
///  - `__s` for section VMA's start (usually points to RAM)
///  - `__e` for section VMA's end (usually points to RAM)
///  - `__si` for section LMA's start (usually points to FLASH)
///
/// If the symbols in the linker script are named `__scustom_data`, `__ecustom_data` and `__sicustom_data`,
/// as depicted in an example below, the macro call should be
///
/// ```
/// init_sections!(custom_data)
/// ```
///
/// ```text
/// MEMORY
/// {
///     FLASH   : ORIGIN = 0x08000000, LENGTH = 32K
///     RAM     : ORIGIN = 0x20000000, LENGTH = 16K
///     DATA    : ORIGIN = 0x20004000, LENGTH = 16K
/// }
///
/// SECTIONS
/// {
///     .custom_data : ALIGN(4)
///     {
///         . = ALIGN(4);
///         __scustom_data = .;
///         *(.custom_data .custom_data.*);
///
///         . = ALIGN(4);
///         __ecustom_data = .;
///     } > DATA AT>FLASH
///
///     __sicustom_data = LOADADDR(.custom_data);
/// } INSERT BEFORE .uninit;
/// ```
///
/// Multiple section names could be passed as
///
/// ```
/// init_sections!(section_a, section_b, section_c);
/// ```
/// ```
/// init_sections!(section_a, section_b, section_c,);
/// ```
/// ```
/// init_sections!(section_a section_b section_c);
/// ```
macro_rules! init_sections {
    ($($section_name:ident$(,)?)+) => {
        $crate::init_sections_with_prefixes!($($section_name(__s, __e, __si),)*);
    };
}

#[macro_export]
/// Defines pre-init function initializing linker section memory.
///
/// This macro accepts linker section names and symbol prefixes as arguments. If your section
/// symbols are prefixed with `__s`, `__e` and `__si`, look at [`init_sections`].
///
/// If the symbols in the linker script are named `__scustom_data`, `__ecustom_data` and `__sicustom_data`,
/// as depicted in an example below, the macro call should be
///
/// ```
/// init_sections_with_prefixes!(custom_data(__s, __e, __si))
/// ```
///
/// ```text
/// MEMORY
/// {
///     FLASH   : ORIGIN = 0x08000000, LENGTH = 32K
///     RAM     : ORIGIN = 0x20000000, LENGTH = 16K
///     DATA    : ORIGIN = 0x20004000, LENGTH = 16K
/// }
///
/// SECTIONS
/// {
///     .custom_data : ALIGN(4)
///     {
///         . = ALIGN(4);
///         __scustom_data = .;
///         *(.custom_data .custom_data.*);
///
///         . = ALIGN(4);
///         __ecustom_data = .;
///     } > DATA AT>FLASH
///
///     __sicustom_data = LOADADDR(.custom_data);
/// } INSERT BEFORE .uninit;
/// ```
///
/// Multiple section names could be passed as
///
/// ```
/// init_sections_with_prefixes!(section_a(__s, __e __si), section_b(__s, __e, __si));
/// ```
/// ```
/// init_sections_with_prefixes!(section_a(__s,__e,__si),);
/// ```
/// ```
/// init_sections_with_prefixes!(
///     section_a(__s, __e, __si)
///     section_b(__s, __e, __si,)
///     section_c(__s __e __si)
/// );
/// ```
macro_rules! init_sections_with_prefixes {
    ($($section_name:ident($beg:ident,$end:ident,$src:ident)$(,)?)+) => {
        fn __init_sections() {$(
            $crate::section_init_with_prefixes!($section_name($beg, $end, $src));
            $section_name();
        )*}

        __init_sections();
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! section_init_with_prefixes {
    ($section_name:ident($beg:ident, $end:ident, $src:ident)) => {
        #[allow(non_snake_case)]
        fn $section_name() {
            $crate::with_eager_expansions! { $crate::pointer_mut!( #{ concat_idents!($beg, $section_name) } ) };
            $crate::with_eager_expansions! { $crate::pointer!( #{ concat_idents!($end, $section_name) } ) };
            $crate::with_eager_expansions! { $crate::pointer!( #{ concat_idents!($src, $section_name) } ) };

            let src: *const u32 = core::ptr::addr_of!(
                $crate::with_builtin! { let $name = concat_idents!($src, $section_name) in { $name } }
            );
            let dst: *mut u32 = core::ptr::addr_of_mut!(
                $crate::with_builtin! { let $name = concat_idents!($beg, $section_name) in { $name } }
            );
            let end: *const u32 = core::ptr::addr_of!(
                $crate::with_builtin! { let $name = concat_idents!($end, $section_name) in { $name } }
            );

            unsafe { $crate::section_init(dst, end, src); }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pointer {
    ($name:ident) => {
        unsafe extern "C" {
            static $name: u32;
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pointer_mut {
    ($name:ident) => {
        unsafe extern "C" {
            static mut $name: u32;
        }
    };
}

#[doc(hidden)]
pub unsafe fn section_init(dst: *mut u32, end: *const u32, src: *const u32) {
    // not using defmt::asserts since defmt is not initialized at the moment this function being executed

    #[cfg(feature = "asserts")]
    {
        // section start shall be less or equal to section end
        assert!(dst as *const u32 <= end);

        // src and dst must be 4-byte aligned because of 4-byte oriented memcopy
        assert!(src as usize % 4 == 0);
        assert!(dst as usize % 4 == 0);

        // to calculate section length, section end must be 4-byte aligned
        assert!(end as usize % 4 == 0);
    }

    let len = unsafe { end.offset_from(dst) } as usize;

    #[cfg(feature = "asserts")]
    {
        let src = src as usize;
        let dst = dst as usize;

        // check for memory region overlap
        assert!(src > dst + len || src + len < dst);
    }

    unsafe { core::ptr::copy_nonoverlapping(src, dst, len) };
}
