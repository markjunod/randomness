use random_numbers::prelude::*;

const NUM_SAMPLES: u32 = 100_000;
const Z: f64 = 3.8906; // 99.99% confidence level for a t-test

pub fn is_random_number_uniform(random_number: &mut dyn RandomNumber, num_samples_opt: Option<u32>) -> Result<(), &'static str> {
    let num_samples = num_samples_opt.unwrap_or(NUM_SAMPLES);

    check_bool_uniform(random_number, num_samples)?;
    
    check_u8_uniform(random_number, num_samples)?;
    check_i8_uniform(random_number, num_samples)?;
    
    check_u16_uniform(random_number, num_samples)?;
    check_i16_uniform(random_number, num_samples)?;
    
    check_u32_uniform(random_number, num_samples)?;
    check_i32_uniform(random_number, num_samples)?;
    
    check_u64_uniform(random_number, num_samples)?;
    check_i64_uniform(random_number, num_samples)?;
    
    check_u128_uniform(random_number, num_samples)?;
    check_i128_uniform(random_number, num_samples)?;
    
    check_f32_uniform(random_number, num_samples)?;
    check_f64_uniform(random_number, num_samples)?;

    Ok(())
}

fn check_bool_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut trues = 0;
    for _ in 0..num_samples {
        if random_number.next_bool() {
            trues += 1;
        }
    }

    if !passes_t_test(trues, 0.5, num_samples) {
        Err("bool uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_u8_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let interval = ((u8::MAX as f64 + 1.0) / 8.0).round() as u8;
    let mut bucket_counts: [u32; 8] = [0; 8];
    for _ in 0..num_samples {
        let n = random_number.next_u8();
        if n < interval {
            bucket_counts[0] += 1;
        } else if n < interval * 2 {
            bucket_counts[1] += 1;
        } else if n < interval * 3 {
            bucket_counts[2] += 1;
        } else if n < interval * 4 {
            bucket_counts[3] += 1;
        } else if n < interval * 5 {
            bucket_counts[4] += 1;
        } else if n < interval * 6 {
            bucket_counts[5] += 1;
        } else if n < interval * 7 {
            bucket_counts[6] += 1;
        } else {
            bucket_counts[7] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("u8 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_i8_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut negative_count = 0;
    for _ in 0..num_samples {
        if random_number.next_i8() < 0 {
            negative_count += 1;
        }
    }

    if !passes_t_test(negative_count, 0.5, num_samples) {
        Err("i8 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_u16_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let interval = ((u16::MAX as f64 + 1.0) / 8.0).round() as u16;
    let mut bucket_counts: [u32; 8] = [0; 8];
    for _ in 0..num_samples {
        let n = random_number.next_u16();
        if n < interval {
            bucket_counts[0] += 1;
        } else if n < interval * 2 {
            bucket_counts[1] += 1;
        } else if n < interval * 3 {
            bucket_counts[2] += 1;
        } else if n < interval * 4 {
            bucket_counts[3] += 1;
        } else if n < interval * 5 {
            bucket_counts[4] += 1;
        } else if n < interval * 6 {
            bucket_counts[5] += 1;
        } else if n < interval * 7 {
            bucket_counts[6] += 1;
        } else {
            bucket_counts[7] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("u16 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_i16_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut negative_count = 0;
    for _ in 0..num_samples {
        if random_number.next_i16() < 0 {
            negative_count += 1;
        }
    }

    if !passes_t_test(negative_count, 0.5, num_samples) {
        Err("i16 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_u32_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let interval = ((u32::MAX as f64 + 1.0) / 8.0).round() as u32;
    let mut bucket_counts: [u32; 8] = [0; 8];
    for _ in 0..num_samples {
        let n = random_number.next_u32();
        if n < interval {
            bucket_counts[0] += 1;
        } else if n < interval * 2 {
            bucket_counts[1] += 1;
        } else if n < interval * 3 {
            bucket_counts[2] += 1;
        } else if n < interval * 4 {
            bucket_counts[3] += 1;
        } else if n < interval * 5 {
            bucket_counts[4] += 1;
        } else if n < interval * 6 {
            bucket_counts[5] += 1;
        } else if n < interval * 7 {
            bucket_counts[6] += 1;
        } else {
            bucket_counts[7] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("u32 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_i32_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut negative_count = 0;
    for _ in 0..num_samples {
        if random_number.next_i32() < 0 {
            negative_count += 1;
        }
    }

    if !passes_t_test(negative_count, 0.5, num_samples) {
        Err("i32 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_u64_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let interval = ((u64::MAX as f64 + 1.0) / 8.0).round() as u64;
    let mut bucket_counts: [u32; 8] = [0; 8];
    for _ in 0..num_samples {
        let n = random_number.next_u64();
        if n < interval {
            bucket_counts[0] += 1;
        } else if n < interval * 2 {
            bucket_counts[1] += 1;
        } else if n < interval * 3 {
            bucket_counts[2] += 1;
        } else if n < interval * 4 {
            bucket_counts[3] += 1;
        } else if n < interval * 5 {
            bucket_counts[4] += 1;
        } else if n < interval * 6 {
            bucket_counts[5] += 1;
        } else if n < interval * 7 {
            bucket_counts[6] += 1;
        } else {
            bucket_counts[7] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("u8 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_i64_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut negative_count = 0;
    for _ in 0..num_samples {
        if random_number.next_i64() < 0 {
            negative_count += 1;
        }
    }

    if !passes_t_test(negative_count, 0.5, num_samples) {
        Err("i64 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_u128_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let interval = ((u128::MAX as f64 + 1.0) / 8.0).round() as u128;
    let mut bucket_counts: [u32; 8] = [0; 8];
    for _ in 0..num_samples {
        let n = random_number.next_u128();
        if n < interval {
            bucket_counts[0] += 1;
        } else if n < interval * 2 {
            bucket_counts[1] += 1;
        } else if n < interval * 3 {
            bucket_counts[2] += 1;
        } else if n < interval * 4 {
            bucket_counts[3] += 1;
        } else if n < interval * 5 {
            bucket_counts[4] += 1;
        } else if n < interval * 6 {
            bucket_counts[5] += 1;
        } else if n < interval * 7 {
            bucket_counts[6] += 1;
        } else {
            bucket_counts[7] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("u8 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_i128_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut negative_count = 0;
    for _ in 0..num_samples {
        if random_number.next_i128() < 0 {
            negative_count += 1;
        }
    }

    if !passes_t_test(negative_count, 0.5, num_samples) {
        Err("i128 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_f32_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut bucket_counts: [u32; 10] = [0; 10];
    for _ in 0..num_samples {
        let r = random_number.next_f32();
        if r < 0.1 {
            bucket_counts[0] += 1;
        } else if r < 0.2 {
            bucket_counts[1] += 1;
        } else if r < 0.3 {
            bucket_counts[2] += 1;
        } else if r < 0.4 {
            bucket_counts[3] += 1;
        } else if r < 0.5 {
            bucket_counts[4] += 1;
        } else if r < 0.6 {
            bucket_counts[5] += 1;
        } else if r < 0.7 {
            bucket_counts[6] += 1;
        } else if r < 0.8 {
            bucket_counts[7] += 1;
        } else if r < 0.9 {
            bucket_counts[8] += 1;
        } else {
            bucket_counts[9] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("f32 uniformity check failed")
    } else {
        Ok(())
    }
}

fn check_f64_uniform(random_number: &mut dyn RandomNumber, num_samples: u32) -> Result<(), &'static str> {
    let mut bucket_counts: [u32; 10] = [0; 10];
    for _ in 0..num_samples {
        let r = random_number.next_f64();
        if r < 0.1 {
            bucket_counts[0] += 1;
        } else if r < 0.2 {
            bucket_counts[1] += 1;
        } else if r < 0.3 {
            bucket_counts[2] += 1;
        } else if r < 0.4 {
            bucket_counts[3] += 1;
        } else if r < 0.5 {
            bucket_counts[4] += 1;
        } else if r < 0.6 {
            bucket_counts[5] += 1;
        } else if r < 0.7 {
            bucket_counts[6] += 1;
        } else if r < 0.8 {
            bucket_counts[7] += 1;
        } else if r < 0.9 {
            bucket_counts[8] += 1;
        } else {
            bucket_counts[9] += 1;
        }
    }

    if !buckets_pass_t_test(&bucket_counts) {
        Err("f64 uniformity check failed")
    } else {
        Ok(())
    }
}

fn buckets_pass_t_test(bucket_counts: &[u32]) -> bool {
    let expected_percent = 1.0 / bucket_counts.len() as f64;
    let num_samples = bucket_counts.iter().fold(0, |sum, count| sum + count);

    bucket_counts.iter().fold(true, |passes, count| passes && passes_t_test(*count, expected_percent, num_samples))
}

fn passes_t_test(observed: u32, expected_percent: f64, num_samples: u32) -> bool {
    let r = (observed as f64) / (num_samples as f64);

    let e = Z / (2.0 * (num_samples as f64).sqrt());

    r - e < expected_percent && expected_percent < r + e
}
