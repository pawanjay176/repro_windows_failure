#ifndef DATA_H
#define DATA_H

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

#define BYTES_PER_BLOB 131072

/**
 * A basic blob data.
 */
typedef struct {
    uint8_t bytes[BYTES_PER_BLOB];
} Blob;

/**
 * An array of 48 bytes. Represents an untrusted
 * (potentially invalid) commitment/proof.
 */
typedef struct {
    uint8_t bytes[48];
} Bytes48;

/**
 * An array of 32 bytes
 */
typedef struct {
    uint8_t bytes[32];
} Bytes32;


void verify_blob(
    bool *ok,
    const Blob *blobs,
    const Bytes48 *commitments_bytes,
    const Bytes48 *proofs_bytes
);

#endif