#![no_std] //We need a freestanding binary and by default Rust crates link the std libarary thus we use no_std
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bareMetal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bareMetal_os::println;
use bootloader::{BootInfo, entry_point};
use bareMetal_os::memory::BootInfoFrameAllocator;
use bareMetal_os::memory::{translate_addr};
use x86_64::VirtAddr;
use x86_64::structures::paging::PageTable;
use bareMetal_os::allocator;
use bareMetal_os::memory::{self};

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

// mod vga_buffer;
// mod serial;


pub trait Testable {
    fn run (&self) -> ();
}

// impl<T> Testable for T
// where 
//     T: Fn(),

// {
//     fn run(&self) {
//         serial_print!("{}...\t", core::any::type_name::<T>());
//         self();
//         serial_println!("[ok]");
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    // loop {}
    bareMetal_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    bareMetal_os::test_panic_handler(_info);
    // serial_println!("[failed]\n");
    // serial_println!("Error: {}\n", _info);
    // exit_qemu(QemuExitCode::Failed);
    // loop {}
}

// static HEY: &[u8] = b"Hello Everyone I am BareMetal-OS Good to see Everyone";

// #[no_mangle] (while using entry point we don't need this or extern "C" fn _start)

// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;
//     for(buff, &byte) in HEY.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(buff as isize * 2) = byte;
//             *vga_buffer.offset(buff as isize * 2 + 1) = 0xb;
//         }
//     }
//     loop {}
// }

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // vga_buffer::print_on_Display();
    // use core::fmt::Write;
    // vga_buffer::SCREENWRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::SCREENWRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello ,Guys{}", "!");
    //panic!("Some garbage we don't understand :)");
    
    bareMetal_os::init();

    use bareMetal_os::{memory};
    use x86_64::{
        structures::paging::{Page, Translate}, VirtAddr};
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)
    };

    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (item, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", item, entry);
            
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!(" L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }
    
    // let addresses = [
    // // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,];

    // for &addr in &addresses {
    //     let virt = VirtAddr::new(addr);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }
    

    // x86_64::instructions::interrupts::int3();
    
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42; //This was to demonstrate double fault
    // }


    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // //Here we trigger the stack overflow
    // stack_overflow();
    // 
    // (testing if it is causing page fault )
    // let ptr = 0x20521a as *mut u8;

    // unsafe { let x = *ptr; }
    // println!("read worked");

    // unsafe { *ptr = 42; }
    // println!("write worked");

    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:
    // ?}", level_4_page_table.start_address());

    
    
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    let mut ptr = vec.as_slice().as_ptr();
    println!("vec is at {ptr:p}");

    for i in 0..500 {
        vec.push(i);
        let new_ptr = vec.as_ptr();
        if ptr == new_ptr {
            ptr = new_ptr;
            println!(" vec moved to {ptr:p}");
        }
    }

    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);

    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    bareMetal_os::hlt_loop();
}

// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) {

//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }

//     exit_qemu(QemuExitCode::Success);
// }

//  
