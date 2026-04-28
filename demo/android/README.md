# Android Demo (`parseCryptoMultiAccounts`)

This demo calls the JNI method exported by this Rust SDK:

- `Java_com_keystone_sdk_KeystoneNativeSDK_parseCryptoMultiAccounts`

In Java/Kotlin it maps to:

- `com.keystone.sdk.KeystoneNativeSDK.parseCryptoMultiAccounts(String urType, String cborHex)`

## 1) Build Android `.so`

Run in repo root:

```bash
make android
```

## 2) Copy `.so` files into demo

Put files under `app/src/main/jniLibs`:

- `arm64-v8a/libur_registry_ffi.so` from `target/aarch64-linux-android/release/libur_registry_ffi.so`
- `armeabi-v7a/libur_registry_ffi.so` from `target/armv7-linux-androideabi/release/libur_registry_ffi.so`
- `x86/libur_registry_ffi.so` from `target/i686-linux-android/release/libur_registry_ffi.so`
- `x86_64/libur_registry_ffi.so` from `target/x86_64-linux-android/release/libur_registry_ffi.so`

## 3) Run

Open `demo/android` in Android Studio and run the app.

The launcher page prints parsed JSON for the sample CBOR.

