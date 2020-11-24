use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_nanos() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_nanos_test() {
        // make sure current_nanos() doesn't panic
        current_nanos();
    }
}