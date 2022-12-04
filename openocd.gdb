target extended-remote :3333

set print asm-demangle on

set backtrace limit 32

break DefaultHandler
break HardFault
break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
# break main

monitor arm semihosting enable

load
continue

