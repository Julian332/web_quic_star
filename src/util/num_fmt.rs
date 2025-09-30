pub trait NumFmt {
    //eg: 1,000,000
    fn to_fmt_num_str(&self) -> String;
}

impl<T: ToString> NumFmt for T {
    fn to_fmt_num_str(&self) -> String {
        let num_str = self.to_string();
        let mut split = num_str.split('.');
        let mut integer = split
            .next()
            .expect("impossible decimal format")
            .as_bytes()
            .to_vec();
        integer.reverse();
        let scala = split.next();
        let mut count = 0;
        for i in 0..integer.len() + integer.len() / 3 {
            if i == 0 {
                continue;
            }
            if i % 3 == 0 {
                if i + count >= integer.len() {
                    break;
                }
                integer.insert(i + count, ',' as u8);
                count += 1;
            }
        }
        if *integer.last().expect("impossiable") == ',' as u8 {
            integer.pop();
        }
        integer.reverse();

        match scala {
            None => String::from_utf8(integer).unwrap(),
            Some(scala) => {
                let integer = String::from_utf8(integer).unwrap();
                format!("{}.{}", integer, scala)
            }
        }
    }
}
mod test {
    #![allow(unused_imports)]
    use crate::util::num_fmt::NumFmt;
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;
    #[test]
    fn test1() {
        let decimal = Decimal::from(123456);
        println!("{}", decimal.to_fmt_num_str());

        let decimal = Decimal::from(1234567);
        println!("{}", decimal.to_fmt_num_str());

        let decimal = Decimal::from_f64(123456789.123).unwrap();
        println!("{}", decimal.to_fmt_num_str());

        let decimal = Decimal::from_f64(12345678912312.12345).unwrap();
        println!("{}", decimal.to_fmt_num_str());
        // 123456789123123123321.12345
        // 123456789123123118080
        let decimal = Decimal::from(1_234_567_123_456_712_123u128);
        println!("{}", decimal.to_fmt_num_str());

        println!("{}", 10000.to_fmt_num_str());
        println!("{}", 10000.01.to_fmt_num_str());
    }
}
