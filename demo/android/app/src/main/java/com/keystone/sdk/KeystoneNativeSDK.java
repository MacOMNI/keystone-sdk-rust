package com.keystone.sdk;

public final class KeystoneNativeSDK {
    static {
        System.loadLibrary("ur_registry_ffi");
    }

    private KeystoneNativeSDK() {
    }

    public static native String parseCryptoMultiAccounts(String urType, String cborHex);
}

