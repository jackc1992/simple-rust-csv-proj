# Simple CSV bank app

## Running

This will write output to stdout. Errors are logged to stderr.

`cargo run -- input_file.csv`

## Architecture

* Where possible I've minimised allocations, with some exceptions. For instance, I didn't shy
away from using csv and serde's deserialize, rather than byterecord.

* I've kept it relatively open to make this work over tcp. The main lib input takes generic
writer and reader traits.

* Uses rust_decimal for precision

* To keep it memory efficient, I made transactions only save if it's a deposit, as the dispute
logic only impacts past deposits

* On that note, there's a `storage` trait. In a real system this would wrap around a database of
some sort to set a hard limit on memory usage.

* Custom errors!

* No ai was used to generate code
