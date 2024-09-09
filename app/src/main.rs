use std::alloc::System;

use camino::Utf8Path;
use interner::{DisplayHash, HashMemorySlab, InternerCache};
use smash_hash::Hash40;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

mod interner;

#[global_allocator]
static ALLOC: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    let reg = Region::new(&ALLOC);
    // let slab = {
    //     let file = std::fs::read("./memory.blob").unwrap().into_boxed_slice();
    //     let meta = std::fs::read("./memory.meta").unwrap().into_boxed_slice();
    //     HashMemorySlab::from_blob(file, meta)
    // };
    let mut slab = HashMemorySlab::new();
    {
        let file = std::fs::read_to_string("/Users/blujay/Downloads/Hashes_FullPath").unwrap();
        let mut cache = InternerCache::default();
        for line in file.lines() {
            slab.intern_path(&mut cache, Utf8Path::new(line)).range();
        }
        slab.finalize(cache);
    }

    let blob = slab.dump_blob();
    let meta = slab.dump_meta();
    std::fs::write("./memory.blob", &blob).unwrap();
    std::fs::write("./memory.meta", &meta).unwrap();

    println!("Total memory usage: {:#?}", reg.change());

    println!(
        "{}",
        DisplayHash {
            slab: &slab,
            hash: Hash40::const_new("fighter/mario/model/body/c00/model.numdlb")
        }
    );

    println!("{}", slab.report());
}
