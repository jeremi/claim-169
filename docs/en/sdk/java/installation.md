# Installation

## Requirements

- **JDK 17+** --- The SDK requires Java 17 or later
- **Android API 24+** --- For Android targets (Android 7.0 Nougat and above)
- **JNA** --- Included as a transitive dependency

## Installing with Gradle (Kotlin DSL)

```kotlin
dependencies {
    implementation("fr.acn.claim169:claim169-core:0.2.0-alpha")
}
```

## Installing with Gradle (Groovy DSL)

```groovy
dependencies {
    implementation 'fr.acn.claim169:claim169-core:0.2.0-alpha'
}
```

## Installing with Maven

```xml
<dependency>
    <groupId>fr.acn.claim169</groupId>
    <artifactId>claim169-core</artifactId>
    <version>0.2.0-alpha</version>
</dependency>
```

## Development Installation

For contributing or building from source:

```bash
# Clone the repository
git clone https://github.com/jeremi/claim-169.git
cd claim-169

# Build the Rust native library
cargo build --release

# Run tests (requires native library on library path)
cd sdks/kotlin
./gradlew test
```

### Building the Native Library

The SDK depends on a native library built from the Rust core. The build produces platform-specific shared libraries:

```bash
# Build for current platform
cargo build --release

# The output is in target/release/
# Linux:  libclaim169_core.so
# macOS:  libclaim169_core.dylib
# Windows: claim169_core.dll
```

## Platform Support

Pre-built native libraries are bundled in the JAR for supported platforms:

| Platform | Architecture | Native Library |
|----------|--------------|----------------|
| Linux | x86_64 | `libclaim169_core.so` |
| Linux | aarch64 | `libclaim169_core.so` |
| macOS | x86_64 | `libclaim169_core.dylib` |
| macOS | aarch64 (Apple Silicon) | `libclaim169_core.dylib` |
| Windows | x86_64 | `claim169_core.dll` |
| Android | arm64-v8a | `libclaim169_core.so` |
| Android | armeabi-v7a | `libclaim169_core.so` |
| Android | x86_64 | `libclaim169_core.so` |

## Native Library Loading

The SDK uses JNA to load the native library. It searches in this order:

1. **Bundled in JAR** --- The published artifact includes native libraries for all supported platforms
2. **`java.library.path`** --- System property pointing to the native library directory
3. **`jna.library.path`** --- JNA-specific library search path

### Custom Library Path

If the native library is not bundled or you need to override it:

=== "System Property"

    ```bash
    java -Djava.library.path=/path/to/native/libs -jar your-app.jar
    ```

=== "Gradle Task"

    ```kotlin
    tasks.test {
        systemProperty("java.library.path", "${rootProject.rootDir}/../target/release")
    }
    ```

=== "Environment Variable"

    ```bash
    export JNA_LIBRARY_PATH=/path/to/native/libs
    java -jar your-app.jar
    ```

### Android Native Library Loading

For Android, place native libraries in the `jniLibs` directory:

```
src/main/jniLibs/
    arm64-v8a/libclaim169_core.so
    armeabi-v7a/libclaim169_core.so
    x86_64/libclaim169_core.so
```

Or use the published AAR which includes all supported ABIs.

## Verifying Installation

```java
import fr.acn.claim169.Claim169;

public class Main {
    public static void main(String[] args) {
        System.out.println("claim169 version: " + Claim169.version());
    }
}
```

Output:

```
claim169 version: 0.2.0-alpha
```

## Upgrading

Update the version in your build file:

=== "Gradle Kotlin DSL"

    ```kotlin
    dependencies {
        implementation("fr.acn.claim169:claim169-core:0.2.0")
    }
    ```

=== "Gradle Groovy DSL"

    ```groovy
    dependencies {
        implementation 'fr.acn.claim169:claim169-core:0.2.0'
    }
    ```

=== "Maven"

    ```xml
    <dependency>
        <groupId>fr.acn.claim169</groupId>
        <artifactId>claim169-core</artifactId>
        <version>0.2.0</version>
    </dependency>
    ```

## Troubleshooting Installation

### UnsatisfiedLinkError

```
java.lang.UnsatisfiedLinkError: Unable to load library 'claim169_core'
```

1. Verify the native library exists for your platform
2. Check `java.library.path`:
   ```java
   System.out.println(System.getProperty("java.library.path"));
   ```
3. Set the library path explicitly:
   ```bash
   java -Djava.library.path=/path/to/libs -jar your-app.jar
   ```

### JNA Not Found

```
java.lang.NoClassDefFoundError: com/sun/jna/Library
```

JNA should be included as a transitive dependency. If missing, add it explicitly:

=== "Gradle Kotlin DSL"

    ```kotlin
    dependencies {
        implementation("net.java.dev.jna:jna:5.14.0")
    }
    ```

=== "Maven"

    ```xml
    <dependency>
        <groupId>net.java.dev.jna</groupId>
        <artifactId>jna</artifactId>
        <version>5.14.0</version>
    </dependency>
    ```

### Version Mismatch

If `Claim169.version()` returns an unexpected version, clear your dependency cache:

```bash
# Gradle
./gradlew --refresh-dependencies

# Maven
mvn dependency:purge-local-repository -DactTransitively=false
```
