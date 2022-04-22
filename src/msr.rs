use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

fn msr_open(
    cpu: usize,
    seek_from: &SeekFrom,
    opts: &mut OpenOptions,
) -> Result<File, std::io::Error> {
    let msr_fd = format!("/dev/cpu/{}/msr", cpu);

    // Open MSR fd, seek to the target MSR data
    let mut msr_fd = opts.open(msr_fd)?;
    msr_fd.seek(*seek_from)?;
    Ok(msr_fd)
}

pub(crate) fn read_msr(cpu: usize, seek_from: &SeekFrom) -> u64 {
    // Create buffer to read the temperature target MSR data to
    // MSR access device reads and writes are 8 bytes.
    // https://man7.org/linux/man-pages/man4/msr.4.html
    let mut buffer = [0u8; 8];

    // Open MSR fd, seek to the target MSR data
    let mut msr_fd = msr_open(cpu, seek_from, OpenOptions::new().read(true)).unwrap();
    msr_fd.read(&mut buffer).unwrap();

    // Convert buffer to something we can use
    u64::from_le_bytes(buffer)
}

pub(crate) fn write_msr(
    cpu: usize,
    seek_from: &SeekFrom,
    buf: [u8; 8],
) -> Result<(), std::io::Error> {
    let mut msr_fd = msr_open(cpu, seek_from, OpenOptions::new().write(true)).unwrap();
    msr_fd.write(&buf)?;
    Ok(())
}

pub(crate) fn extract_value(data: u64, high: u64, low: u64) -> u64 {
    let bits = high - low + 1;
    (data >> low) & ((1 << bits) - 1)
}

pub(crate) fn clear_and_set(data: &mut u64, high: u64, low: u64, value: u64) {
    // Get our bitfield size based in high and low indexes
    let bits = high - low + 1;

    // Calculate clear mask, clear our size amount of bits at starting index
    let clear_mask = !(((1u64 << bits) - 1) << low);

    // Clear our bitfield in current value
    *data &= clear_mask;

    // Set new value to our bitfield
    *data |= value << low;
}
