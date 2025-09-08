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
    // let mut slab = {
    //     let file = std::fs::read("./memory.blob").unwrap().into_boxed_slice();
    //     let meta = std::fs::read("./memory.meta").unwrap().into_boxed_slice();
    //     HashMemorySlab::from_blob(file, meta)
    // };
    let mut slab = HashMemorySlab::new();
    {
        let file = std::fs::read_to_string("/home/blujay/Downloads/Hashes_FullPath").unwrap();
        let mut cache = slab.create_cache();
        for line in file.lines() {
            let path = Utf8Path::new(line);
            if let Some(extension) = path.extension() {
                slab.intern_path(&mut cache, Utf8Path::new(extension));
            }
            if let Some(file_name) = path.file_name() {
                slab.intern_path(&mut cache, Utf8Path::new(file_name));
            }
            slab.intern_path(&mut cache, Utf8Path::new(line)).range();
        }

        slab.finalize(cache);
    }

    let hashes_src = "/home/blujay/Downloads/Hashes_FullPath";

    for line in std::fs::read_to_string(hashes_src).unwrap().lines() {
        let line_hash = Hash40::const_new(line);
        let out = format!(
            "{}",
            DisplayHash {
                slab: &slab,
                hash: line_hash
            }
        );
        if line != out {
            panic!(
                "{} ({:#x}) != {} ({:#x})",
                line,
                line_hash.raw(),
                out,
                Hash40::const_new(&out).raw()
            );
        }
    }

    let mut cache = slab.create_cache();
    for line in std::fs::read_to_string("./extra.txt").unwrap().lines() {
        slab.intern_path(&mut cache, Utf8Path::new(line)).range();
    }
    slab.finalize(cache);

    for line in std::fs::read_to_string(hashes_src).unwrap().lines() {
        let line_hash = Hash40::const_new(line);
        let out = format!(
            "{}",
            DisplayHash {
                slab: &slab,
                hash: line_hash
            }
        );
        if line != out {
            panic!(
                "{} ({:#x}) != {} ({:#x})",
                line,
                line_hash.raw(),
                out,
                Hash40::const_new(&out).raw()
            );
        }
    }

    for component in slab
        .components_for(Hash40::const_new("fighter/this_is_a_test_file.txt"))
        .unwrap()
    {
        println!("{component}");
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
