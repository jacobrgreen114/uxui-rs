/*
  Copyright 2023 Jacob Green

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

extern crate freetype;
extern crate glm;
extern crate image;
extern crate lazy_static;
extern crate num_traits;
extern crate wgpu;
extern crate winit;

mod application;
mod component;
pub mod controls;
mod drawing;
pub mod font;
mod gfx;
pub mod input_handling;
pub mod layouts;
mod scene;
mod window;
mod datatypes;
pub use datatypes::*;

pub use self::application::*;
pub use self::scene::*;
pub use self::window::*;

use std::ops::*;
use std::sync::Arc;

pub struct StringProperty {
    value: Arc<String>,
}

impl StringProperty {
    pub fn new() -> Self {
        Self {
            value: Arc::new(String::new()),
        }
    }

    pub fn create_binding(&self) -> StringPropertyBinding {
        StringPropertyBinding {
            value: self.value.clone(),
        }
    }
}

impl From<&str> for StringProperty {
    fn from(value: &str) -> Self {
        Self {
            value: Arc::new(value.into()),
        }
    }
}

pub struct StringPropertyBinding {
    value: Arc<String>,
}

#[derive(Debug)]
pub enum BindableString {
    Static(String),
    // Binding(StringPropertyBinding),
}

impl Default for BindableString {
    fn default() -> Self {
        Self::Static(String::new())
    }
}

pub trait Builder<T: component::Component>: Sized {
    fn build(self) -> T;

    #[inline]
    fn build_boxed(self) -> Box<T> {
        Box::new(self.build())
    }
}

/**
 * Initializes the UXUI library in a multithreaded manner.
 * If this function is not called, the library will be initialized using lazy statics.
 */
pub fn initialize() {
    let t = std::thread::spawn(|| {
        font::initialize();
    });
    gfx::initialize();
    t.join().unwrap();
}
