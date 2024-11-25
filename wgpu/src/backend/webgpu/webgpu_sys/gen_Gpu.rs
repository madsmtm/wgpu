// DO NOT EDIT THIS FILE!
//
// This module part of a subset of web-sys that is used by wgpu's webgpu backend.
//
// These bindings are vendored into wgpu for the sole purpose of letting
// us pin the WebGPU backend to a specific version of the bindings, not
// to enable local changes. There are no provisions to preserve changes
// you make here the next time we re-vendor the bindings.
//
// The `web-sys` crate does not treat breaking changes to the WebGPU API
// as semver breaking changes, as WebGPU is "unstable". This means Cargo
// will not let us mix versions of `web-sys`, pinning WebGPU bindings to
// a specific version, while letting other bindings like WebGL get
// updated. Vendoring WebGPU was the workaround we chose.
//
// Vendoring also allows us to avoid building `web-sys` with
// `--cfg=web_sys_unstable_apis`, needed to get the WebGPU bindings.
//
// If you want to improve the generated code, please submit a PR to the https://github.com/rustwasm/wasm-bindgen repository.
//
// This file was generated by the `cargo xtask vendor-web-sys --version 0.2.91` command.
#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPU , typescript_type = "GPU")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Gpu` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type Gpu;

    # [wasm_bindgen (structural , method , getter , js_class = "GPU" , js_name = wgslLanguageFeatures)]
    #[doc = "Getter for the `wgslLanguageFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/wgslLanguageFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn wgsl_language_features(this: &Gpu) -> WgslLanguageFeatures;

    # [wasm_bindgen (method , structural , js_class = "GPU" , js_name = getPreferredCanvasFormat)]
    #[doc = "The `getPreferredCanvasFormat()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/getPreferredCanvasFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_preferred_canvas_format(this: &Gpu) -> GpuTextureFormat;

    # [wasm_bindgen (method , structural , js_class = "GPU" , js_name = requestAdapter)]
    #[doc = "The `requestAdapter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_adapter(this: &Gpu) -> ::js_sys::Promise;

    # [wasm_bindgen (method , structural , js_class = "GPU" , js_name = requestAdapter)]
    #[doc = "The `requestAdapter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn request_adapter_with_options(
        this: &Gpu,
        options: &GpuRequestAdapterOptions,
    ) -> ::js_sys::Promise;
}
