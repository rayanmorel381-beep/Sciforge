## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| SciForge | `Cargo.toml` | 100% native Rust (edition 2024, zero dependency) |
| HTML dashboard | `demo/` | Static self-contained HTML with integrated results |
| Setup script | `script/setup_sciforge.sh` | Installs Rust + cargo build (desktop only) |
| RenderSink trait | `src/benchmark/simulation.rs` | Pluggable frame hook for concrete renderer |
| SimState | `src/benchmark/simulation.rs` | N-body simulation with positions/velocities `[f64;3]` |
| Framebuffer | planned (`3&5d.md`) | Software RGBA buffer + depth buffer + BMP/PPM export |
| MD→HTML compiler | `presentation.rs` | Custom compiler generating Presentation.html |
| Hub dispatch | `src/hub/` | `Experiment → dispatch() → RunOutput` all domains |

No WASM, no Android NDK, no JNI, no cross-compilation.

## 2️⃣ Android Target Setup

| Task | Description |
|---|---|
| NDK toolchains | aarch64-linux-android, armv7-linux-androideabi, x86_64-linux-android, i686-linux-android |
| `.cargo/config.toml` | Map each target triple to its NDK linker |
| Verify std | Rust tier 2 targets |
| Test build | `cargo build --target aarch64-linux-android --release` → .so |

## 3️⃣ JNI Bridge — `src/android/jni.rs`

| Component | Description |
|---|---|
| Entry points | `#[no_mangle] pub extern "C" fn Java_com_sciforge_Native_*` |
| `init()` | Initialize the library |
| `run_experiment(json_params)` | → json_result |
| `render_frame(width, height)` | → pixel_ptr |
| Type conversion | Rust ↔ JNI via raw pointers (jstring, jbyteArray, jlong) |
| FFI | Manual FFI with std::ffi only (no jni crate) |
| Panic safety | `std::panic::catch_unwind` around each exported fn |

## 4️⃣ Shared Library Build

| Task | Description |
|---|---|
| `Cargo.toml` | `[lib] crate-type = ["cdylib"]` conditional on android target |
| Architectures | arm64-v8a, armeabi-v7a, x86_64, x86 |
| Release profile | `strip = "symbols"`, `lto = true`, `opt-level = "z"` |
| Output | `jniLibs/<abi>/libsciforge.so` — Android project structure |

## 5️⃣ Android Shell Project — `android/`

| File | Description |
|---|---|
| `MainActivity.java` | Main activity |
| `Native.java` | `System.loadLibrary("sciforge")` + native method declarations |
| `SciForgeView.java` | Custom View, Canvas drawing from pixel buffer |
| `activity_main.xml` | Layout |
| `AndroidManifest.xml` | minSdk 26 (Android 8.0), targetSdk 34 |
| `build.gradle.kts` | externalNativeBuild pointing to precompiled .so files |

No C/C++ code — the Rust .so is the only native component.

## 6️⃣ Android Rendering Pipeline

| Component | Description |
|---|---|
| AndroidSink | impl RenderSink, stores Framebuffer, JNI callback pushes pixels to Java |
| Java side | `Bitmap.copyPixelsFromBuffer()` from native RGBA buffer |
| `SciForgeView.onDraw()` | Draws Bitmap on Canvas |
| Frame budget | 30 FPS target (33ms per frame) |
| Fallback | Reduce framebuffer resolution if budget exceeded |

## 7️⃣ Experiment Runner UI

| Component | Description |
|---|---|
| Domain dropdown | Physics, chemistry, astronomy, biology, geology, meteorology |
| Parameter fields | Generated from experiment metadata |
| Run button | `Native.run_experiment(json)` → parse JSON → display |
| Results display | Formatted text + optional ASCII DataTable rendering |
| Share button | Export results as plain text (clipboard or file) |

## 8️⃣ Simulation Viewer

| Component | Description |
|---|---|
| SciForgeView | Full screen for N-body / orbit / projectile simulations |
| Touch controls | Pinch-to-zoom (camera distance), swipe (camera orbit via Quaternion::from_euler) |
| Buttons | Play/pause/step |
| FPS counter | Overlay |
| 3D Pipeline | From `3&5d.md` (Camera, projection, rasterizer) compiled into the .so |

## 9️⃣ APK Packaging Script — `script/build_apk.sh`

| Step | Description |
|---|---|
| Cross-compile | `libsciforge.so` for all 4 ABIs |
| Copy .so | Into `android/app/src/main/jniLibs/<abi>/` |
| Gradle | `assembleRelease` (or assembleDebug) |
| Signing | Debug keystore or keystore provided via environment variable |
| Output | `build/sciforge-<version>.apk` |
| Optional | bundletool for AAB generation |

## 🔟 CI Integration

| Component | Description |
|---|---|
| Workflow | `.github/workflows/android.yml` |
| Matrix | aarch64-linux-android, armv7-linux-androideabi, x86_64-linux-android |
| Steps | Install Rust + NDK → cargo build per target → gradle assembleDebug → upload APK |
| Emulator test | `cargo test --target x86_64-linux-android` (optional) |

## 1️⃣1️⃣ Alternative WebView Path

| Component | Description |
|---|---|
| WASM target | Compile to `wasm32-unknown-unknown` |
| WebView integration | Serve the HTML dashboard in an Android WebView |
| C-ABI exports | Approach without wasm-bindgen, called from JS via WebAssembly.instantiate() |
| Asset reuse | HTML generation from demo/ with results computed by WASM |

## 1️⃣2️⃣ Constraints

| Constraint | Description |
|---|---|
| Dependencies | Zero external Rust dependency (only std, core, alloc) |
| FFI | No jni crate, no ndk crate, no android-activity crate, raw manual FFI |
| APK size | Target < 10 MB per ABI for the .so (strip + LTO + opt-level z) |
| Minimum API | Level 26 (Android 8.0 Oreo) |

## 1️⃣3️⃣ Tests

| Test | Description |
|---|---|
| JNI bridge | Unit tests via cargo test (mock JNIEnv, serialization verification) |
| Cross-compilation | Verify cargo build succeeds for all 4 targets |
| APK | Install on emulator (x86_64), launch, verify valid experiment |
| Rendering | Verify Framebuffer pixels match a known SimState |
| Performance | Benchmark simulation step time on aarch64 (budget 33ms) |
