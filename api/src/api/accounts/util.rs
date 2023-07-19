use rand::Rng;

pub fn get_random_account_number() -> String {
    let mut an: String = "".to_string();

    for _ in 1..10 {
        let num = rand::thread_rng().gen_range(0..10);
        let str = num.to_string();
        an.push_str(&str);
    }

    an
}

#[cfg(test)]
mod tests {
    use super::get_random_account_number;

    #[test]
    fn right_size() {
        let an = get_random_account_number();
        assert_eq!(9, an.len());
    }
}
