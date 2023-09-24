#[repr(C, packed)]
pub struct PartitionTableEntry {
    pub drive_number: u8,
    pub first_sector: CHSAddress,
    pub partition_type: u8,
    pub last_sector: CHSAddress,
    pub first_sector_lba: u32,
    pub sectors: u32,
}

impl PartitionTableEntry {
    pub fn from_raw_pointer<'a>(partition_table_start: *const u8) -> &'a Self {
        let ptr = (partition_table_start) as *const PartitionTableEntry;
        let partition_table = unsafe { &*ptr };

        partition_table
    }
}

#[repr(C, packed)]
pub struct CHSAddress {
    pub head: u8,
    pub sector: u8,
    pub cylinder: u8,
}
