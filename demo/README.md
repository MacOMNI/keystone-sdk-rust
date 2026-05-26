# FFI Demo: Batch Multi-Part Verify (`decode_ur_to_cbor_hex` + `parse_crypto_multi_accounts`)

This folder contains Android and iOS demos that do batch verification:

- extract candidate multipart groups from sample UR parts
- call `decode_ur_to_cbor_hex(ur_input)` for each group
- call `parse_crypto_multi_accounts(ur_type, cbor_hex)` for each decoded group

Sample pool source:

```text
demo/fixtures/crypto_multi_accounts_parts_2_23.txt
demo/fixtures/crypto_multi_accounts_group_plan.txt
```

Batch target:

```text
10 groups tested, at least 8 groups must pass decode+parse
```

Sample grouping strategy:

```text
- source pool: 22 parts (UR:.../2-5 to UR:.../23-5)
- candidate groups: sliding windows (12..16) + even/odd index groups
- demo execution: run first 10 candidate groups, print per-group details
- rust verification: validate candidate groups, require >= 8 successful decode+parse
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
