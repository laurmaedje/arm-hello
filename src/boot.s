.extern stack_top
.globl _start


// Start
_start:
    ldr x30, =stack_top
    mov sp, x30
    bl main


// System off
.equ _psci_system_off, 0x84000008
.globl system_off
system_off:
    ldr x0, =_psci_system_off
    hvc #0
