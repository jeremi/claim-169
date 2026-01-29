# 169 - QR Code Specifications

## CBOR Identity Data in QR Code

**Tag**: 169 (identity-data)

**Data Item**: JSON Object

**Semantics**: Identity Data of a Person in QR-Code

**Point of Contact**: Resham Chugani ([resham@mosip.io](mailto:resham@mosip.io))

**IANA Registration**: [IANA CWT Registry](https://www.iana.org/assignments/cwt/cwt.xhtml) (Search for: 169)

**Version:** 1.2.0

## 1. Introduction

This document specifies an enhanced version of the generic data structure and encoding mechanism for storing the Identity Data of a registered person using any ID platform, along with the corresponding transport encoding mechanism in a machine-readable optical format (QR).

This enhanced version is the outcome of the revival of the Claim 169 Working Group in September 2025, which undertook a collaborative effort to refine and extend the specification. As part of the detailed discussions and brainstorming sessions within the working group, **additional attributes (19–23)** were introduced to strengthen applicability, usability, and interoperability across diverse identity ecosystems; bring in multi-language support (for _Full Name_), along with certain updates on [guidelines](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#guidelines), [standard CWT attributes](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-standard-cwt-attributes), [standard COSE attributes](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-standard-cose-attributes) (for public key discovery), [credential status](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-status-of-credential) and [security considerations](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#id-4.-security-considerations). **For details, refer to the section titled, "**[**What Changed**](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#id-8.-what-changed)**" below.**

Further details on the evolution of these changes and detailed discussions can be found under the section titled "Iteration 2" [here](https://mosip.atlassian.net/wiki/external/OWExMjBhYzQ3Mjk1NGZlOWExMWEzODA2YzVjYjExNmQ).

## 2. Rationale

Once a person is registered in an identity system, their data serves as the foundation for identification, granting them access to social benefits and government services. The level of assurance in this identification process varies depending on the authentication methods employed. Low assurance is achieved through basic identifiers like ID numbers, demographic data, passwords, or PINs. Conversely, higher assurance levels are attained through one-time passwords (OTP) and biometrics.

Among these methods, biometric-based authentication, such as facial authentication, offers the highest level of assurance as it assures the presence of the individual. While this is effective for online systems & personal phones where verification is conducted on a server or a personal device; offline authentication presents challenges in maintaining a similarly high level of assurance. The offline authentication mechanism should work for people with no phone.

For instance, in a cross-border scenario remote areas often face significant internet connectivity issues. Even when internet access is available, server reliability may be inconsistent. In such circumstances, scanning a QR code containing the person's facial photograph and identity information, alongside assurance that the data is signed by an authorized issuing authority or other trusted source (e.g. Country/state/others), provides an additional layer of security and affirmation for the countries and/or entities involved.

**Please note:** The trust layers required to sync the country's keys are beyond the scope of this document. We assume the app scanning the QR code already has the country's key to verify.

To tackle the challenge above, we propose a standard CBOR-based QR Code that involves embedding a low-resolution image of the person with a minimal demographic dataset within the QR code. This QR code would be digitally signed by the ID authorities (Issuer) and then printed on a physical card. Subsequently, the signed data within the QR code can be utilized for facial authentication. However, it's essential to recognize that QR codes have limitations regarding size. We suggest leveraging CBOR Web Token (CWT) with ED25519/ECC keys to generate a smaller signature and more condensed data.

Claim 169 represents a JSON Object that includes the below table as ID attributes. You can find an illustration of the ID structure contained within Claim 169, where:

## 3. Semantics

### 3.1 CBOR Map Structure Overview

**Note**:

* All the fields here are optional.
* The issuer of **ID Claim169** is expected to host the **JWKS** file at the standard **.well-known** URL. This allows relying parties to verify the signature of the issued IDClaim169.
* Please ensure to review the [Guidelines](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#guidelines) and important note(s) below with respect to [standard CWT attributes](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-standard-cwt-attributes) and [credential status](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-status-of-credential).

<table><thead><tr><th width="155">Attribute</th><th width="169">Type</th><th width="193">Attribute Name</th><th>Description</th></tr></thead><tbody><tr><td><code>1</code></td><td><code>tstr</code></td><td>ID</td><td>Unique ID to indicate the PII data</td></tr><tr><td><code>2</code></td><td><code>tstr</code></td><td>Version</td><td>Version of the ID data</td></tr><tr><td><code>3</code></td><td><code>tstr</code></td><td>Language</td><td>Language used in other attributes: Use the three-letter <a href="https://en.wikipedia.org/wiki/ISO_639-3">ISO 639-3</a> language code</td></tr><tr><td><code>4</code></td><td><code>tstr</code></td><td>Full Name</td><td>Full name of the person</td></tr><tr><td><code>5</code></td><td><code>tstr</code></td><td>First Name</td><td>First name of the person</td></tr><tr><td><code>6</code></td><td><code>tstr</code></td><td>Middle Name</td><td>Middle name of the person</td></tr><tr><td><code>7</code></td><td><code>tstr</code></td><td>Last Name</td><td>Last name of the person</td></tr><tr><td><code>8</code></td><td><code>tstr</code></td><td>Date of Birth</td><td>Date of birth in YYYYMMDD format</td></tr><tr><td><code>9</code></td><td><code>int</code></td><td>Gender</td><td>Gender with the following values <code>1</code> - Male, <code>2</code> - Female, <code>3</code> - Others</td></tr><tr><td><code>10</code></td><td><code>tstr</code></td><td>Address</td><td>Address of the person, separator character <code>\n</code></td></tr><tr><td><code>11</code></td><td><code>tstr</code></td><td>Email ID</td><td>Email id of the person</td></tr><tr><td><code>12</code></td><td><code>tstr</code></td><td>Phone Number</td><td>Contact number of the person: Use <a href="https://en.wikipedia.org/wiki/E.123">E.123</a> international notation</td></tr><tr><td><code>13</code></td><td><code>tstr</code></td><td>Nationality</td><td>Nationality of the person:<br>Use the two-letter <a href="https://en.wikipedia.org/wiki/ISO_3166-2">ISO 3166-2</a> country code or three-letter <a href="https://en.wikipedia.org/wiki/ISO_3166-1_alpha-3">ISO 3166-1 alpha-3</a> country code</td></tr><tr><td><code>14</code></td><td><code>int</code></td><td>Marital Status</td><td>Marital status - Can contain the following values <code>1</code> - Unmarried, <code>2</code> - Married, <code>3</code> - Divorced</td></tr><tr><td><code>15</code></td><td><code>tstr</code></td><td>Guardian</td><td>Name/id of the entity playing the role of a guardian, such as a mother, father, spouse, sister, legal guardian etc.</td></tr><tr><td><code>16</code></td><td><code>tstr</code></td><td>Binary Image</td><td>Binary image of the person's photograph</td></tr><tr><td><code>17</code></td><td><code>int</code></td><td>Binary Image Format</td><td>Binary image format. Can contain the following values <code>1</code> - JPEG, <code>2</code> - JPEG2, <code>3</code> - AVIF, <code>4</code> - WEBP</td></tr><tr><td><code>18</code></td><td><code>[int]</code></td><td>Best Quality Fingers</td><td>An unsigned 8-bit number encoding the hand position of the finger. It must be in the range 0-10, where 0 represents "Unknown", 1-5 represents right thumb to little finger, and 6-10 represents left thumb to little finger in sequence</td></tr><tr><td><code>19</code></td><td><code>tstr</code></td><td>Full Name - Secondary Language</td><td>Secondary Language Identity Full Name</td></tr><tr><td><code>20</code></td><td><code>tstr</code></td><td>Secondary Language</td><td>Secondary Language Code. Language used in other attributes: Use the three-letter ISO 639-3 language code</td></tr><tr><td><code>21</code></td><td><code>tstr</code></td><td>Location Code</td><td>Geo Location/Code</td></tr><tr><td><code>22</code></td><td><code>tstr</code></td><td>Legal Status</td><td>Legal Status of the identity</td></tr><tr><td><code>23</code></td><td><code>tstr</code></td><td>Country of Issuance</td><td>Country of Issuance</td></tr><tr><td><code>24.. 49</code></td><td></td><td>Unassigned</td><td>For future - For Demographic Data attributes</td></tr><tr><td><code>50</code></td><td><code>[Biometrics]</code></td><td>Right Thumb</td><td>Person's Right Thumb biometrics</td></tr><tr><td><code>51</code></td><td><code>[Biometrics]</code></td><td>Right Pointer Finger</td><td>Person's Right Pointer Finger biometrics</td></tr><tr><td><code>52</code></td><td><code>[Biometrics]</code></td><td>Right Middle Finger</td><td>Person's Right Middle Finger biometrics</td></tr><tr><td><code>53</code></td><td><code>[Biometrics]</code></td><td>Right Ring Finger</td><td>Person's Right Ring Finger biometrics</td></tr><tr><td><code>54</code></td><td><code>[Biometrics]</code></td><td>Right Little Finger</td><td>Person's Right Little Finger biometrics</td></tr><tr><td><code>55</code></td><td><code>[Biometrics]</code></td><td>Left Thumb</td><td>Person's Left Thumb biometrics</td></tr><tr><td><code>56</code></td><td><code>[Biometrics]</code></td><td>Left Pointer Finger</td><td>Person's Left Pointer Finger biometrics</td></tr><tr><td><code>57</code></td><td><code>[Biometrics]</code></td><td>Left Middle Finger</td><td>Person's Left Middle Finger biometrics</td></tr><tr><td><code>58</code></td><td><code>[Biometrics]</code></td><td>Left Ring Finger</td><td>Person's Left Ring Finger biometrics</td></tr><tr><td><code>59</code></td><td><code>[Biometrics]</code></td><td>Left Little Finger</td><td>Person's Left Little Finger biometrics</td></tr><tr><td><code>60</code></td><td><code>[Biometrics]</code></td><td>Right Iris</td><td>Person's Right Iris biometrics</td></tr><tr><td><code>61</code></td><td><code>[Biometrics]</code></td><td>Left Iris</td><td>Person's Left Iris biometrics</td></tr><tr><td><code>62</code></td><td><code>[Biometrics]</code></td><td>Face</td><td>Person's Face biometrics</td></tr><tr><td><code>63</code></td><td><code>[Biometrics]</code></td><td>Right Palm Print</td><td>Person's Right Palm Print biometrics</td></tr><tr><td><code>64</code></td><td><code>[Biometrics]</code></td><td>Left Palm Print</td><td>Person's Left Palm Print biometrics</td></tr><tr><td><code>65</code></td><td><code>[Biometrics]</code></td><td>Voice</td><td>Person's Voice biometrics</td></tr><tr><td><code>66.. 74</code></td><td></td><td>Unassigned</td><td>For future - For Biometrics Data attributes</td></tr><tr><td><code>75.. 99</code></td><td></td><td>Unassigned</td><td>For future - For any other data</td></tr></tbody></table>

**Biometrics**

<table><thead><tr><th width="161">Attribute</th><th width="183">Type</th><th>Attribute Name</th><th>Description</th></tr></thead><tbody><tr><td><code>0</code></td><td><code>bstr</code></td><td>Data</td><td>Biometrics binary data</td></tr><tr><td><code>1</code></td><td><code>int</code></td><td>Data format</td><td>Optional biometrics data format</td></tr><tr><td><code>2</code></td><td><code>int</code></td><td>Data sub format</td><td>Optional biometrics data sub format</td></tr><tr><td><code>3</code></td><td><code>tstr</code></td><td>Data issuer</td><td>Optional biometric data issuer</td></tr></tbody></table>

**Data formats**

| Data format | Description |
| ----------- | ----------- |
| `0`         | Image       |
| `1`         | Template    |
| `2`         | Sound       |
| `3`         | Bio hash    |

**Data sub formats**

**Image**

| Subformat  | Description     |
| ---------- | --------------- |
| `0`        | PNG             |
| `1`        | JPEG            |
| `2`        | JPEG2000        |
| `3`        | AVIF            |
| `4`        | WEBP            |
| `5`        | TIFF            |
| `6`        | WSQ             |
| `100..200` | Vendor specific |

**Template**

| Subformat  | Description                      |
| ---------- | -------------------------------- |
| `0`        | Fingerprint Template ANSI 378    |
| `1`        | Fingerprint Template ISO 19794-2 |
| `2`        | Fingerprint Template NIST        |
| `100..200` | Vendor specific                  |

**Sound**

| Subformat | Description |
| --------- | ----------- |
| `0`       | WAV         |
| `1`       | MP3         |

<details>

<summary><mark style="color:blue;">Guidelines</mark></summary>

* The reserved numeric ranges defined above are designated for **global interoperability** and should be used consistently across implementations.
* **Unassigned numbers** may be utilized within **closed ecosystems**, provided both the issuer and the consumer mutually agree that such usage is **not intended for global interoperability**.
* To propose **new globally consumable attributes** for inclusion within the interoperable assigned range, entities are encouraged to contact the **Claim 169 Working Group** using the contact details provided below.

</details>

### <mark style="color:blue;">**Note on Standard CWT Attributes**</mark>

The attributes listed below are already included as part of the standard CWT (CBOR Web Token) metadata in the payload and therefore do not need to be separately specified within Claim 169. Implementers should refer to the standard CWT definitions for format and usage.

These fields are inherently part of the CWT structure and must be interpreted according to the standard specification. For details, refer to the [IANA CWT registry here](https://www.iana.org/assignments/cwt/cwt.xhtml).

**Included Attributes:**

| Attribute | Attribute Type | Attribute Name        | Description                                                |
| --------- | -------------- | --------------------- | ---------------------------------------------------------- |
| `1`       | `tstr`         | Issuer (iss)          | Identifier of the entity issuing the credential            |
| `2`       | `tstr`         | Subject (sub)         | Identifier of the subject of the credential                |
| `4`       | `int`          | Expiration Time (exp) | Timestamp indicating when the credential expires           |
| `5`       | `int`          | Not Before (nbf)      | Timestamp before which the credential must not be accepted |
| `6`       | `int`          | Issued At (iat)       | Timestamp indicating when the credential was issued        |

### <mark style="color:blue;">**Note on Standard COSE Attributes**</mark>

The attributes listed below are already defined as part of the standard CWT structure, and may be present in either the protected or unprotected COSE headers. As such, they do not need to be separately specified within Claim 169.

Implementers MUST refer to the standard COSE specifications for the correct format, semantics, and usage of these fields. These attributes are inherently part of the COSE structure and must be interpreted in accordance with the relevant standards. For details, refer to the [IANA COSE registry here](https://www.iana.org/assignments/cose/cose.xhtml). The issuer may choose whether to include any of these attributes for purposes such as **public key discovery**, taking into account practical constraints such as QR code size limitations.

**Included Attributes:**

| Attribute | Attribute Type  | Attribute Name | Description                             |
| --------- | --------------- | -------------- | --------------------------------------- |
| `32`      | `COSE_X509`     | x4bag          | An unordered list of X.509 certificates |
| `33`      | `COSE_X509`     | x5chain        | An ordered chain of X.509 certificates  |
| `34`      | `COSE_CertHash` | x5t            | Hash of an X.509 certificate            |
| `35`      | `uri`           | x5u            | URI pointing to an X.509 certificate    |

### <mark style="color:blue;">**Note on Status of Credential**</mark>

The status of the credential, when represented using CWT, is outside the scope of the Claim 169 CBOR structure. This specification does not define or constrain how credential status should be encoded or managed within a CWT. Issuers may determine the status handling mechanism independently, using the IETF-recommended CWT conventions, or any other standards-compliant method appropriate for their ecosystem. No specific guidance or constraints on CWT handling are prescribed by this specification.

### 3.2 CBOR Map Structure Example

```json
1: www.mosip.io # iss
4: 1787912445 # exp
5: 1756376445 # nbf
6: 1756376445 # iat
169: # identity-data
  1: 3918592438 # ID
  2: 1.0 # Version
  3: eng # Language
  4: Janardhan BS # Full name
  8: 19880102 # Date of birth
  9: 1 # Gender: Male
  10: New House, Near Metro Line, Bengaluru, KA # Address
  11: janardhan@example.com # Email ID
  12: "+919876543210" # Phone number
  13: IN # Nationality
  14: 2 # Marital status: Married
  16: 03CBABDF83D068ACB5DE65B3CDF25E0036F2C54(...)E54D23D8EC7DC9BB9F69FD7B7B23383B64F22E25F # Binary image
  17: 2 # Binary image format: JPEG
  18: [1, 2] # Best quality fingers
  19 : جاناردان بنغالور سرينيفاس
  20 : AR
  21 : "849VCWC8+R9"
  22 : Refugee
  23 : IN
  50: # Right Thumb Biometrics
    # Right Thumb image
    - 0: 03CBA(...)0378C58 # Data
      1: 0 # Image
      2: 1 # JPEG
    # Right Thumb template
    - 0: 03CBA(...)0378C58 # Data
      1: 1 # Template
      2: 100 # Vendor specific
      3: VendorA # Biometric data issuer
  51: # Right Pointer Finger Biometrics
    # Right Pointer Finger image
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Image
      2: 6 # WSQ
      3: VendorA # Biometric data issuer
    # Right Pointer Finger template
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Template
      2: 1 # Fingerprint Template ISO 19794-2
      3: VendorA # Biometric data issuer
  58: # Left Ring Finger Biometrics
    # Left Ring Finger image
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Image
      2: 6 # WSQ
      3: VendorA # Biometric data issuer
    # Left Ring Finger template
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Template
      2: 1 # Fingerprint Template ISO 19794-2
      3: VendorA # Biometric data issuer
   60: # Right Iris Biometrics
    # Right Iris image
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Image
      2: 6 # WSQ
      3: VendorX # Biometric data issuer
    # Right Iris image 
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Image
      2: 6 # WSQ
      3: VendorY # Biometric data issuer
   61: # Left Iris Biometrics
    # Left Iris template
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Template
      2: 100 # Vendor specific
      3: VendorX # Biometric data issuer
    # Left Iris image
    - 0: 36F2C546(...)CB90378C58 # Data
      1: 1 # Template
      2: 100 # Vendor specific
      3: VendorY # Biometric data issuer
   65: # Voice Biometrics
    # Voice sound
    - 0: 03CBA(...)0378C58 # Data
      1: 2 # Sound
      2: 1 # MP3
    # Voice template
    - 0: 03CBA(...)0378C58 # Data
      1: 1 # Template
      2: 100 # Vendor specific
      3: VendorZ # Biometric data issuer
```

### 3.2.1 Steps for Claim 169 Compliant QR Code Generation

* **Prepare Identity Data**: Start with the sample JSON identity data provided for conversion into Claim 169 format.

```
{
  "id": "3918592438",
  "fullName": "Janardhan BS",
  "dob": "1984-04-18",
  "gender": "Male",
  "address": "New House, Near Metro Line, Bengaluru, KA",
  "email": "janardhan@example.com",
  "phone": "+919876543210",
  "nationality": "IN",
  "SecondaryLangFullName": "جاناردان بنغالور سرينيفاس",
  "SecondaryLangCode": "AR",
  "locationCode": "j849VCWC8+R9",
  "legalStatus": "Refugee",
  "countryOfIssuance": "IN",
  "face": {
    "data": "52494646dc0100005745425056503820d0010000b00d009d012a400040003e913c9b4925
    a322a12a1ccae8b01209690013e295b2585d5ee72395f7fe4a35103d1894a549b58a4febe751ae9a3
    d00cb96f016fc35075f892786b3bcce1deffb2b3e55e3598b7d4913c80a237f1d9e51be7f271cc971
    d63fda0c2c3c34b27a574ec1bbd7752969c56c8c0000fefeffce44d1e6b7ad2535538b4cc7a3cf016
    f5b7d160c4e7202269bc041f0609efdf8e687702cdd6bd64e90b2931c9210f095f3c3bef00a954bfe
    f4e70c76948b9eedf20e5be9e885edbcceada8f6fbdb9037490fa2eecaeaa62de8123028505f9f2eb
    2f781fdfc9b55ff127f12cb657cdc5927866e650426e3032500af838514711241395bfb130fda3c29
    d836527eeb82d92121b5a6f3b951d4ecc51ae1566c58266227b0f02ced0050fe35e0e42a33026a2c4
    4c581fc65ddd135b6a7e5bc888ef852f6c477ccd817b850b90fa3565e11b61e7fe46f965abe210d09
    7ef03eaaf028c4ff9dff5f55ad472464b4920a5958b8c98ef0e0029160f20a8f4d1a02ad3b5ad0c43
    c0b03dc549576cafb6c3d6c36f1014c57d94f6985f8a328dc7aef8df3507041dc440e99fe9acd90cd
    3ede4381d5b3d64064bce4bb8d05113fd901b158698312bdf8a21049288d6006a2c944dae7bc3e240
    00000",
    "dataFormat": "image",
    "dataSubFormat": "png"
  }
}
```

* **Convert to Claim 169 Format**
  * Transform the JSON data into the required Claim 169 structure.
  * Refer to the sample converted data for guidance.

```
{
	1: "3918592438",
	4: "Janardhan BS",
	8: "1984-04-18",
	9: 1,
	10: "New House, Near Metro Line, Bengaluru, KA",
	11: "janardhan@example.com",
	12: "+919876543210",
	13: "IN",
  19 : "جاناردان بنغالور سرينيفاس",
  20 : "AR",
  21 : "849VCWC8+R9",
  22 : "Refugee",
  23 : "IN",
	62: {
			0: "52494646dc0100005745425056503820d0010000b00d009d012a40004
			0003e913c9b4925a322a12a1ccae8b01209690013e295b2585d5ee72395f7
			fe4a35103d1894a549b58a4febe751ae9a3d00cb96f016fc35075f892786b
			3bcce1deffb2b3e55e3598b7d4913c80a237f1d9e51be7f271cc971d63fda
			0c2c3c34b27a574ec1bbd7752969c56c8c0000fefeffce44d1e6b7ad25355
			38b4cc7a3cf016f5b7d160c4e7202269bc041f0609efdf8e687702cdd6bd6
			4e90b2931c9210f095f3c3bef00a954bfef4e70c76948b9eedf20e5be9e88
			5edbcceada8f6fbdb9037490fa2eecaeaa62de8123028505f9f2eb2f781fd
			fc9b55ff127f12cb657cdc5927866e650426e3032500af838514711241395
			bfb130fda3c29d836527eeb82d92121b5a6f3b951d4ecc51ae1566c582662
			27b0f02ced0050fe35e0e42a33026a2c44c581fc65ddd135b6a7e5bc888ef
			852f6c477ccd817b850b90fa3565e11b61e7fe46f965abe210d097ef03eaa
			f028c4ff9dff5f55ad472464b4920a5958b8c98ef0e0029160f20a8f4d1a0
			2ad3b5ad0c43c0b03dc549576cafb6c3d6c36f1014c57d94f6985f8a328dc
			7aef8df3507041dc440e99fe9acd90cd3ede4381d5b3d64064bce4bb8d051
			13fd901b158698312bdf8a21049288d6006a2c944dae7bc3e24000000",
			1: 0,
			2: 4
	}
}
```

* **Generate CWT Data**
  * Use the Claim 169–formatted data to create the CBOR Web Token (CWT) for QR code generation.

```
61 / CWT Tag / (
 		18 / COSE_Sign1 Tag  / (
				[
				 	h'A10127', / Protected Header /
				  {4: h'6B2D31313031'}, / Unprotected Header /
				  h'A5016C7777772E6D6F7369702E696F041A6A9160FD051A68B02D7D061A68B02D7D18A95902
				  6DA9016A33393138353932343338046C4A616E61726468616E20425308683139383430343138
				  0961310A78294E657720486F7573652C204E656172204D6574726F204C696E652C2042656E67
				  616C7572752C204B410B756A616E61726468616E406578616D706C652E636F6D0C6D2B393139
				  3837363534333231300D62494E183EA3005901E452494646DC0100005745425056503820D001
				  0000B00D009D012A400040003E913C9B4925A322A12A1CCAE8B01209690013E295B2585D5EE7
				  2395F7FE4A35103D1894A549B58A4FEBE751AE9A3D00CB96F016FC35075F892786B3BCCE1DEF
				  FB2B3E55E3598B7D4913C80A237F1D9E51BE7F271CC971D63FDA0C2C3C34B27A574EC1BBD775
				  2969C56C8C0000FEFEFFCE44D1E6B7AD2535538B4CC7A3CF016F5B7D160C4E7202269BC041F0
				  609EFDF8E687702CDD6BD64E90B2931C9210F095F3C3BEF00A954BFEF4E70C76948B9EEDF20E
				  5BE9E885EDBCCEADA8F6FBDB9037490FA2EECAEAA62DE8123028505F9F2EB2F781FDFC9B55FF
				  127F12CB657CDC5927866E650426E3032500AF838514711241395BFB130FDA3C29D836527EEB
				  82D92121B5A6F3B951D4ECC51AE1566C58266227B0F02CED0050FE35E0E42A33026A2C44C581
				  FC65DDD135B6A7E5BC888EF852F6C477CCD817B850B90FA3565E11B61E7FE46F965ABE210D09
				  7EF03EAAF028C4FF9DFF5F55AD472464B4920A5958B8C98EF0E0029160F20A8F4D1A02AD3B5A
				  D0C43C0B03DC549576CAFB6C3D6C36F1014C57D94F6985F8A328DC7AEF8DF3507041DC440E99
				  FE9ACD90CD3EDE4381D5B3D64064BCE4BB8D05113FD901B158698312BDF8A21049288D6006A2
				  C944DAE7BC3E2400000001000204', / Payload with claim 169 tag /
				  h'74E64803A946B30EC091D138433DD6A288CCBB44A8614DFA6094695B998FBCC9D8AD3EEB56
				  8B3360FA67EEAD58B89F924DB5F58781A80E501E908231EDEE1C05' / Signature /
				]
		)
)
```

* **Compress the CWT**
  * Apply `zlib` compression to the generated CWT data.
* **Encode to Base45 and Generate QR Code**
  * Encode the compressed CWT using Base45.
  * Use this encoded string to generate the final QR code.

## 4. Security Considerations

1. The current MAP structure is in plain text and is equivalent to having a physical card with printed details. Additionally, the QR code is digitally signed, providing trust and preventing tampering or the insertion of fake information. Please ensure that you do not include any data elements that are not permissible under your country’s legal and regulatory requirements.
2. CWT MUST be signed, create a COSE\_Sign/COSE\_Sign1 object using the Message as the COSE\_Sign/COSE\_Sign1 Payload; all steps specified in [RFC8152](https://www.rfc-editor.org/rfc/rfc8152) for creating a COSE\_Sign/COSE\_Sign1 object MUST be followed.
3. If the CWT is a COSE\_Encrypt/COSE\_Encrypt0 object,create a COSE\_Encrypt/COSE\_Encrypt0 using the Message as the plaintext for the COSE\_Encrypt/COSE\_Encrypt0 object; all steps specified in [RFC8152](https://www.rfc-editor.org/rfc/rfc8152) for creating a COSE\_Encrypt/COSE\_Encrypt0 object MUST be followed.
   1. It is recommended that sensitive information, such as biometrics, be encrypted.
   2. If you choose to encrypt the payload, please ensure that key sharing for decryption is handled separately, outside the scope of this specification. There are multiple key-sharing mechanisms that can be followed, and the choice remains to be outside of this specification. As an example, you could potentially follow the approach of the TOTP mobile authenticators.
   3. A cached key may be used to enable offline encrypted QR code reading, where applicable.
4. To verify the claims the CWT is a COSE\_Sign/COSE\_Sign1. Follow the steps specified in Section 4 of [RFC8152](https://www.rfc-editor.org/rfc/rfc8152) ("Signing Objects") for validating a COSE\_Sign/COSE\_Sign1 object. Let the Message be the COSE\_Sign/COSE\_Sign1 payload. Once signature is valid we SHOULD validate the public key against a preconfigured key, in case encrypted. Else, if the CWT is a COSE\_Encrypt/COSE\_Encrypt0 object, follow the steps specified in Section 5 of \[[RFC8152](https://www.rfc-editor.org/rfc/rfc8152)] ("Encryption Objects") for validating a COSE\_Encrypt/COSE\_Encrypt0 object. Let the Message be the resulting plaintext.

The security of the CWT relies upon on the protections offered by COSE. Unless the claims in a CWT are protected, an adversary can modify, add, or remove claims.

Since the claims conveyed in a CWT are used to make identity claim decisions, it is not only important to protect the CWT but also to ensure that the recipient can authenticate the party that assembled the claims and created the CWT. Without trust of the recipient in the party that created the CWT, no sensible identity verification can be made. Furthermore, the creator of the CWT needs to carefully evaluate each claim value prior to including it in the CWT, so that the recipient can be assured of the validity of the information provided.

Syntactically, the signing and encryption operations for Nested CWTs may be applied in any order; however, if encryption is necessary, producers normally should sign the message and then encrypt the result (thus encrypting the signature). This prevents attacks in which the signature is stripped, leaving just an encrypted message, as well as providing privacy for the signer. Furthermore, signatures over encrypted text are not considered valid in many jurisdictions.

## 5. IANA Considerations:

### 5.1 Registry Content

Claim Name: identity-data\
Claim Description: Registering the claim for storing identity data of a person, which could be Personally Identifiable Data (PII) mostly used in Foundational/National ID for cross-border interoperability.\
Claim Key: 169\
Claim Value Type(s): map\
Change Controller: MOSIP\
Specification Document(s): Section 3, Section 4

## 6. Acknowledgments

This work is the result of the dedicated efforts of contributors who recognize the critical importance of interoperability and a consistent QR code specification. The revised version has been shaped significantly by the input of our working group committee, comprising members from the following organizations: GetGroup, PWC, Tech 5, UNHCR, Ooru, GIZ, OpenSPP.

We extend our gratitude to the committee members for their invaluable time and insights throughout the evaluation phase.

### 6.1 Working Group Committee Members:

GetGroup: Aiman Tarek\
PWC: Chaitanya Giri\
Tech 5: Bejoy Ak, Nelson Branco, Rahul Parthe\
UNHCR: Norbert Trosien, Samantha Eisenhauer, Sam Jefferies\
Ooru: Rounak Nayak, Priyank Trivdei\
GIZ: Anita Mittal, Aisha Merhebi\
OpenSPP: Jeremi Joslin\
\
MOSIP: Janardhan BS, Mahammed Taheer, Mayura Deshmukh, Pragya Kumari, Preeti Hongal, Ramesh Narayanan, Reeba Thomas, Resham Chugani, Sanchi Singh, Sasikumar Ganesan, Sivanand Lanka, Swati Goel, Varaniya Selvaraja, Vishwanath V

## 7. Authors

Mahammed Taheer ([mohd.taheer@gmail.com](mailto:mohd.taheer@gmail.com))

Resham Chugani ([resham@mosip.io](mailto:resham@mosip.io))

Sasikumar G ([sasi@](mailto:sasi@duck.com)mosip.io)

## 8. What Changed

*   **Addition of new attributes (19–23)**&#x20;

    * \#19: Full Name - Secondary Language
    * \#20: Secondary Language
    * \#21: Location Code
    * \#22: Legal Status
    * \#23: Country of Issuance

    Refer to the [table above](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#id-3.1-cbor-map-structure-overview) for details.
* **Inclusion of/updates to the following sections:**
  * [Guidelines](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#guidelines)
  * [Standard CWT attributes](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-standard-cwt-attributes)
  * [Standard COSE attributes](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-standard-cose-attributes)
  * [Credential status](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#note-on-status-of-credential)&#x20;
  * [Security considerations](https://docs.mosip.io/1.2.0/readme/standards-and-specifications/mosip-standards/169-qr-code-specification#id-4.-security-considerations)
