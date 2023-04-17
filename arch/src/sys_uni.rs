#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
include!(concat!(env!("OUT_DIR"), "/sys_uni.rs.enum.inc"));
include!(concat!(env!("OUT_DIR"), "/sys_uni.rs.to_str.inc"));
include!(concat!(env!("OUT_DIR"), "/sys_uni.rs.map.inc"));
pub mod a64 {
include!(concat!(env!("OUT_DIR"), "/sys_64.inc"));
}
pub mod a32 {
include!(concat!(env!("OUT_DIR"), "/sys_32.inc"));
}