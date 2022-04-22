// Source https://www.intel.com/content/dam/www/public/us/en/documents/datasheets/10th-gen-core-families-datasheet-vol-2-datasheet.pdf
// Section 3.3.30 Temperature Target
// High and low values are bit ranges
pub const MSR_TEMPERATURE_TARGET: u64 = 0x1a2;

pub const TJ_MAX_TCC_OFFSET_HIGH_BIT: u64 = 29;
pub const TJ_MAX_TCC_OFFSET_LOW_BIT: u64 = 24;

pub const TJ_MAX_HIGH_BIT: u64 = 23;
pub const TJ_MAX_LOW_BIT: u64 = 16;

pub(crate) fn calculate_offset(tjmax: u64, target: u64) -> u64 {
    if target >= tjmax {
        return 0;
    }
    tjmax - target
}
