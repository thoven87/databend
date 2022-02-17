// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_datavalues2::prelude::*;
use common_exception::Result;
use common_functions::scalars::RegexpLikeFunction;

use crate::scalars::scalar_function2_test::test_scalar_functions2;
use crate::scalars::scalar_function2_test::ScalarFunction2Test;

#[test]
fn test_regexp_like_function() -> Result<()> {
    let tests = vec![
        ScalarFunction2Test {
            name: "regexp-like-two-column-passed",
            columns: vec![
                Series::from_data(vec!["abc", "abd", "Abe", "new*\n*line", "fo\nfo", ""]),
                Series::from_data(vec!["^a", "Ab", "abe", "new\\*.\\*line", "^fo$", ""]),
            ],
            expect: Series::from_data(vec![1i8, 1, 1, 0, 0, 1]),
            error: "",
        },
        ScalarFunction2Test {
            name: "regexp-like-three-column-passed",
            columns: vec![
                Series::from_data(vec!["abc", "abd", "Abe", "new*\n*line", "fo\nfo", ""]),
                Series::from_data(vec!["^a", "Ab", "abe", "new\\*.\\*line", "^fo$", ""]),
                Series::from_data(vec!["", "c", "i", "n", "m", "c"]),
            ],
            expect: Series::from_data(vec![1i8, 0, 1, 1, 1, 1]),
            error: "",
        },
        ScalarFunction2Test {
            name: "regexp-like-with-type-error",
            columns: vec![
                Series::from_data(vec!["abc", "abd"]),
                Series::from_data(vec![2, 3]),
            ],
            expect: Series::from_data(vec![1i8, 0]),
            error: "Expected string arg, but got [String, Int32]",
        },
        ScalarFunction2Test {
            name: "regexp-like-with-match-type-error",
            columns: vec![
                Series::from_data(vec!["abc"]),
                Series::from_data(vec!["abc"]),
                Series::from_data(vec!["x"]),
            ],
            expect: Series::from_data(vec![1i8]),
            error: "Incorrect arguments to REGEXP_LIKE match type: x",
        },
        ScalarFunction2Test {
            name: "regexp-like-with-match-type-error",
            columns: vec![
                Series::from_data(vec!["abc"]),
                Series::from_data(vec!["abc"]),
                Series::from_data(vec!["u"]),
            ],
            expect: Series::from_data(vec![1i8]),
            error: "Unsupported arguments to REGEXP_LIKE match type: u",
        },
        ScalarFunction2Test {
            name: "regexp-like-nullable-passed",
            columns: vec![
                Series::from_data(vec![Some("abc"), Some("abc"), None, Some("abc")]),
                Series::from_data(vec![Some("abc"), None, None, Some("abc")]),
                Series::from_data(vec![Some(""), Some("i"), Some("i"), None]),
            ],
            expect: Series::from_data(vec![Some(1i8), None, None, None]),
            error: "",
        },
    ];

    test_scalar_functions2(RegexpLikeFunction::try_create("regexp_like")?, &tests)
}
