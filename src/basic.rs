#[cfg(test)]
mod vars {
    #[test]
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    fn mutation() {
        // variable are by default immutable
        // any attempt to change its value will lead to compilation error
        let x = 5;
        let mut y = x;
        y = y + 2;
        assert_eq!(x + 2, y);
    }

    #[test]
    fn constants() {
        const MAX: u32 = 100;
        assert_eq!(MAX, 100);
    }

    #[test]
    // too confusing feature ?
    fn shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);
    }
}

#[cfg(test)]
mod data_types {
    #[test]
    fn scalar() {
        // Note
        // 1. multiple syntax(es) to associate data ype with a number
        // 2. rust performs bounds check
        // 3. bounds check is not efficient during typecasting
        // 4. prefix _ for unused variables, to avoid warning
        // 5. Interesting feature visual saperators
        let _x: u8 = 255; // 256 will lead to compilation error, rust seems to do bounds check
        let y: u16 = (2_u32.pow(16) - 1) as u16;
        let z: u32 = (2u64.pow(32) - 1) as u32;
        println!("{}, {}", y, z);

        let _z = 1_00_1000; // visual separator

        // similarly floating point
        let _a: f32 = 1.2;
        let pi: f64 = 22_f64 / 7.0;
        println!("{}", pi);
    }

    #[test]
    fn operators() {
        assert_eq!(2 + 2, 4);
        assert_eq!(2 - 2, 0);
        assert_eq!(2 * 2, 4);
        assert_eq!(2 % 2, 0);
    }

    #[test]
    fn boolean() {
        let _t = true;
        let _f: bool = false;
    }

    #[test]
    fn char() {
        // wow supports unicode characters with basic data types
        let _z = 'ℤ';
        let _heart_eyed_cat = '😻';
        // TODO, check memory
    }
}

#[cfg(test)]
mod compound_types {
    #[test]
    fn tuples() {
        let _t = (1, 2, 3);
        let t: (i32, i16, char) = (1, 2, '3');
        // println!("{}", t);
        assert_eq!(t.0, 1);
        assert_eq!(t.1, 2);
        assert_eq!(t.2, '3');

        // prefer this ?
        let (a, b, c) = t;
        assert_eq!(t.0, a);
        assert_eq!(t.1, b);
        assert_eq!(t.2, c);

        // access only one item
        let (_, y, _) = t;
        assert_eq!(t.1, y);
    }

    #[test]
    fn arrays() {
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        assert_eq!(months.len(), 12);
        assert_eq!(months[0], "January");
        assert_eq!(months[11], "December");
    }
}

#[cfg(test)]
mod functions {
    // skipping
    // 1. basic function description
    // 2. function parameters
    #[test]
    fn implicit_return() {
        assert_eq!(plus_one(9), 10);
    }

    fn plus_one(i: i32) -> i32 {
        // NOTE, no semicolon implies return
        i + 1
    }
}

#[cfg(test)]
mod control_flow {
    // skipping

    #[test]
    fn conditionals() {
        if true {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn repetition() {
        let mut count = 0;

        // 01. loop
        let result = loop {
            count += 1;
            if count == 10 {
                break count;
            }
        };
        assert_eq!(result, 10);

        // 02. while loop
        while count > 0 {
            count -= 1;
        }
        assert_eq!(count, 0);

        // 03. for loop range based
        for i in 1..10 {
            count = i;
            break;
        }
        assert_eq!(count, 1);

        // 04. for loop on iterator (foreach)
        let data = [10, 20, 30, 40, 50];
        for elem in data.iter() {
            count = *elem;
        }
        assert_eq!(count, 50);

        count = 0;
        for (key, _val) in data.iter().enumerate() {
            assert_eq!(key, count);
            // assert_eq!(_val, data[key as i32]);      // TODO: understand error
            count += 1;
        }
    }
}
