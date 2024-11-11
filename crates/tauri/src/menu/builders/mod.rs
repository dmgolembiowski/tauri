// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(desktop)]

//! A module containing menu builder types

mod menu;
pub use menu::MenuBuilder;
pub use menu::SubmenuBuilder;
mod normal;
pub use normal::MenuItemBuilder;
mod check;
pub use check::CheckMenuItemBuilder;
mod icon;
pub use icon::IconMenuItemBuilder;
mod types {
  use crate::menu::MenuEvent;
  /// On a given [`MenuItem`] a handler function can be attached to be called
  /// whenever the associated event is emitted by the runtime.
  /// It is worth noting that while the fat pointer arguments do not explicitly
  /// hold [`AppState`], the calling code is allowed to clone it from a higher scope
  /// level, or indirectly reach it from within a [`tauri::generate_handler`]()'s target function.
  ///
  /// When the calling code needs to be explicit about the generic type information
  /// for `I`, it is intended to be used in a `impl<R: Runtime> ... { ... }` block
  /// and to be written as any of:
  /// - [`HandlerFn<tauri::menu::MenuItem<R>>`]
  /// - [`HandlerFn<tauri::menu::CheckMenuItem<R>>`]
  /// - [`HandlerFn<tauri::menu::IconMenuItem<R>>`]
  pub type HandlerFn<T> = Box<dyn Fn(&T, MenuEvent) + Send + Sync + 'static>;
}
pub use types::HandlerFn;
