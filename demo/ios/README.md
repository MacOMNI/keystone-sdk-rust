# iOS Demo (Batch Multi-Part Verify)

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

CLI compile check (same command used for local verification):

```bash
cd /Users/mackun/keystone-sdk-rust/demo/ios
xcodebuild -project KeystoneFFIDemo.xcodeproj \
  -scheme KeystoneFFIDemo \
  -configuration Debug \
  -derivedDataPath ../../.xcode-derived \
  -destination 'generic/platform=iOS Simulator' \
  CODE_SIGNING_ALLOWED=NO build
```

App launch page has a **Batch Verify** button. Tap it to run:

- candidate multipart groups from sample pool
- `decode_ur_to_cbor_hex(ur_input)`
- `parse_crypto_multi_accounts(ur_type, cbor_hex)`

and display per-group details plus overall pass/fail summary.

Sample pool file:

- `demo/fixtures/crypto_multi_accounts_parts_2_23.txt`

## Notes

- The linked SDK path is `../../target/URRegistryFFI.xcframework`.
- Demo source already includes:
  - sample pool from your log (`CryptoMultiAccountsLogSamples.parts`)
  - safe release of Rust-allocated strings via `keystone_sdk_destroy_string`
