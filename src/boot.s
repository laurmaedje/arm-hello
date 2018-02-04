.globl _start
.equ _psci_system_off, 0x84000008
.globl system_off


// Start
_start:
	bl main


// System off
system_off:
	ldr x0, =_psci_system_off
    hvc #0