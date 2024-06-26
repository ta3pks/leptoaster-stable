/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{cell::RefCell, rc::Rc};

use leptos::*;

use crate::toast::{ToastBuilder, ToastData, ToastId, ToastLevel};

/// The global context of the toaster. You should provide this as a global context
/// in your root component to allow any component in your application to toast
/// using the same toast queue.
///
///  # Examples
/// ```
///  #[leptos::component]
///  fn App() -> impl leptos::IntoView {
///      leptoaster::provide_toaster();
///  }
///  ```
#[derive(Clone, Debug)]
pub struct ToasterContext {
    stats: Rc<RefCell<ToasterStats>>,
    pub queue: RwSignal<Vec<ToastData>>,
    defaults: Option<ToastBuilder>,
}

#[derive(Clone, Default, Debug)]
struct ToasterStats {
    visible: u32,
    total: u64,
}

impl ToasterContext {
    pub(crate) fn new_with_defaults(defaults: ToastBuilder) -> Self {
        ToasterContext {
            stats: Rc::new(RefCell::new(ToasterStats::default())),
            queue: create_rw_signal(Vec::new()),
            defaults: Some(defaults),
        }
    }
    /// Adds the supplied toast to the toast queue, displaying it onto the screen.
    ///
    /// # Examples
    /// ```
    /// #[leptos::component]
    /// fn Component() -> impl leptos::IntoView {
    ///     let toaster = leptoaster::expect_toaster();
    ///
    ///     toaster.toast(
    ///         leptoaster::ToastBuilder::new("My toast message.")
    ///             .with_expiry(1_500.into())
    ///     );
    /// }
    /// ```
    pub fn toast(&self, builder: ToastBuilder) {
        let toast = builder.build(self.stats.borrow().total + 1);

        let mut queue = self.queue.get_untracked();
        queue.push(toast);
        self.queue.set(queue);

        self.stats.borrow_mut().visible += 1;
        self.stats.borrow_mut().total += 1;
    }

    /// Quickly display an `info` toast with default parameters. For more customization,
    /// use the `toast` function.
    ///
    /// # Examples
    /// ```
    /// #[leptos::component]
    /// fn Component() -> impl leptos::IntoView {
    ///     let toaster = leptoaster::expect_toaster();
    ///     toaster.info("My toast message.");
    /// }
    /// ```
    pub fn info(&self, message: &str) {
        self.toast(
            self.defaults
                .as_ref()
                .map(|defaults| defaults.clone().with_message(message))
                .unwrap_or_else(|| ToastBuilder::new(message))
                .with_level(ToastLevel::Info),
        );
        // ToastBuilder::new(message).with_level(ToastLevel::Info));
    }

    /// Quickly display a `success` toast with default parameters. For more customization,
    /// use the `toast` function.
    ///
    /// # Examples
    /// ```
    /// #[leptos::component]
    /// fn Component() -> impl leptos::IntoView {
    ///     let toaster = leptoaster::expect_toaster();
    ///     toaster.success("My toast message.");
    /// }
    /// ```
    pub fn success(&self, message: &str) {
        self.toast(
            self.defaults
                .as_ref()
                .map(|defaults| defaults.clone().with_message(message))
                .unwrap_or_else(|| ToastBuilder::new(message))
                .with_level(ToastLevel::Success),
        );
    }

    /// Quickly display a `warn` toast with default parameters. For more customization,
    /// use the `toast` function.
    ///
    /// # Examples
    /// ```
    /// #[leptos::component]
    /// fn Component() -> impl leptos::IntoView {
    ///     let toaster = leptoaster::expect_toaster();
    ///     toaster.warn("My toast message.");
    /// }
    /// ```
    pub fn warn(&self, message: &str) {
        self.toast(
            self.defaults
                .as_ref()
                .map(|defaults| defaults.clone().with_message(message))
                .unwrap_or_else(|| ToastBuilder::new(message))
                .with_level(ToastLevel::Warn),
        );
    }

    /// Quickly display an `error` toast with default parameters. For more customization,
    /// use the `toast` function.
    ///
    /// # Examples
    /// ```
    /// #[leptos::component]
    /// fn Component() -> impl leptos::IntoView {
    ///     let toaster = leptoaster::expect_toaster();
    ///     toaster.error("My toast message.");
    /// }
    /// ```
    pub fn error(&self, message: &str) {
        self.toast(
            self.defaults
                .as_ref()
                .map(|defaults| defaults.clone().with_message(message))
                .unwrap_or_else(|| ToastBuilder::new(message))
                .with_level(ToastLevel::Error),
        );
    }

    /// Clears all currently visible toasts.
    ///
    /// # Examples
    /// ```
    /// #[leptos::component]
    /// fn Component() -> impl leptos::IntoView {
    ///     let toaster = leptoaster::expect_toaster();
    ///
    ///     toaster.toast(
    ///         leptoaster::ToastBuilder::new("My toast message.")
    ///             .with_expiry(None) // the toast will not self-expire
    ///     );
    ///
    ///     toaster.clear();
    /// }
    /// ```
    pub fn clear(&self) {
        for toast in &self.queue.get_untracked() {
            toast.clear_signal.set(true);
        }
    }

    /// Removes the toast corresponding with the supplied `ToastId`.
    pub fn remove(&self, toast_id: ToastId) {
        let index = self
            .queue
            .get_untracked()
            .iter()
            .enumerate()
            .find(|(_, toast)| toast.id == toast_id)
            .map(|(index, _)| index);

        if let Some(index) = index {
            let mut queue = self.queue.get_untracked();
            queue.remove(index);
            self.queue.set(queue);

            self.stats.borrow_mut().visible -= 1;
        }
    }
}

impl Default for ToasterContext {
    fn default() -> Self {
        ToasterContext {
            stats: Rc::new(RefCell::new(ToasterStats::default())),
            queue: create_rw_signal(Vec::new()),
            defaults: None,
        }
    }
}
