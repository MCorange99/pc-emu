

// struct Io {
//     readline_waiting: bool
// }

// impl Io {
    
//     pub fn new() -> Self {
//         Self {
//             readline_waiting: false,
//         }
//     }

// }


// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::std::print::macros::_print(format_args!($($arg)*)));
// }

#[macro_export]
macro_rules! putc {
    ($($arg:expr)*) => {
        crate::EMULATOR.lock().unwrap().get_mut().screen.putc($arg)
    };
}

#[macro_export]
macro_rules! puts {
    ($($arg:tt)*) => {
        crate::EMULATOR.lock().unwrap().get_mut().screen.puts(format_args!($($arg)*).to_string())
    };
}