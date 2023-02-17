pub use bstringify;
pub use paste;

cfg_if::cfg_if! {
    if #[cfg(feature = "log-itm")] {
        macro_rules! sys_log {
            ($s:expr) => {
                unsafe {
                    let stim = &mut (*cortex_m::peripheral::ITM::PTR).stim[0];
                    cortex_m::iprintln!(stim, $s);
                }
            };
            ($s:expr, $($tt:tt)*) => {
                unsafe {
                    let stim = &mut (*cortex_m::peripheral::ITM::PTR).stim[0];
                    cortex_m::iprintln!(stim, $s, $($tt)*);
                }
            };
        }
    } else if #[cfg(feature = "log-semihosting")] {
        macro_rules! sys_log {
            ($s:expr) => {
                { let _ = cortex_m_semihosting::hprintln!($s); }
            };
            ($s:expr, $($tt:tt)*) => {
                { let _ = cortex_m_semihosting::hprintln!($s, $($tt)*); }
            };
        }
    } else {
        /*macro_rules! sys_log {
            ($s:expr) => {};
            ($s:expr, $($x:expr),*$(,)?) => {
                {
                    $(
                        let _ = &$x;
                    )*
                }
            };
        }*/
        compile_error!("Specify logging output");
    }
}

pub(crate) use sys_log;