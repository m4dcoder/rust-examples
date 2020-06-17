// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::type_name;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_obj_type_name() {
        let mut m = HashMap::new();
        m.insert(String::from("foo"), String::from("boo"));
        m.insert(String::from("fu"), String::from("bar"));

        assert_eq!(
            "std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>",
            type_of(m)
        );
    }
}
