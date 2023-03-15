# Hubris user library for tasks

This crate provides the Hubris system call interface and assorted utility code
for use in task programs.

## Crate features

- `log-itm` / `log-semihosting`: select one of two backends for the `log!`
  macros. If you provide neither, the log macros won't compile.

- Stripped panic handlers. On panic, the task will simply execute an invalid instruction
  in order to return to the supervisor. This is because panic info contains lots of absolute
  addresses that must be fixed for ROPI/RWPI, increasing the size of the HBF.