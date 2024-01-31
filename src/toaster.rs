/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod context;

use leptos::*;
use crate::toaster::context::ToasterContext;
use crate::toast::Toast;

/// Creates the toaster containers as fixed-position elements on the corners of the screen.
///
/// # Examples
/// ```
/// use leptos::*;
/// use leptoaster::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Toaster />
///     }
/// }
/// ```
#[component]
pub fn Toaster() -> impl IntoView {
	let toaster = expect_toaster();

	view! {
		<style>
			"
			@keyframes leptoaster-slide-in-left {
				from { left: -344px }
				to { left: 0 }
			}

			@keyframes leptoaster-slide-out-left {
				from { left: 0 }
				to { left: -344px }
			}

			@keyframes leptoaster-slide-in-right {
				from { right: -344px }
				to { right: 0 }
			}

			@keyframes leptoaster-slide-out-right {
				from { right: 0 }
				to { right: -344px }
			}

			@keyframes leptoaster-progress {
				from { width: 100%; }
				to { width: 0; }
			}
			"
		</style>

		<Show
			when=move || !toaster.top_left_queue.get().is_empty()
		>
			<div
				style:position="fixed"
				style:top="0"
				style:left="0"
				style:z-index="99999"
			>
				<For
					each=toaster.top_left_queue
					key=|toast| toast.id
					let:toast
				>
					<Toast toast={toast} />
				</For>
			</div>
		</Show>

		<Show
			when=move || !toaster.top_right_queue.get().is_empty()
		>
			<div
				style:position="fixed"
				style:top="0"
				style:right="0"
				style:z-index="99999"
			>
				<For
					each=toaster.top_right_queue
					key=|toast| toast.id
					let:toast
				>
					<Toast toast={toast} />
				</For>
			</div>
		</Show>

		<Show
			when=move || !toaster.bottom_right_queue.get().is_empty()
		>
			<div
				style:position="fixed"
				style:bottom="0"
				style:right="0"
				style:z-index="99999"
			>
				<For
					each=toaster.bottom_right_queue
					key=|toast| toast.id
					let:toast
				>
					<Toast toast={toast} />
				</For>
			</div>
		</Show>

		<Show
			when=move || !toaster.bottom_left_queue.get().is_empty()
		>
			<div
				style:position="fixed"
				style:bottom="0"
				style:left="0"
				style:z-index="99999"
			>
				<For
					each=toaster.bottom_left_queue
					key=|toast| toast.id
					let:toast
				>
					<Toast toast={toast} />
				</For>
			</div>
		</Show>
	}
}

pub fn provide_toaster() {
	if use_context::<ToasterContext>().is_none() {
		provide_context(ToasterContext::default());
	}
}

pub fn expect_toaster() -> ToasterContext {
	expect_context::<ToasterContext>()
}
