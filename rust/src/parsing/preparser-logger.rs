// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8 {
    pub mod internal {

        /// A logger for pre-parsing information about functions.
        pub struct PreParserLogger {
            end: i32,
            num_parameters: i32,
            function_length: i32,
            num_inner_infos: i32,
        }

        impl PreParserLogger {
            /// Creates a new PreParserLogger with default values.
            pub fn new() -> Self {
                PreParserLogger {
                    end: -1,
                    num_parameters: -1,
                    function_length: -1,
                    num_inner_infos: -1,
                }
            }

            /// Logs information about a function.
            ///
            /// # Arguments
            ///
            /// * `end` - The end position of the function.
            /// * `num_parameters` - The number of parameters in the function.
            /// * `function_length` - The length of the function.
            /// * `num_inner_infos` - The number of inner infos in the function.
            pub fn log_function(&mut self, end: i32, num_parameters: i32, function_length: i32, num_inner_infos: i32) {
                self.end = end;
                self.num_parameters = num_parameters;
                self.function_length = function_length;
                self.num_inner_infos = num_inner_infos;
            }

            /// Returns the end position of the logged function.
            pub fn end(&self) -> i32 {
                self.end
            }

            /// Returns the number of parameters of the logged function.
            pub fn num_parameters(&self) -> i32 {
                self.num_parameters
            }

            /// Returns the length of the logged function.
            pub fn function_length(&self) -> i32 {
                self.function_length
            }

            /// Returns the number of inner infos of the logged function.
            pub fn num_inner_infos(&self) -> i32 {
                self.num_inner_infos
            }
        }
    }
}