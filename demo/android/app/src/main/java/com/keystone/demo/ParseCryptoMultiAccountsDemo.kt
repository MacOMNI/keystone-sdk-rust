package com.keystone.demo

import com.keystone.sdk.KeystoneNativeSDK
import org.json.JSONArray
import org.json.JSONObject

object ParseCryptoMultiAccountsDemo {
    private const val TARGET_GROUP_COUNT = 10
    private const val MIN_SUCCESS_GROUP_COUNT = 8
    private const val SOURCE_PART_START = 2

    private data class CandidateGroup(
        val label: String,
        val parts: List<String>
    )

    @JvmStatic
    fun run(): String {
        val groups = candidateGroups(CryptoMultiAccountsLogSamples.parts)
        val details = mutableListOf<String>()
        var success = 0
        var tested = 0

        for ((index, group) in groups.withIndex()) {
            if (tested >= TARGET_GROUP_COUNT) {
                break
            }
            tested += 1

            val decodeInput = JSONArray(group.parts).toString()
            val decodedJson = KeystoneNativeSDK.decodeUrToCborHex(decodeInput)
            val decoded = JSONObject(decodedJson)
            if (decoded.has("error")) {
                details += "Group ${index + 1} (${group.label}) [${group.parts.size} parts]: decode error=${decoded.optString("error")}"
                continue
            }

            val urType = decoded.optString("type")
            val cborHex = decoded.optString("cbor")
            if (urType.isBlank() || cborHex.isBlank()) {
                details += "Group ${index + 1} (${group.label}) [${group.parts.size} parts]: decode payload invalid"
                continue
            }

            val parsedJson = KeystoneNativeSDK.parseCryptoMultiAccounts(urType, cborHex)
            val parsed = JSONObject(parsedJson)
            if (parsed.has("error")) {
                details += "Group ${index + 1} (${group.label}) [${group.parts.size} parts]: parse error=${parsed.optString("error")}"
                continue
            }

            success += 1
            val keyCount = parsed.optJSONArray("keys")?.length() ?: 0
            val fingerprint = parsed.optString("master_fingerprint", "")
            details += "Group ${index + 1} (${group.label}) [${group.parts.size} parts]: OK type=$urType cbor_len=${cborHex.length} keys=$keyCount xfp=$fingerprint"
        }

        val status = if (success >= MIN_SUCCESS_GROUP_COUNT) "PASS" else "FAIL"
        return buildString {
            appendLine("Batch multipart decode+parse: $status")
            appendLine("success=$success tested=$tested target=$TARGET_GROUP_COUNT min_success=$MIN_SUCCESS_GROUP_COUNT")
            appendLine()
            details.forEach { appendLine(it) }
        }
    }

    private fun candidateGroups(parts: List<String>): List<CandidateGroup> {
        val groups = mutableListOf<CandidateGroup>()
        for (window in 12..16) {
            if (window > parts.size) {
                continue
            }
            for (start in 0..(parts.size - window)) {
                val partStart = SOURCE_PART_START + start
                val partEnd = partStart + window - 1
                groups += CandidateGroup(
                    label = "parts $partStart-$partEnd",
                    parts = parts.subList(start, start + window)
                )
            }
        }
        groups += CandidateGroup(
            label = "even-index-pick",
            parts = parts.filterIndexed { index, _ -> index % 2 == 0 }
        )
        groups += CandidateGroup(
            label = "odd-index-pick",
            parts = parts.filterIndexed { index, _ -> index % 2 == 1 }
        )
        return groups
    }
}
