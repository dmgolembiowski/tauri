// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
  menu::{builders::HandlerFn, CheckMenuItem, MenuEvent, MenuId},
  Manager, Runtime,
};

/// A builder type for [`CheckMenuItem`]
pub struct CheckMenuItemBuilder<R: Runtime> {
  id: Option<MenuId>,
  text: String,
  enabled: bool,
  checked: bool,
  accelerator: Option<String>,
  handler: Option<HandlerFn<CheckMenuItem<R>>>,
}

impl<R: Runtime> CheckMenuItemBuilder<R> {
  /// Create a new menu item builder.
  ///
  /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
  ///   for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
  pub fn new<S: AsRef<str>>(text: S) -> Self {
    Self {
      id: None,
      text: text.as_ref().to_string(),
      enabled: true,
      checked: true,
      accelerator: None,
      handler: None,
    }
  }

  /// Create a new menu item builder with the specified id.
  ///
  /// - `text` could optionally contain an `&` before a character to assign this character as the mnemonic
  ///   for this menu item. To display a `&` without assigning a mnemenonic, use `&&`.
  pub fn with_id<I: Into<MenuId>, S: AsRef<str>>(id: I, text: S) -> Self {
    Self {
      id: Some(id.into()),
      text: text.as_ref().to_string(),
      enabled: true,
      checked: true,
      accelerator: None,
      handler: None,
    }
  }

  /// Set the id for this menu item.
  pub fn id<I: Into<MenuId>>(mut self, id: I) -> Self {
    self.id.replace(id.into());
    self
  }

  /// Set the enabled state for this menu item.
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }

  /// Set the checked state for this menu item.
  pub fn checked(mut self, checked: bool) -> Self {
    self.checked = checked;
    self
  }

  /// Set the accelerator for this menu item.
  pub fn accelerator<S: AsRef<str>>(mut self, accelerator: S) -> Self {
    self.accelerator.replace(accelerator.as_ref().to_string());
    self
  }

  /// Set a handler to be called when this item is activated.
  pub fn handler<F: Fn(&CheckMenuItem<R>, MenuEvent) + Send + Sync + 'static>(
    mut self,
    handler: F,
  ) -> Self {
    self.handler.replace(Box::new(handler));
    self
  }

  /// Build the menu item
  pub fn build<M: Manager<R>>(self, manager: &M) -> crate::Result<CheckMenuItem<R>> {
    let i = if let Some(id) = self.id {
      CheckMenuItem::with_id(
        manager,
        id,
        self.text,
        self.enabled,
        self.checked,
        self.accelerator,
      )?
    } else {
      CheckMenuItem::new(
        manager,
        self.text,
        self.enabled,
        self.checked,
        self.accelerator,
      )?
    };

    if let Some(handler) = self.handler {
      let i = i.clone();
      manager
        .manager()
        .menu
        .on_menu_item_event(i.id().clone(), move |_app, e| {
          if e.id == i.id() {
            handler(&i, e)
          }
        });
    };

    Ok(i)
  }
}
