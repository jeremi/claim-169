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
