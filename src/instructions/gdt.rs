//===================================================================================================================================================================================//
//
//   /$$$$$$  /$$           /$$                 /$$       /$$$$$$$                                          /$$$$$$$$        /$$       /$$          
//  /$$__  $$| $$          | $$                | $$      | $$__  $$                                        |__  $$__/       | $$      | $$          
// | $$  \__/| $$  /$$$$$$ | $$$$$$$   /$$$$$$ | $$      | $$  \ $$  /$$$$$$   /$$$$$$$  /$$$$$$$             | $$  /$$$$$$ | $$$$$$$ | $$  /$$$$$$ 
// | $$ /$$$$| $$ /$$__  $$| $$__  $$ |____  $$| $$      | $$  | $$ /$$__  $$ /$$_____/ /$$_____/             | $$ |____  $$| $$__  $$| $$ /$$__  $$
// | $$|_  $$| $$| $$  \ $$| $$  \ $$  /$$$$$$$| $$      | $$  | $$| $$$$$$$$|  $$$$$$ | $$                   | $$  /$$$$$$$| $$  \ $$| $$| $$$$$$$$
// | $$  \ $$| $$| $$  | $$| $$  | $$ /$$__  $$| $$      | $$  | $$| $$_____/ \____  $$| $$                   | $$ /$$__  $$| $$  | $$| $$| $$_____/
// |  $$$$$$/| $$|  $$$$$$/| $$$$$$$/|  $$$$$$$| $$      | $$$$$$$/|  $$$$$$$ /$$$$$$$/|  $$$$$$$ /$$         | $$|  $$$$$$$| $$$$$$$/| $$|  $$$$$$$
//  \______/ |__/ \______/ |_______/  \_______/|__/      |_______/  \_______/|_______/  \_______/|__/         |__/ \_______/|_______/ |__/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! Handles the partioning and layout of a Global Descriptor Table (GDT), along with its owned Task
//! State Segment (TSS) for the processing and switching of kernel stacks to safeguard against
//! unhandleable CPU interrrupts, as well as switching between and configurating kernel and user
//! spaces.
//!

use core::ptr::addr_of;

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{ Descriptor, GlobalDescriptorTable as GlobalDescTable, SegmentSelector };
use x86_64::instructions::tables::load_tss;
use x86_64::instructions::segmentation::{ CS, Segment };
use lazy_static::lazy_static;


/*
 * Constant & Static
 *      Declarations
 */


pub const DOUBLE_FAULT_IST_INDEX: u16   = 0;
    const STACK_SIZE:             usize = 4096 * 5;

lazy_static! {
    
    /// A Task State Segment handles the management and switching of kernel stacks in the event
    /// of a unhandleable CPU interruption or exception.
    static ref TSS: TaskStateSegment = {
        let mut tss: TaskStateSegment = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start: VirtAddr = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end:   VirtAddr = stack_start + STACK_SIZE;
            stack_end    // Stacks grow downward on x86-64
        };
        tss
    };

    /// A Global Descriptor Table, which owns the Task State Segment, and handles kernel/user mode
    /// configuration.
    static ref GDT: (GlobalDescTable, Selectors) = {
        let mut gdt:           GlobalDescTable = GlobalDescTable::new();
        let     code_selector: SegmentSelector = gdt.add_entry(Descriptor::kernel_code_segment());
        let     tss_selector:  SegmentSelector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        
        (gdt, Selectors { code_selector, tss_selector })
    };
}


/*
 * Segment
 *      Selectors
 */


/// Holds the Global Descriptor Table's selectors.
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector:  SegmentSelector
}


/*
 * Function
 *      Declarations
 */


/// Initializes the Global Descriptor Table, along with its own Task State Segment.
pub fn init() -> () {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
