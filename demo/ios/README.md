# iOS Demo (`parse_crypto_multi_accounts`)

This folder is now a complete runnable Xcode iOS app project:

- `KeystoneFFIDemo.xcodeproj`

## Run Directly In Xcode

1. Ensure XCFramework exists:

```bash
cd /Users/mackun/keystone-sdk-rust
make generate_xcframework
```

2. Open project:

- `/Users/mackun/keystone-sdk-rust/demo/ios/KeystoneFFIDemo.xcodeproj`

3. Select target `KeystoneFFIDemo`, pick an iOS Simulator, then Run.

App launch page has a **Parse** button. Tap it to call:

- `parse_crypto_multi_accounts("crypto-multi-accounts", cbor_hex)`

and display returned JSON.

## Notes

- The linked SDK path is `../../target/URRegistryFFI.xcframework`.
- Demo source already includes:
  - fixed `ur_type = "crypto-multi-accounts"`
  - your full sample CBOR hex
  - safe release of Rust-allocated strings via `keystone_sdk_destroy_string`
