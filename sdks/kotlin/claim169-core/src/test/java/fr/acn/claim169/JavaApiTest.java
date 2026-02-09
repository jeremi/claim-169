package fr.acn.claim169;

import org.junit.jupiter.api.Test;

import fr.acn.claim169.Claim169Exception;
import fr.acn.claim169.Claim169Data;
import fr.acn.claim169.CwtMetaData;
import fr.acn.claim169.DecodeResultData;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Java integration test that exercises the Claim 169 API from Java to catch
 * real interop issues (lambda syntax, static method access, etc.).
 *
 * All decode/encode calls use the explicit configurer overloads to avoid
 * ambiguity with the Kotlin DSL overloads.
 */
class JavaApiTest {

    // -- Decode with DecoderConfigurer lambda --

    @Test
    void decodeWithDecoderConfigurerLambda() throws Claim169Exception {
        Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
            b.setId("JAVA-DECODE-001");
            b.setFullName("Java Decode Test");
        });
        CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
            b.setIssuer("https://test.example.com");
            b.setExpiresAt(2000000000L);
        });

        String encoded = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
            b.allowUnsigned();
        });

        DecodeResultData result = Claim169.decode(encoded, (DecoderConfigurer) b -> {
            b.allowUnverified();
            b.withoutTimestampValidation();
        });

        assertEquals("JAVA-DECODE-001", result.getClaim169().getId());
        assertEquals("Java Decode Test", result.getClaim169().getFullName());
        assertEquals("skipped", result.getVerificationStatus());
    }

    // -- Build data with Claim169DataConfigurer lambda --

    @Test
    void buildDataWithClaim169DataConfigurerLambda() {
        Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
            b.setId("JAVA-BUILD-001");
            b.setFullName("Java Build Test");
            b.setGenderEnum(Gender.Female);
            b.setEmail("test@example.com");
        });

        assertEquals("JAVA-BUILD-001", data.getId());
        assertEquals("Java Build Test", data.getFullName());
        assertEquals(Gender.Female.getValue(), data.getGender());
        assertEquals("test@example.com", data.getEmail());
    }

    // -- Build CWT metadata with CwtMetaDataConfigurer lambda --

    @Test
    void buildMetaWithCwtMetaDataConfigurerLambda() {
        CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
            b.setIssuer("https://java.example.com");
            b.setExpiresAt(2000000000L);
            b.setIssuedAt(1700000000L);
        });

        assertEquals("https://java.example.com", meta.getIssuer());
        assertEquals(2000000000L, meta.getExpiresAt());
        assertEquals(1700000000L, meta.getIssuedAt());
    }

    // -- Builder.build() is public --

    @Test
    void builderBuildIsPublic() {
        Claim169DataBuilder dataBuilder = new Claim169DataBuilder();
        dataBuilder.setId("JAVA-BUILDER-001");
        dataBuilder.setFullName("Java Builder Test");
        Claim169Data data = dataBuilder.build();

        assertEquals("JAVA-BUILDER-001", data.getId());

        CwtMetaDataBuilder metaBuilder = new CwtMetaDataBuilder();
        metaBuilder.setIssuer("https://builder.example.com");
        metaBuilder.setExpiresAt(2000000000L);
        CwtMetaData meta = metaBuilder.build();

        assertEquals("https://builder.example.com", meta.getIssuer());
    }

    // -- Gender.fromValue() without .Companion --

    @Test
    void genderFromValueWithoutCompanion() {
        assertEquals(Gender.Male, Gender.fromValue(1L));
        assertEquals(Gender.Female, Gender.fromValue(2L));
        assertEquals(Gender.Other, Gender.fromValue(3L));
        assertNull(Gender.fromValue(99L));
    }

    // -- MaritalStatus.fromValue() without .Companion --

    @Test
    void maritalStatusFromValueWithoutCompanion() {
        assertEquals(MaritalStatus.Unmarried, MaritalStatus.fromValue(1L));
        assertEquals(MaritalStatus.Married, MaritalStatus.fromValue(2L));
        assertEquals(MaritalStatus.Divorced, MaritalStatus.fromValue(3L));
        assertNull(MaritalStatus.fromValue(99L));
    }

    // -- PhotoFormat.fromValue() without .Companion --

    @Test
    void photoFormatFromValueWithoutCompanion() {
        assertEquals(PhotoFormat.Jpeg, PhotoFormat.fromValue(1L));
        assertEquals(PhotoFormat.Jpeg2000, PhotoFormat.fromValue(2L));
        assertEquals(PhotoFormat.Avif, PhotoFormat.fromValue(3L));
        assertEquals(PhotoFormat.Webp, PhotoFormat.fromValue(4L));
        assertNull(PhotoFormat.fromValue(99L));
    }

    // -- VerificationStatus.fromValue() without .Companion --

    @Test
    void verificationStatusFromValueWithoutCompanion() {
        assertEquals(VerificationStatus.Verified, VerificationStatus.fromValue("verified"));
        assertEquals(VerificationStatus.Failed, VerificationStatus.fromValue("failed"));
        assertEquals(VerificationStatus.Skipped, VerificationStatus.fromValue("skipped"));
        assertEquals(VerificationStatus.Unknown, VerificationStatus.fromValue("unknown"));
        assertEquals(VerificationStatus.Unknown, VerificationStatus.fromValue("something_else"));
    }

    // -- Claim169.version() as static method --

    @Test
    void versionIsAccessibleAsStaticMethod() {
        String version = Claim169.version();
        assertNotNull(version);
        assertFalse(version.isEmpty());
    }

    // -- Claim169.verificationStatus() static helper --

    @Test
    void verificationStatusStaticHelper() throws Claim169Exception {
        Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
            b.setId("JAVA-STATUS-001");
            b.setFullName("Java Status Test");
        });
        CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
            b.setIssuer("https://test.example.com");
            b.setExpiresAt(2000000000L);
        });

        String encoded = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
            b.allowUnsigned();
        });
        DecodeResultData result = Claim169.decode(encoded, (DecoderConfigurer) b -> {
            b.allowUnverified();
            b.withoutTimestampValidation();
        });

        VerificationStatus status = Claim169.verificationStatus(result);
        assertEquals(VerificationStatus.Skipped, status);
    }

    // -- Error handling with instanceof --

    @Test
    void errorHandlingWithInstanceof() {
        try {
            Claim169.decode("not-valid-base45!!!", (DecoderConfigurer) b -> {
                b.allowUnverified();
            });
            fail("Should have thrown Claim169Exception");
        } catch (Claim169Exception.Base45Decode e) {
            assertNotNull(e.getMessage());
        } catch (Claim169Exception e) {
            // Other exception subtypes are also acceptable
            assertNotNull(e.getMessage());
        }
    }

    // -- try-with-resources with CloseableDecodeResult --

    @Test
    void tryWithResourcesCloseableDecodeResult() throws Claim169Exception {
        Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
            b.setId("JAVA-CLOSEABLE-001");
            b.setFullName("Java Closeable Test");
            b.setPhoto(new byte[]{0x01, 0x02, 0x03});
        });
        CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
            b.setIssuer("https://test.example.com");
            b.setExpiresAt(2000000000L);
        });

        String encoded = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
            b.allowUnsigned();
        });

        byte[] capturedPhoto;

        try (CloseableDecodeResult result = Claim169.decodeCloseable(encoded, (DecoderConfigurer) b -> {
            b.allowUnverified();
            b.withoutTimestampValidation();
        })) {
            assertEquals("JAVA-CLOSEABLE-001", result.getData().getClaim169().getId());
            capturedPhoto = result.getData().getClaim169().getPhoto();
            assertNotNull(capturedPhoto);
        }

        // After close, photo bytes should be zeroed
        for (byte b : capturedPhoto) {
            assertEquals(0, b, "Photo bytes should be zeroed after close");
        }
    }

    // -- Encode with EncoderConfigurer lambda --

    @Test
    void encodeWithEncoderConfigurerLambda() throws Claim169Exception {
        Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
            b.setId("JAVA-ENCODE-001");
            b.setFullName("Java Encode Test");
        });
        CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
            b.setIssuer("https://test.example.com");
            b.setExpiresAt(2000000000L);
        });

        String encoded = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
            b.allowUnsigned();
        });
        assertNotNull(encoded);
        assertFalse(encoded.isEmpty());

        // Verify roundtrip
        DecodeResultData result = Claim169.decode(encoded, (DecoderConfigurer) b -> {
            b.allowUnverified();
            b.withoutTimestampValidation();
        });
        assertEquals("JAVA-ENCODE-001", result.getClaim169().getId());
    }

    // -- Full roundtrip from Java --

    @Test
    void fullRoundtripFromJava() throws Claim169Exception {
        // Build data
        Claim169Data data = Claim169.claim169((Claim169DataConfigurer) b -> {
            b.setId("JAVA-ROUNDTRIP-001");
            b.setFullName("Jane Java");
            b.setDateOfBirth("1990-05-15");
            b.setGenderEnum(Gender.Female);
            b.setEmail("jane@example.com");
            b.setNationality("US");
            b.setMaritalStatusEnum(MaritalStatus.Unmarried);
        });

        // Build metadata
        CwtMetaData meta = Claim169.cwtMeta((CwtMetaDataConfigurer) b -> {
            b.setIssuer("https://java.example.com");
            b.setExpiresAt(2000000000L);
            b.setIssuedAt(1700000000L);
        });

        // Encode
        String qrData = Claim169.encode(data, meta, (EncoderConfigurer) b -> {
            b.allowUnsigned();
        });
        assertNotNull(qrData);
        assertFalse(qrData.isEmpty());

        // Decode
        DecodeResultData result = Claim169.decode(qrData, (DecoderConfigurer) b -> {
            b.allowUnverified();
            b.withoutTimestampValidation();
        });

        // Verify fields
        assertEquals("JAVA-ROUNDTRIP-001", result.getClaim169().getId());
        assertEquals("Jane Java", result.getClaim169().getFullName());
        assertEquals("1990-05-15", result.getClaim169().getDateOfBirth());
        assertEquals(Gender.Female.getValue(), result.getClaim169().getGender());
        assertEquals("jane@example.com", result.getClaim169().getEmail());
        assertEquals("US", result.getClaim169().getNationality());
        assertEquals(MaritalStatus.Unmarried.getValue(), result.getClaim169().getMaritalStatus());

        // Verify metadata
        assertEquals("https://java.example.com", result.getCwtMeta().getIssuer());
        assertEquals(2000000000L, result.getCwtMeta().getExpiresAt());

        // Verify verification status
        VerificationStatus status = Claim169.verificationStatus(result);
        assertEquals(VerificationStatus.Skipped, status);
    }
}
