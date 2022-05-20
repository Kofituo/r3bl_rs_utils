/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.

 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at

 *   http://www.apache.org/licenses/LICENSE-2.0

 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
*/

use r3bl_rs_utils::LazyField;

#[test]
fn test_name() {
  let boxed_compute_fn = Box::new(|| Ok(1));
  let mut thunk = LazyField::new(boxed_compute_fn);

  // First access to the field will trigger the computation.
  {
    let result = thunk.access_field();
    if result.is_err() {
      panic!("error");
    } else {
      let field_value = result.unwrap();
      assert_eq!(field_value, 1);
    }
  }

  // Subsequent accesses to the field will return the cached value.
  {
    let result = thunk.access_field();
    if result.is_err() {
      panic!("error");
    } else {
      let field_value = result.unwrap();
      assert_eq!(field_value, 1);
    }
  }
}