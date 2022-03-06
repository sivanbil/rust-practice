use std::process;

fn main() {
    println!("{}", get_process_id());
}

fn get_process_id() -> u32 {
    process::id()
}

#[cfg(test)]
mod tests {
    use super::get_process_id;
    #[test]
    fn test_if_get_process_id_is_returned() {
        assert!(get_process_id() > 0);
    }
}