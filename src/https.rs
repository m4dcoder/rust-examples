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

    #[test]
    fn test_basic_http() {
        let result = reqwest::blocking::get("https://www.google.com");
        let response = result.unwrap();
        assert_eq!(200, response.status());
        let body = response.text().unwrap();
        assert_eq!(true, body.contains("<html"));        
    }
}
