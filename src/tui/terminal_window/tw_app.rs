/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::{fmt::{Debug, Display},
          sync::Arc};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::*;

/// See [Component].
/// Async trait docs: <https://doc.rust-lang.org/book/ch10-02-traits.html>
#[async_trait]
pub trait TWApp<S, A>
where
  S: Display + Default + Clone + PartialEq + Eq + Debug + Sync + Send,
  A: Display + Default + Clone + Sync + Send,
{
  /// Use the state to render the output (via crossterm). To change the state, dispatch an action.
  async fn app_render(
    &mut self, state: &S, shared_store: &SharedStore<S, A>, window_size: Size,
  ) -> CommonResult<TWCommandQueue>;

  /// Use the input_event to dispatch an action to the store if needed.
  async fn app_handle_event(
    &mut self, input_event: &TWInputEvent, state: &S, shared_store: &SharedStore<S, A>,
    window_size: Size,
  ) -> CommonResult<EventPropagation>;

  /// Wrap a new instance in [Box].
  fn new_owned() -> BoxedSafeTWApp<S, A>
  where
    Self: Default + Sync + Send + 'static,
  {
    Box::new(Self::default())
  }

  /// Wrap a new instance in [std::sync::Arc] & [tokio::sync::RwLock].
  fn new_shared() -> SharedTWApp<S, A>
  where
    Self: Default + Sync + Send + 'static,
  {
    Arc::new(RwLock::new(Self::default()))
  }
}
