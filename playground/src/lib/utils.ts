import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function hexToBytes(hex: string): Uint8Array {
  const cleanHex = hex.replace(/\s/g, '').replace(/^0x/i, '')
  if (cleanHex.length % 2 !== 0) {
    throw new Error('Invalid hex string: odd length')
  }
  const bytes = new Uint8Array(cleanHex.length / 2)
  for (let i = 0; i < cleanHex.length; i += 2) {
    const byte = parseInt(cleanHex.slice(i, i + 2), 16)
    if (isNaN(byte)) {
      throw new Error(`Invalid hex character at position ${i}`)
    }
    bytes[i / 2] = byte
  }
  return bytes
}

export function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map(b => b.toString(16).padStart(2, '0'))
    .join('')
}

export function formatTimestamp(timestamp: number | undefined): string {
  if (!timestamp) return 'N/A'
  const date = new Date(timestamp * 1000)
  return date.toLocaleString()
}

export function isExpired(expiresAt: number | undefined): boolean {
  if (!expiresAt) return false
  return expiresAt < Math.floor(Date.now() / 1000)
}

export function truncateString(str: string, maxLength: number): string {
  if (str.length <= maxLength) return str
  return str.slice(0, maxLength) + '...'
}

export function copyToClipboard(text: string): Promise<void> {
  return navigator.clipboard.writeText(text)
}

export const GENDER_MAP: Record<number, string> = {
  1: 'Male',
  2: 'Female',
  3: 'Other',
}

export const MARITAL_STATUS_MAP: Record<number, string> = {
  1: 'Unmarried',
  2: 'Married',
  3: 'Divorced',
}

export const PHOTO_FORMAT_MAP: Record<number, string> = {
  1: 'JPEG',
  2: 'JPEG2000',
  3: 'AVIF',
  4: 'WebP',
}

export interface KeyPair {
  privateKey: string  // hex
  publicKey: string   // hex
}

/**
 * Generate a random Ed25519 key pair using Web Crypto API.
 * Returns keys as hex strings.
 */
export async function generateEd25519KeyPair(): Promise<KeyPair> {
  try {
    // Try Web Crypto API (supported in modern browsers)
    const keyPair = await crypto.subtle.generateKey(
      { name: "Ed25519" },
      true,
      ["sign", "verify"]
    )

    const privateKeyBuffer = await crypto.subtle.exportKey("pkcs8", keyPair.privateKey)
    const publicKeyBuffer = await crypto.subtle.exportKey("raw", keyPair.publicKey)

    // PKCS8 format for Ed25519 has a 16-byte prefix, the actual key is the last 32 bytes
    const privateKeyBytes = new Uint8Array(privateKeyBuffer).slice(-32)
    const publicKeyBytes = new Uint8Array(publicKeyBuffer)

    return {
      privateKey: bytesToHex(privateKeyBytes),
      publicKey: bytesToHex(publicKeyBytes),
    }
  } catch {
    // Fallback: generate random bytes (public key won't be derivable without WASM support)
    const privateKeyBytes = new Uint8Array(32)
    crypto.getRandomValues(privateKeyBytes)

    return {
      privateKey: bytesToHex(privateKeyBytes),
      publicKey: "", // Can't derive without Ed25519 support
    }
  }
}

/**
 * Generate a random ECDSA P-256 key pair using Web Crypto API.
 * Returns keys as hex strings.
 */
export async function generateEcdsaP256KeyPair(): Promise<KeyPair> {
  try {
    const keyPair = await crypto.subtle.generateKey(
      { name: "ECDSA", namedCurve: "P-256" },
      true,
      ["sign", "verify"]
    )

    const privateKeyBuffer = await crypto.subtle.exportKey("pkcs8", keyPair.privateKey)
    const publicKeyBuffer = await crypto.subtle.exportKey("raw", keyPair.publicKey)

    // Parse PKCS8 to extract the 32-byte private key scalar
    // PKCS8 structure: SEQUENCE { version, AlgorithmIdentifier, OCTET STRING { ECPrivateKey } }
    // ECPrivateKey: SEQUENCE { INTEGER 1, OCTET STRING (32-byte scalar), [1] public key? }
    // Look for pattern: 02 01 01 04 20 (INTEGER 1, OCTET STRING of 32 bytes)
    const privateKeyArray = new Uint8Array(privateKeyBuffer)
    let privateKeyBytes: Uint8Array | null = null

    for (let i = 0; i < privateKeyArray.length - 36; i++) {
      if (
        privateKeyArray[i] === 0x02 &&
        privateKeyArray[i + 1] === 0x01 &&
        privateKeyArray[i + 2] === 0x01 &&
        privateKeyArray[i + 3] === 0x04 &&
        privateKeyArray[i + 4] === 0x20
      ) {
        privateKeyBytes = privateKeyArray.slice(i + 5, i + 5 + 32)
        break
      }
    }

    if (!privateKeyBytes) {
      throw new Error("Failed to extract private key from PKCS8")
    }

    // Raw public key is 65 bytes: 0x04 || x (32 bytes) || y (32 bytes)
    const publicKeyBytes = new Uint8Array(publicKeyBuffer)

    return {
      privateKey: bytesToHex(privateKeyBytes),
      publicKey: bytesToHex(publicKeyBytes),
    }
  } catch {
    // Fallback: generate random bytes
    const privateKeyBytes = new Uint8Array(32)
    crypto.getRandomValues(privateKeyBytes)

    return {
      privateKey: bytesToHex(privateKeyBytes),
      publicKey: "",
    }
  }
}

/**
 * Generate a random AES key.
 * Returns key as hex string.
 */
export function generateAesKey(bits: 128 | 256): string {
  const bytes = new Uint8Array(bits / 8)
  crypto.getRandomValues(bytes)
  return bytesToHex(bytes)
}

/**
 * Key format types for public verification keys
 */
export type PublicKeyFormat = 'hex' | 'pem' | 'unknown'

/**
 * Key format types for encryption keys (AES)
 */
export type EncryptionKeyFormat = 'hex' | 'base64' | 'unknown'

/**
 * Detect the format of a public verification key.
 * @param key - The key string to detect
 * @returns 'hex', 'pem', or 'unknown'
 */
export function detectPublicKeyFormat(key: string): PublicKeyFormat {
  const trimmed = key.trim()

  // PEM format detection: starts with -----BEGIN
  if (trimmed.startsWith('-----BEGIN')) {
    return 'pem'
  }

  // Hex format detection: only hex characters (with optional 0x prefix and whitespace)
  const cleanHex = trimmed.replace(/\s/g, '').replace(/^0x/i, '')
  if (/^[0-9a-fA-F]+$/.test(cleanHex) && cleanHex.length > 0) {
    // Valid lengths for Ed25519 (32 bytes = 64 chars) or P-256 (33/65 bytes = 66/130 chars)
    if (cleanHex.length === 64 || cleanHex.length === 66 || cleanHex.length === 130) {
      return 'hex'
    }
    // Could still be hex but unusual length
    if (cleanHex.length % 2 === 0) {
      return 'hex'
    }
  }

  return 'unknown'
}

/**
 * Detect the format of an encryption key (AES).
 * @param key - The key string to detect
 * @returns 'hex', 'base64', or 'unknown'
 */
export function detectEncryptionKeyFormat(key: string): EncryptionKeyFormat {
  const trimmed = key.trim()

  // Empty string
  if (!trimmed) {
    return 'unknown'
  }

  // Hex format: only hex characters, even length, valid AES key size
  const cleanHex = trimmed.replace(/\s/g, '').replace(/^0x/i, '')
  if (/^[0-9a-fA-F]+$/.test(cleanHex) && cleanHex.length % 2 === 0) {
    // AES-128 (16 bytes = 32 hex chars) or AES-256 (32 bytes = 64 hex chars)
    if (cleanHex.length === 32 || cleanHex.length === 64) {
      return 'hex'
    }
  }

  // Base64 format: valid base64 characters, decodes to valid AES key size
  try {
    // Standard base64 or URL-safe base64
    const normalized = trimmed.replace(/-/g, '+').replace(/_/g, '/')
    const decoded = atob(normalized)
    // AES-128 (16 bytes) or AES-256 (32 bytes)
    if (decoded.length === 16 || decoded.length === 32) {
      return 'base64'
    }
  } catch {
    // Not valid base64
  }

  return 'unknown'
}

/**
 * Parse an encryption key from either hex or base64 format.
 * @param key - The key string (hex or base64)
 * @param expectedLength - Expected key length in bytes (16 or 32)
 * @returns The parsed key as Uint8Array
 * @throws Error if the key format is invalid or length doesn't match
 */
export function parseEncryptionKey(key: string, expectedLength: 16 | 32): Uint8Array {
  const format = detectEncryptionKeyFormat(key)
  const trimmed = key.trim()

  if (format === 'hex') {
    const bytes = hexToBytes(trimmed)
    if (bytes.length !== expectedLength) {
      throw new Error(`Expected ${expectedLength} bytes, got ${bytes.length}`)
    }
    return bytes
  }

  if (format === 'base64') {
    const normalized = trimmed.replace(/-/g, '+').replace(/_/g, '/')
    const decoded = atob(normalized)
    const bytes = new Uint8Array(decoded.length)
    for (let i = 0; i < decoded.length; i++) {
      bytes[i] = decoded.charCodeAt(i)
    }
    if (bytes.length !== expectedLength) {
      throw new Error(`Expected ${expectedLength} bytes, got ${bytes.length}`)
    }
    return bytes
  }

  throw new Error('Invalid encryption key format. Expected hex or base64.')
}
