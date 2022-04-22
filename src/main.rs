mod msr;
mod temp;

use clap::{Args, Parser, Subcommand};
use std::io::SeekFrom;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read current values from MSR
    Read,
    /// Write new values to MSR
    Write(Write),
}

#[derive(Args)]
struct Write {
    #[clap(short, long)]
    target_temperature: u64,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Read => {
            let raw = msr::read_msr(0, &SeekFrom::Start(temp::MSR_TEMPERATURE_TARGET));

            // Get offset value
            let offset_value = msr::extract_value(
                raw,
                temp::TJ_MAX_TCC_OFFSET_HIGH_BIT,
                temp::TJ_MAX_TCC_OFFSET_LOW_BIT,
            );

            // Get tjmax value
            let tjmax_value = msr::extract_value(raw, temp::TJ_MAX_HIGH_BIT, temp::TJ_MAX_LOW_BIT);

            println!(
                "Temperature target: {} (Offset: {}, TjMax: {})",
                (tjmax_value - offset_value),
                offset_value,
                tjmax_value
            );
        }
        Commands::Write(args) => {
            // Get current value from MSR
            let seek = SeekFrom::Start(temp::MSR_TEMPERATURE_TARGET);
            let mut value = msr::read_msr(0, &seek);

            // Extract tjmax and calculate offset based on given target temperature
            let tjmax_value =
                msr::extract_value(value, temp::TJ_MAX_HIGH_BIT, temp::TJ_MAX_LOW_BIT);
            let offset = temp::calculate_offset(tjmax_value, args.target_temperature);

            // Clear our old value and set the new value to our bitfield
            msr::clear_and_set(
                &mut value,
                temp::TJ_MAX_TCC_OFFSET_HIGH_BIT,
                temp::TJ_MAX_TCC_OFFSET_LOW_BIT,
                offset,
            );

            println!(
                "Setting temperature target to {} (offset {}) (raw: {})",
                args.target_temperature, offset, value,
            );

            msr::write_msr(0, &seek, value.to_le_bytes()).unwrap();

            println!("MSR written!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example value with tjmax 100 and offset 15 used
    const TEST_REGISTER: u64 = 0xf640000;

    // Example value with tjmax 100 and offset 16 used
    const TEST_REGISTER2: u64 = 0x10640000;

    #[test]
    fn extract_offset() {
        let value = msr::extract_value(
            TEST_REGISTER,
            temp::TJ_MAX_TCC_OFFSET_HIGH_BIT,
            temp::TJ_MAX_TCC_OFFSET_LOW_BIT,
        );
        assert_eq!(15, value);
    }

    #[test]
    fn extract_tjmax() {
        let value = msr::extract_value(TEST_REGISTER, temp::TJ_MAX_HIGH_BIT, temp::TJ_MAX_LOW_BIT);
        assert_eq!(100, value);
    }

    #[test]
    fn test_change() {
        let mut register = TEST_REGISTER2;
        msr::clear_and_set(
            &mut register,
            temp::TJ_MAX_TCC_OFFSET_HIGH_BIT,
            temp::TJ_MAX_TCC_OFFSET_LOW_BIT,
            15,
        );
        assert_eq!(TEST_REGISTER, register);
    }
}
