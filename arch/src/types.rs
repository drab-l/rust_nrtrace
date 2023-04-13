pub mod a64 {
    include!(concat!(env!("OUT_DIR"), "/types_64.inc"));
}
pub mod a32 {
    include!(concat!(env!("OUT_DIR"), "/types_32.inc"));
}

pub mod madvise {
    include!(concat!(env!("OUT_DIR"), "/uni_header/madvise.inc"));
}

pub mod mmap {
    include!(concat!(env!("OUT_DIR"), "/uni_header/mmap.inc"));
}

pub mod stat {
    include!(concat!(env!("OUT_DIR"), "/uni_header/stat_newfstatat_flag.inc"));
    pub mod a64 {
        include!(concat!(env!("OUT_DIR"), "/header/a64/stat_newfstatat.inc"));
    }
    pub mod a32 {
        include!(concat!(env!("OUT_DIR"), "/header/a32/stat_newfstatat.inc"));
    }
}

