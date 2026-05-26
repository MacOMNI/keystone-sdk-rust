# Android Demo (Batch Multi-Part Verify)

This demo runs batch verification with sample UR multipart data:

- `Java_com_keystone_sdk_KeystoneNativeSDK_decodeUrToCborHex`
- `Java_com_keystone_sdk_KeystoneNativeSDK_parseCryptoMultiAccounts`

In Java/Kotlin it maps to:

- `com.keystone.sdk.KeystoneNativeSDK.decodeUrToCborHex(String urInput)`
- `com.keystone.sdk.KeystoneNativeSDK.parseCryptoMultiAccounts(String urType, String cborHex)`

Sample pool file:

- `demo/fixtures/crypto_multi_accounts_parts_2_23.txt`

## 1) Build Android `.so`

Run in repo root:

```bash
ANDROID_NDK_HOME="$HOME/Library/Android/sdk/ndk/28.2.13676358" make android
```

## 2) Copy `.so` files into demo

Put files under `app/src/main/jniLibs`:

- `arm64-v8a/libur_registry_ffi.so` from `target/aarch64-linux-android/release/libur_registry_ffi.so`
- `armeabi-v7a/libur_registry_ffi.so` from `target/armv7-linux-androideabi/release/libur_registry_ffi.so`
- `x86/libur_registry_ffi.so` from `target/i686-linux-android/release/libur_registry_ffi.so`
- `x86_64/libur_registry_ffi.so` from `target/x86_64-linux-android/release/libur_registry_ffi.so`

## 3) Run

Open `demo/android` in Android Studio and run the app.

CLI compile check (same command used for local verification):

```bash
cd /Users/mackun/keystone-sdk-rust/demo/android
GRADLE_USER_HOME=/Users/mackun/keystone-sdk-rust/.gradle-home gradle :app:assembleDebug
```

The launcher page prints per-group results:

- decode status
- parse status
- summary (`success/tested`)
