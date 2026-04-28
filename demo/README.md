# FFI Demo: `parse_crypto_multi_accounts`

This folder contains Android and iOS demos that call:

- `parse_crypto_multi_accounts("crypto-multi-accounts", cbor_hex)`

Sample input (`cbor_hex`):

```text
a5011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65047828323834373563386438306636633036626166626534366137643137353066336663663235363566370565312e302e30
```

Expected result:

```json
{"device":"keystone","device_id":"28475c8d80f6c06bafbe46a7d1750f3fcf2565f7","device_version":"1.0.0","keys":[{"chain":"SOL","chain_code":"","extended_public_key":"","extra":{"okx":{"chain_id":501}},"name":"","path":"m/44'/501'/0'/0'","public_key":"02eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b"}],"master_fingerprint":"e9181cf3"}
```

See platform guides:

- [Android Demo](./android/README.md)
- [iOS Demo](./ios/README.md)

iOS quick start:

```bash
cd /Users/mackun/keystone-sdk-rust/demo/ios
./prepare_demo.sh
open KeystoneFFIDemo.xcodeproj
```
