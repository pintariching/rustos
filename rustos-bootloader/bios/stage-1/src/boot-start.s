.section .boot, "awx" # Assemble the following code in a section named 'boot'
# With flags
# a - section is allocatable
# w - section is writable
# x - section is excecutable

.global _start # Makes the '_start' symbol visible to the linker
.code16 # Sets up 16 bit mode

_start:
    push dx
    call start