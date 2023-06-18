#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use panic_reset as _;
use cortex_m_rt::entry;
use crate::ssd1306::SSD1306;
pub mod function;
mod ssd1306;
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let mut function = crate::function::Function::new();
    function.ssd1306_init();
    function.ssd1306_clear();
    loop{
        function.blinking();
        function.ssd1306_print(1, 1, "Temperatuer");
        function.ssd1306_print(3, 1, "Humidity");

        match function.sht31() {
            Ok(x) => {
                function.ssd1306_print(2, 1, format!("{:.2}", x[0]).as_str());
                function.ssd1306_print(4, 1, format!("{:.2}", x[1]).as_str());
            },
            Err(_) => {
                function.serial_print("error");
            }
        }
    }
}