package com.keystone.demo

import com.keystone.sdk.KeystoneNativeSDK

object ParseCryptoMultiAccountsDemo {
    private const val UR_TYPE = "crypto-multi-accounts"
    private const val CBOR_HEX =
        "a5011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65047828323834373563386438306636633036626166626534366137643137353066336663663235363566370565312e302e30"

    @JvmStatic
    fun run(): String {
        return KeystoneNativeSDK.parseCryptoMultiAccounts(UR_TYPE, CBOR_HEX)
    }
}

