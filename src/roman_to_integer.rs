pub fn roman_to_int(s: String) -> i32 {
    let mut num = 0;
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            'I' => {
                if let Some(&next) = iter.peek() {
                    match next {
                        'V' => {
                            num += 4;
                            iter.next();
                        }
                        'X' => {
                            num += 9;
                            iter.next();
                        }
                        _ => {
                            num += 1;
                        }
                    }
                } else {
                    num += 1;
                }
            }
            'V' => num += 5,
            'X' => {
                if let Some(&next) = iter.peek() {
                    match next {
                        'L' => {
                            num += 40;
                            iter.next();
                        }
                        'C' => {
                            num += 90;
                            iter.next();
                        }
                        _ => {
                            num += 10;
                        }
                    }
                } else {
                    num += 10;
                }
            }
            'L' => num += 50,
            'C' => {
                if let Some(&next) = iter.peek() {
                    match next {
                        'D' => {
                            num += 400;
                            iter.next();
                        }
                        'M' => {
                            num += 900;
                            iter.next();
                        }
                        _ => num += 100,
                    }
                } else {
                    num += 100
                }
            }
            'D' => num += 500,
            'M' => num += 1000,
            _ => unreachable!(),
        }
    }
    num
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(roman_to_int("I".to_string()), 1);
    }

    #[test]
    fn three() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn four() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn five() {
        assert_eq!(roman_to_int("V".to_string()), 5);
    }

    #[test]
    fn nine() {
        assert_eq!(roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn ten() {
        assert_eq!(roman_to_int("X".to_string()), 10);
    }

    #[test]
    fn fifty() {
        assert_eq!(roman_to_int("L".to_string()), 50);
    }

    #[test]
    fn hundred() {
        assert_eq!(roman_to_int("C".to_string()), 100);
    }

    #[test]
    fn five_hundred() {
        assert_eq!(roman_to_int("D".to_string()), 500);
    }

    #[test]
    fn thousand() {
        assert_eq!(roman_to_int("M".to_string()), 1000);
    }

    #[test]
    fn test() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
