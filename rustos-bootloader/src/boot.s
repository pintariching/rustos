.section .boot, "awx" # Assemble the following code in a section named 'boot'
# With flags
# a - section is allocatable
# w - section is writable
# x - section is excecutable

.global _start # Makes the '_start' symbol visible to the linker
.code16 # Sets up 16 bit mode

BOOT_DRIVE: .byte 0
KERNEL_OFFSET: .int 0x1000
STACK_OFFSET: .int 0x7c00

_start:
    xor ax, ax # Set AX register to 0
    mov cs, ax # Set Code Segment register to 0
    mov ds, ax # Set Data Segment register to 0
    mov ss, ax # Set Stack Segment register to 0
    mov es, ax # Set Extra Segment register to 0
    mov fs, ax # Set General Purpose Segment register to 0
    mov gs, ax # Set General Purpose Segment register to 0

    # Clear the direction flag (e.g. go forward in memory when using instructions like lodsb)
    cld

    # Load boot drive number from register into variable
    mov [BOOT_DRIVE], dl

    # Set Stack Pointer to variable
    mov sp, [STACK_OFFSET]

# Enable A-20 line for 64 bits?

rust:
    push [BOOT_DRIVE] # Push boot drive number to stack
    call boot # Call the 'boot' function in main