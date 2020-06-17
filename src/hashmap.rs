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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_create_hashmap() {
        let mut m = HashMap::new();
        m.insert(String::from("foo"), String::from("boo"));
        m.insert(String::from("fu"), String::from("bar"));
        println!("{:?}", m);
    }

    #[test]
    fn test_iterate_kvps() {
        let mut m = HashMap::new();
        m.insert(String::from("foo"), String::from("boo"));
        m.insert(String::from("fu"), String::from("bar"));

        for (k, v) in &m {
            println!("{}: {}", k, v);
        }
    }
}
