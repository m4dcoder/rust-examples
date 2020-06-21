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

use std::error::Error;

#[allow(dead_code)]
struct Action {
    name: String,
    enabled: bool,
}

#[allow(dead_code)]
fn create_new_action(name: &str, enabled: bool) -> Result<Action, Box<dyn Error>> {
    let action = Action {name: name.to_string(), enabled};
    Ok(action)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typing;

    #[test]
    fn test_create_and_return_new_struct() {
        let action = create_new_action("noop", true).unwrap();
        assert_eq!("rust_examples::functions::Action", typing::type_of(&action));
        assert_eq!("noop", action.name);
        assert_eq!(true, action.enabled);
    }
}
