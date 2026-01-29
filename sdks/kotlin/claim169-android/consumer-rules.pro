# Claim 169 SDK ProGuard rules for consumers

# Keep UniFFI generated bindings
-keep class uniffi.claim169_jni.** { *; }

# Keep Kotlin DSL wrapper classes
-keep class org.acn.claim169.** { *; }

# Keep JNA classes used by UniFFI
-keep class com.sun.jna.** { *; }
-dontwarn com.sun.jna.**

# Keep callback interfaces (used for HSM/KMS integration)
-keep interface org.acn.claim169.SignatureVerifier { *; }
-keep interface org.acn.claim169.Decryptor { *; }
-keep interface org.acn.claim169.Signer { *; }
-keep interface org.acn.claim169.Encryptor { *; }
