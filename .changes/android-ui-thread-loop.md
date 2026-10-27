---
"tao": minor
---

On Android, run the event loop on the UI thread. `onFirstActivityCreate` now calls the `main` passed to `android_binding!` on the UI thread instead of on a spawned thread, so `main` must return and starts the event loop with the new `EventLoopExtAndroid::spawn`, which drives it from callbacks on the UI thread's looper. Calling `EventLoop::run` there panics because the UI thread's looper rejects the fds `run_return` polls. `WindowBuilder::with_activity_name` no longer waits for the new activity's `onCreate`, and `WindowEvent::Started` reports the activity once it exists.
