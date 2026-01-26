/**
 * Error parsing and display utilities for the Claim 169 playground.
 * Provides structured error information with actionable suggestions.
 */

/**
 * Pipeline stages in the Claim 169 decoding process.
 */
export type PipelineStage =
  | 'Base45'
  | 'zlib'
  | 'COSE_Encrypt0'
  | 'COSE_Sign1'
  | 'CWT/CBOR'
  | 'Claim169'

/**
 * Parsed error information with stage detection and suggestions.
 */
export interface ParsedError {
  /** The pipeline stage where the error occurred */
  stage: PipelineStage
  /** User-friendly error message */
  message: string
  /** Original error message for technical details */
  originalMessage: string
  /** Actionable suggestion for resolving the error */
  suggestion?: string
}

/**
 * Pipeline stage status for display.
 */
export interface PipelineStageStatus {
  name: string
  status: 'pending' | 'success' | 'error' | 'skipped'
  detail?: string
}

/**
 * Parse a decode error message to determine which pipeline stage failed
 * and provide helpful suggestions.
 */
export function parseDecodeError(errorMessage: string): ParsedError {
  const msg = errorMessage.toLowerCase()
  const original = errorMessage

  // Base45 decoding errors
  if (msg.includes('base45') || msg.includes('invalid character') && msg.includes('base')) {
    return {
      stage: 'Base45',
      message: 'Invalid QR code format.',
      originalMessage: original,
      suggestion:
        'The QR code contains invalid Base45 characters. Ensure you\'re scanning a valid Claim 169 QR code.',
    }
  }

  // Zlib decompression errors
  if (
    msg.includes('decompress') ||
    msg.includes('zlib') ||
    msg.includes('inflate') ||
    msg.includes('decompression')
  ) {
    return {
      stage: 'zlib',
      message: 'Decompression failed.',
      originalMessage: original,
      suggestion:
        'The QR code data could not be decompressed. The data may be corrupted or not a valid Claim 169 credential.',
    }
  }

  // Decryption errors (COSE_Encrypt0)
  if (
    msg.includes('decryption failed') ||
    msg.includes('decrypt') && msg.includes('fail') ||
    msg.includes('aes') && msg.includes('error') ||
    msg.includes('authentication tag')
  ) {
    return {
      stage: 'COSE_Encrypt0',
      message: 'Decryption failed.',
      originalMessage: original,
      suggestion:
        'The decryption key is incorrect or the encrypted data is corrupted. Verify you\'re using the correct AES key.',
    }
  }

  // Signature verification errors (COSE_Sign1)
  if (
    msg.includes('signature') ||
    msg.includes('verification failed') ||
    msg.includes('unsupported algorithm') ||
    msg.includes('invalid public key') ||
    msg.includes('pem') && (msg.includes('invalid') || msg.includes('error'))
  ) {
    return {
      stage: 'COSE_Sign1',
      message: 'Signature verification failed.',
      originalMessage: original,
      suggestion:
        'The public key doesn\'t match the credential\'s signature, or the credential wasn\'t signed with the expected algorithm. Verify you\'re using the correct public key.',
    }
  }

  // CBOR/CWT parsing errors
  if (msg.includes('cbor') || msg.includes('cwt') || msg.includes('cose parse')) {
    return {
      stage: 'CWT/CBOR',
      message: 'Invalid credential structure.',
      originalMessage: original,
      suggestion:
        'The credential\'s internal structure is invalid. This may indicate a corrupted or non-standard credential.',
    }
  }

  // Timestamp validation errors
  if (msg.includes('expired') || msg.includes('exp')) {
    return {
      stage: 'Claim169',
      message: 'Credential has expired.',
      originalMessage: original,
      suggestion:
        'This credential has passed its expiration date and should no longer be accepted.',
    }
  }

  if (msg.includes('not valid until') || msg.includes('not yet valid') || msg.includes('nbf')) {
    return {
      stage: 'Claim169',
      message: 'Credential not yet valid.',
      originalMessage: original,
      suggestion:
        'This credential\'s validity period hasn\'t started yet. Check if the system clock is correct.',
    }
  }

  // Date validation errors (from TypeScript SDK date validation)
  if (
    (msg.includes('invalid') && (msg.includes('dateofbirth') || msg.includes('date_of_birth') || msg.includes('date of birth'))) ||
    (msg.includes('yyyy-mm-dd') || msg.includes('yyyymmdd') || msg.includes('expected yyyy'))
  ) {
    return {
      stage: 'Claim169',
      message: 'Invalid date format in credential.',
      originalMessage: original,
      suggestion:
        'Date fields must be in ISO 8601 format: YYYY-MM-DD (e.g., 1984-04-18) or YYYYMMDD (e.g., 19840418). The credential contains a malformed date value.',
    }
  }

  // Claim 169 not found
  if (msg.includes('claim 169 not found') || msg.includes('claim169 not found')) {
    return {
      stage: 'Claim169',
      message: 'Not a Claim 169 credential.',
      originalMessage: original,
      suggestion:
        'The QR code doesn\'t contain a Claim 169 identity credential. It may be a different type of QR code.',
    }
  }

  // Generic Claim169 stage error
  if (msg.includes('claim169') || msg.includes('claim 169')) {
    return {
      stage: 'Claim169',
      message: 'Invalid credential data.',
      originalMessage: original,
      suggestion:
        'The credential data contains invalid or unexpected values.',
    }
  }

  // Default: unknown error, assume COSE parsing issue
  return {
    stage: 'CWT/CBOR',
    message: 'Failed to decode credential.',
    originalMessage: original,
    suggestion:
      'An unexpected error occurred while decoding the credential. Check that the QR code is a valid Claim 169 credential.',
  }
}

/**
 * Build a partial pipeline status array showing which stages completed
 * before the error occurred.
 */
export function buildPartialPipeline(
  failedStage: PipelineStage,
  isEncrypted: boolean
): PipelineStageStatus[] {
  const stages: PipelineStageStatus[] = [
    { name: 'Base45', status: 'pending' },
    { name: 'zlib', status: 'pending' },
  ]

  if (isEncrypted) {
    stages.push({ name: 'COSE_Encrypt0', status: 'pending' })
  }

  stages.push(
    { name: 'COSE_Sign1', status: 'pending' },
    { name: 'CWT/CBOR', status: 'pending' },
    { name: 'Claim169', status: 'pending' }
  )

  // Mark stages as success until we hit the failed stage
  for (let i = 0; i < stages.length; i++) {
    if (stages[i].name === failedStage) {
      stages[i].status = 'error'
      break
    }
    stages[i].status = 'success'
  }

  return stages
}

/**
 * Get user-friendly stage name for display.
 */
export function getStageDisplayName(stage: PipelineStage): string {
  switch (stage) {
    case 'Base45':
      return 'Base45 Decode'
    case 'zlib':
      return 'Decompress'
    case 'COSE_Encrypt0':
      return 'Decryption'
    case 'COSE_Sign1':
      return 'Signature Verify'
    case 'CWT/CBOR':
      return 'CWT Parse'
    case 'Claim169':
      return 'Claim 169'
    default:
      return stage
  }
}
