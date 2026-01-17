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

* Simple unit tests in lib.rs and bank_engine.rs

* Logging via eprintln. This was purely done to keep things simple as well as respect the need to
write to stdout.

## Design trade off

From a software tradeoff, it was tempting to consider some opportunities to maximise throughput.
One such option was to not use hashmaps at all for clients, and rather to just allocate all 2 ^ 16
clients as arrays. I could do some struct of arrays style thing here, using bit vectors to
determine whether the client was in use. I projected 4mb of data cost. A hashmap for this task
seemed like a reasonable choice to prioritise code simplicity

If I wanted to spend significantly more time on this the storage for transactions would be put into
a database. I did seriously consider using sled or fjall here, but I'm not convinced this is worth
the effort for a couple hour take home.

I seriously considered making the lib functions async in some manner. One of the options that was
appealing was to use tokio, and use a dashmap, another was to use io_uring and shard on client_id,
and minimise inter thread communication and locks.


Please enjoy.
