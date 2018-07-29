# ValvePak 
ValvePak format parser for Rust

## Limitations
This library limited to parse CS:GO VPK files for _now_. This means what VPKs with `file_data_section_size` not equals `0` is not supported for now.

## Examples
* `cargo run --example list <path to bundle_dir.vpk>` - list content of specified bundle file
* `cargo run --example extract <path to bundle_dir.vpk> <path to resutling dir>` - extracts content of specified bundle file
