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
    use inline_python::Context;
    use inline_python::python;

    #[test]
    fn test_inline_py() {
        let c = Context::new();

        c.run(python! {
            z = 1 + 1 
        });

        assert_eq!(c.get::<i32>("z"), 2);
    }

    #[test]
    fn test_inline_py_pass_vars() {
        let c = Context::new();
        let x = 2;
        let y = 3;

        c.run(python! {
            z = 'x + 'y
        });

        assert_eq!(c.get::<i32>("z"), 5);
    }
}
