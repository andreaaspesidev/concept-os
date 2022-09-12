// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use bstringify;
pub use paste;

cfg_if::cfg_if! {
    if #[cfg(feature = "log-itm")] {
        #[macro_export]
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
        #[macro_export]
        macro_rules! sys_log {
            ($s:expr) => {
                { let _ = cortex_m_semihosting::hprintln!($s); }
            };
            ($s:expr, $($tt:tt)*) => {
                { let _ = cortex_m_semihosting::hprintln!($s, $($tt)*); }
            };
        }
    } else if #[cfg(feature = "log-null")] {
        #[macro_export]
        macro_rules! sys_log {
            ($s:expr) => {};
            ($s:expr, $($x:expr),*$(,)?) => {
                {
                    $(
                        let _ = &$x;
                    )*
                }
            };
        }
    } else {
        // Note: we provide macros that contain compiler_error, instead of just
        // using compiler_error here, to allow programs to omit these features
        // if they don't use logging.

        #[macro_export]
        macro_rules! sys_log {
            ($s:expr) => {
                compile_error!(concat!(
                        "to use sys_log! must enable either ",
                        "'log-semihosting' or 'log-itm' feature"
                ))
            };
            ($s:expr, $($tt:tt)*) => {
                compile_error!(concat!(
                        "to use sys_log! must enable either ",
                        "'log-semihosting' or 'log-itm' feature"
                ))
            };
        }
    }
}