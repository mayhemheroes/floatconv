#![no_main]
use libfuzzer_sys::fuzz_target;
use floatconv::{soft, fast};

fuzz_target!(|data: &[u8]| {
    if data.len() > 1 {
        let opt = data[0];
        let mut i = 1;
        match opt {
            0=>{
                while i + 4 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u32;
                    soft::f32_to_i128(float_in);
                    soft::f32_to_i64(float_in);
                    soft::f32_to_i32(float_in);
                    soft::f32_to_i16(float_in);
                    soft::f32_to_i8(float_in);
                    i += 4;
                }
            },
            1=>{
                while i + 4 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u32;
                    soft::f32_to_u128(float_in);
                    soft::f32_to_u64(float_in);
                    soft::f32_to_u32(float_in);
                    soft::f32_to_u16(float_in);
                    soft::f32_to_u8(float_in);
                    i += 4;
                }
            },
            2=>{
                while i + 4 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f32;
                    fast::f32_to_i128(float_in);
                    fast::f32_to_i64(float_in);
                    fast::f32_to_i32(float_in);
                    i += 4;
                }
            },
            3=>{
                while i + 4 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f32;
                    fast::f32_to_u128(float_in);
                    fast::f32_to_u64(float_in);
                    fast::f32_to_u32(float_in);
                    i += 4;
                }
            },
            4=>{
                while i + 8 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u64;
                    soft::f64_to_i128(float_in);
                    soft::f64_to_i64(float_in);
                    soft::f64_to_i32(float_in);
                    soft::f64_to_i16(float_in);
                    soft::f64_to_i8(float_in);
                    i += 8;
                }
            },
            5=>{
                while i + 8 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u64;
                    soft::f64_to_u128(float_in);
                    soft::f64_to_u64(float_in);
                    soft::f64_to_u32(float_in);
                    soft::f64_to_u16(float_in);
                    soft::f64_to_u8(float_in);
                    i += 8;
                }
            },
            6=>{
                while i + 8 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f64;
                    fast::f64_to_i128(float_in);
                    fast::f64_to_i64(float_in);
                    fast::f64_to_i32(float_in);
                    i += 8;
                }
            },
            7=>{
                while i + 8 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f64;
                    fast::f64_to_u128(float_in);
                    fast::f64_to_u64(float_in);
                    fast::f64_to_u32(float_in);
                    i += 8;
                }
            },
            8=>{
                while i + 16 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u128;
                    soft::u128_to_f32(float_in);
                    soft::u128_to_f64(float_in);
                    fast::u128_to_f32(float_in);
                    fast::u128_to_f64(float_in);
                    i += 16;
                }
            },
            9=>{
                while i + 16 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as i128;
                    soft::i128_to_f32(float_in);
                    soft::i128_to_f64(float_in);
                    fast::i128_to_f32(float_in);
                    fast::i128_to_f64(float_in);
                    i += 16;
                }
            },
            10=>{
                while i + 2 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u16;
                    soft::u16_to_f32(float_in);
                    soft::u16_to_f64(float_in);
                    i += 2;
                }
            },
            11=>{
                while i + 2 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as i16;
                    soft::i16_to_f32(float_in);
                    soft::i16_to_f64(float_in);
                    i += 2;
                }
            },
            12=>{
                while i + 1 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as u8;
                    soft::u8_to_f32(float_in);
                    soft::u8_to_f64(float_in);
                    i += 1;
                }
            },
            13=>{
                while i + 1 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as i8;
                    soft::i8_to_f32(float_in);
                    soft::i8_to_f64(float_in);
                    i += 1;
                }
            },
            _=>()
        }
    }
});
