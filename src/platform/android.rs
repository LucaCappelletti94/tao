// Copyright 2014-2021 The winit contributors
// Copyright 2021-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

#![cfg(target_os = "android")]

pub mod prelude {
  pub use crate::platform_impl::ndk_glue::*;
  pub use tao_macros::{android_fn, generate_package_name};
}
use crate::{
  event::Event,
  event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
  platform_impl::ndk_glue::Rect,
  window::{Window, WindowBuilder},
};
use ndk::configuration::Configuration;

/// Additional methods on `EventLoop` that are specific to Android.
pub trait EventLoopExtAndroid {
  /// A type provided by the user that can be passed through `Event::UserEvent`.
  type UserEvent;

  /// Runs the event loop from callbacks on the calling thread's looper and returns at once, so the UI thread can run it where [`EventLoop::run`] would block, and `ControlFlow::ExitWithCode` exits the process as `run` does.
  fn spawn<F>(self, event_handler: F)
  where
    F: 'static
      + FnMut(Event<'_, Self::UserEvent>, &EventLoopWindowTarget<Self::UserEvent>, &mut ControlFlow);
}

impl<T> EventLoopExtAndroid for EventLoop<T> {
  type UserEvent = T;

  fn spawn<F>(self, event_handler: F)
  where
    F: 'static
      + FnMut(Event<'_, Self::UserEvent>, &EventLoopWindowTarget<Self::UserEvent>, &mut ControlFlow),
  {
    self.event_loop.spawn(event_handler)
  }
}

/// Additional methods on `EventLoopWindowTarget` that are specific to Android.
pub trait EventLoopWindowTargetExtAndroid {}

/// Additional methods on `Window` that are specific to Android.
pub trait WindowExtAndroid {
  fn content_rect(&self) -> Rect;

  fn config(&self) -> Configuration;

  fn activity_name(&self) -> String;
}

impl WindowExtAndroid for Window {
  fn content_rect(&self) -> Rect {
    self.window.content_rect()
  }

  fn config(&self) -> Configuration {
    self.window.config()
  }

  fn activity_name(&self) -> String {
    self.window.activity_name().to_string()
  }
}

impl<T> EventLoopWindowTargetExtAndroid for EventLoopWindowTarget<T> {}

/// Additional methods on `WindowBuilder` that are specific to Android.
pub trait WindowBuilderExtAndroid {
  /// The name of the activity class to create.
  fn with_activity_name(self, activity_name: String) -> Self;

  /// The name of the activity class that created this window.
  ///
  /// This is important to define which stack the activity will be created on.
  fn with_created_by_activity_name(self, created_by_activity_name: String) -> Self;
}

impl WindowBuilderExtAndroid for WindowBuilder {
  fn with_activity_name(mut self, activity_name: String) -> Self {
    self.platform_specific.activity_name = Some(activity_name);
    self
  }

  fn with_created_by_activity_name(mut self, created_by_activity_name: String) -> Self {
    self.platform_specific.created_by_activity_name = Some(created_by_activity_name);
    self
  }
}
