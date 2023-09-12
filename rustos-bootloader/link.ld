ENTRY(_start)

SECTIONS {
    . = 0x500;
    _stack_start = .;
    . = 0x7c00;
    _stack_end = .;

    _mbr_start = .;
    .boot :
    {
        *(.boot .boot*)
    }
    _mbr_end = .;

    . = 0x7c00 + 510;
    .magic_number :
    {
        SHORT(0xaa55)
    }
}