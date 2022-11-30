extern crate bs;

use bs::parse::diagnostic::Diagnostic;
use bs::result::BSResult;
use bs::rt::runtime::Runtime;

macro_rules! bs_test {
    ($fun:tt, $b:expr, $r:expr) => {
        #[test]
        fn $fun() {
            let mut runtime = Runtime::new().expect("Failed to create runtime");
            let res = match runtime.parse_eval($b) {
                BSResult::Ok(result) => format!("{}", result),
                BSResult::Err(err) => panic!("{}", Diagnostic::new("TEST", $b, err)),
            };
            assert_eq!(res.as_str(), $r);
        }
    };
}

bs_test!(lit1, "1", "1");
bs_test!(lit2, "[1,2,3]", "[1, 2, 3]");
bs_test!(lit3, "-1", "-1");
bs_test!(binop1, "1+1", "2");
bs_test!(binop2, "4-3", "1");
bs_test!(binop3, "4 - 3", "1");
bs_test!(binop4, "4- 3", "1");
