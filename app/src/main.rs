use std::alloc::System;

use camino::Utf8Path;
use interner::{DisplayHash, HashMemorySlab, InternerCache};
use smash_hash::Hash40;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

mod interner;

#[global_allocator]
static ALLOC: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    let hello = Hash40::const_new("hello");
    let hell = Hash40::const_new("hell");

    let poly = 0xedb88320;

    let mut table = [0u32; 256];

    for idx in 0..256u32 {
        let mut value = idx;
        for _ in 0..8 {
            value = (value >> 1) ^ (poly & (-(value as i32 & 1) as u32));
        }
        table[idx as usize] = value;
    }

    let mut reverse_table = [0u32; 256];
    for i in 0..256u32 {
        for j in 0..256u32 {
            if table[j as usize] >> 24 == i {
                reverse_table[i as usize] = j;
            }
        }
    }

    println!("{:?}", reverse_table);

    let mut data = [b'o'];
    let mut accum = hello.crc32();

    let mut stack = vec![(data.len(), !accum)];

    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        let prev_offset = node.0 - 1;
        let idx = (node.1 >> 24) & 0xFF;
        let index = reverse_table[idx as usize];
        println!("index: {index}");
        let prev_crc = ((node.1 ^ table[index as usize]) << 8) | (index ^ data[prev_offset] as u32);
        if prev_offset != 0 {
            stack.push((prev_offset, prev_crc));
        } else {
            println!("{:#x?}", !prev_crc);
        }
    }

    // println!("{:#x?}", reverse_table);

    println!("{:#x} vs {:#x}", hello.raw(), hell.raw());

    /*
    let reg = Region::new(&ALLOC);
    // let slab = {
    //     let file = std::fs::read("./memory.blob").unwrap().into_boxed_slice();
    //     let meta = std::fs::read("./memory.meta").unwrap().into_boxed_slice();
    //     HashMemorySlab::from_blob(file, meta)
    // };
    let mut slab = HashMemorySlab::new();
    {
        let file = std::fs::read_to_string("/home/blujay/Downloads/Hashes_FullPath").unwrap();
        let mut cache = InternerCache::default();
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
    */
}
