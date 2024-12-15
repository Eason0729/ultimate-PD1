use std::cell::OnceCell;
use std::collections::BTreeMap;

struct TestCase {
    input: &'static str,
    output: &'static str,
}

const TEST_CASES: OnceCell<BTreeMap<&'static str, TestCase>> = OnceCell::new();

pub fn get_input_output(problem: &str) -> (&'static str, &'static str) {
    let map = TEST_CASES;
    let map = map.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert(
            "A",
            TestCase {
                input: include_str!("../testcase/A.in"),
                output: include_str!("../testcase/A.out"),
            },
        );
        map
    });
    let test_case = map.get(problem).unwrap();

    let TestCase { input, output } = test_case;

    (input, output)
}
