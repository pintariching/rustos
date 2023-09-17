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

#    xor ax, ax # Set AX register to 0
#    mov ds, ax # Set Data Segment register to 0
#    mov es, ax # Set Extra Segment register to 0
#    mov ss, ax # Set Stack Segment register to 0
 
#     # Read boot drive number to variable
#     mov [BOOT_DRIVE], dl
# 
#     # Clear the direction flag (e.g. go forward in memory when using instructions like lodsb)
#     cld
# 
#     # Setup stack
#     mov ax, [STACK_BASE]
#     mov ss, ax
#     add ax, [STACK_SIZE]
#     mov sp, ax
# 
# # Fast A20 line enable
# # On most newer computers starting with the IBM PS/2, the chipset has a FAST A20 option that can quickly enable the A20 line.
# # To enable A20 this way, there is no need for delay loops or polling, just 3 simple instructions.
# # To write to 0x92 only when necesarry and to make sure bit 0 is 0, do a conditional jump.
# fast_a20_enable:
#     in al, 0x92 # Read from IO port 0x92 into register al
#     test al, 2 # If al is 2, set flag zf to 1
#     jnz after_fast_a20_enable # If zf == 0 jump to after
#     or al, 2 # Set al to 2
#     and al, 0xfe
#     out 0x92, al # Output al to port 0x92
# after_fast_a20_enable:
# 
# rust:
#     push [BOOT_DRIVE] # Push boot drive number to stack
#    call stage1 # Call the 'boot' function in main

spin:
    hlt
    jmp spin
