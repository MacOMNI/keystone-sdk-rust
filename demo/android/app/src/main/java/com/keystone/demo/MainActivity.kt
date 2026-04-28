package com.keystone.demo

import android.os.Bundle
import android.widget.ScrollView
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val output = try {
            ParseCryptoMultiAccountsDemo.run()
        } catch (t: Throwable) {
            "JNI call failed: ${t.message}"
        }

        val textView = TextView(this).apply {
            textSize = 14f
            setTextIsSelectable(true)
            text = output
            setPadding(24, 24, 24, 24)
        }

        setContentView(ScrollView(this).apply { addView(textView) })
    }
}

