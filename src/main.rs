#![allow(dead_code)]
mod bits;
mod data_inst;
mod error;
mod instruction;
mod math_inst;
mod register;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_macros {
    #[macro_export]
    macro_rules! assert_ok {
        ($val:ident) => {{
            assert!($val.is_ok(), "{:?}", $val.unwrap_err());
            $val.unwrap()
        }};
        ($e:expr) => {{
            let val = $e;
            assert_ok!(val)
        }};
    }

    #[macro_export]
    macro_rules! mkinst_arr {
        ($b0:expr) => {
            [$b0, 0, 0]
        };
        ($b0:expr, $b1:expr) => {
            [$b0, $b1, 0]
        };
        ($b0:expr, $b1:expr, $b2:expr) => {
            [$b0, $b1, $b2]
        };
    }

    #[macro_export]
    macro_rules! mkinst {
        ($msbs:expr, rp=$rp:expr, $nibble:expr) => {
            $msbs << 6 | (($rp as u8) << 4) | $nibble
        };
        ($msbs:expr, $triple0:expr, $triple1:expr) => {
            $msbs << 6 | (($triple0 as u8) << 3) | $triple1
        };
    }
}
