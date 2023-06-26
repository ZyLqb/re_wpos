use core::panic::PanicInfo;
pub mod printf;

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printf::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printf::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location(){
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            _info.message().unwrap()
        );
    }
    loop {}
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::printf::print(format_args!(concat!("\x1b[32m[INFO]:",concat!($fmt,"\x1b[0m")) $(, $($arg)+)?));
        println!(" \x1b[32m{}:{}\x1b[0m", file!(), line!());
    }
}