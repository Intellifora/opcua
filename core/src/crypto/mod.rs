//! This module contains crypto related functionality. It is used for establishing
//! trust between a client and server via certificate exchange and validation. It also used for
//! encrypting / decrypting messages and signing messages.

pub mod types;
pub mod certificate_store;
pub mod encrypt_decrypt;

pub use self::types::*;
pub use self::certificate_store::*;
pub use self::encrypt_decrypt::*;

use types::*;
use comms::SecurityPolicy;

pub mod consts {
    pub mod basic128rsa15
    {
        // 128Rsa15
        //
        // A suite of algorithms that uses RSA15 as Key-Wrap-algorithm and 128-Bit for encryption algorithms.
        //
        // -> SymmetricSignatureAlgorithm – HmacSha1 – (http://www.w3.org/2000/09/xmldsig#hmac-sha1).
        // -> SymmetricEncryptionAlgorithm – Aes128 – (http://www.w3.org/2001/04/xmlenc#aes128-cbc).

        /// AsymmetricSignatureAlgorithm – RsaSha1 – (http://www.w3.org/2000/09/xmldsig#rsa-sha1).
        pub const ASYMMETRIC_SIGNATURE_ALGORITHM: &'static str = "http://www.w3.org/2000/09/xmldsig#rsa-sha1";

        // -> AsymmetricKeyWrapAlgorithm – KwRsa15 – (http://www.w3.org/2001/04/xmlenc#rsa-1_5).
        // -> AsymmetricEncryptionAlgorithm – Rsa15 – (http://www.w3.org/2001/04/xmlenc#rsa-1_5).
        // -> KeyDerivationAlgorithm – PSha1 – (http://docs.oasis-open.org/ws-sx/ws-secureconversation/200512/dk/p_sha1).
        // -> DerivedSignatureKeyLength – 128.
        // -> MinAsymmetricKeyLength – 1024
        // -> MaxAsymmetricKeyLength – 2048
        // -> CertificateSignatureAlgorithm – Sha1
        //
        // If a certificate or any certificate in the chain is not signed with a hash that is Sha1 or stronger then the certificate shall be rejected.
    }

    pub mod basic256 {
        // Security Basic 256
        //
        // A suite of algorithms that are for 256-Bit encryption, algorithms include:
        //
        // -> SymmetricSignatureAlgorithm – HmacSha1 – (http://www.w3.org/2000/09/xmldsig#hmac-sha1).
        // -> SymmetricEncryptionAlgorithm – Aes256 – (http://www.w3.org/2001/04/xmlenc#aes256-cbc).

        /// AsymmetricSignatureAlgorithm – RsaSha1 – (http://www.w3.org/2000/09/xmldsig#rsa-sha1).
        pub const ASYMMETRIC_SIGNATURE_ALGORITHM: &'static str = "http://www.w3.org/2000/09/xmldsig#rsa-sha1";

        // -> AsymmetricKeyWrapAlgorithm – KwRsaOaep – (http://www.w3.org/2001/04/xmlenc#rsa-oaep-mgf1p).
        // -> AsymmetricEncryptionAlgorithm – RsaOaep – (http://www.w3.org/2001/04/xmlenc#rsa-oaep).
        // -> KeyDerivationAlgorithm – PSha1 – (http://docs.oasis-open.org/ws-sx/ws-secureconversation/200512/dk/p_sha1).
        // -> DerivedSignatureKeyLength – 192.
        // -> MinAsymmetricKeyLength – 1024
        // -> MaxAsymmetricKeyLength – 2048
        // -> CertificateSignatureAlgorithm –
    }

    // Sha1 [deprecated] or Sha256 [recommended]
    //
    // If a certificate or any certificate in the chain is not signed with a hash that is Sha1 or stronger then the certificate shall be rejected.
    // Release 1.03 17 OPC Unified Architecture, Part 7
    // Both Sha1 and Sha256 shall be supported. However, it is recommended to use Sha256 since Sha1 is considered not secure anymore.

    pub mod basic256sha256 {
        // Security Basic 256 Sha256
        //
        // A suite of algorithms that are for 256-Bit encryption, algorithms include.
        //
        // -> SymmetricSignatureAlgorithm – Hmac_Sha256 – (http://www.w3.org/2000/09/xmldsig#hmac-sha256).
        // -> SymmetricEncryptionAlgorithm – Aes256_CBC – (http://www.w3.org/2001/04/xmlenc#aes256-cbc).

        /// AsymmetricSignatureAlgorithm – Rsa_Sha256 – (http://www.w3.org/2001/04/xmldsig#rsa-sha256).
        pub const ASYMMETRIC_SIGNATURE_ALGORITHM: &'static str = "http://www.w3.org/2001/04/xmldsig#rsa-sha256";

        // -> AsymmetricKeyWrapAlgorithm – KwRsaOaep – (http://www.w3.org/2001/04/xmlenc#rsa-oaep-mgf1p).
        // -> AsymmetricEncryptionAlgorithm – Rsa_Oaep – (http://www.w3.org/2001/04/xmlenc#rsa-oaep).
        // -> KeyDerivationAlgorithm – PSHA256 – (http://docs.oasis-open.org/ws-sx/ws-secureconversation/200512/dk/p_sha256).
        // -> DerivedSignatureKeyLength – 256
        // -> MinAsymmetricKeyLength – 2048
        // -> MaxAsymmetricKeyLength – 4096
        // -> CertificateSignatureAlgorithm – Sha256
        //
        // If a certificate or any certificate in the chain is not signed with a hash that is Sha256 or stronger
        // then the certificate shall be rejected. Support for this security profile may require support for
        // a second application instance certificate, with a larger keysize. Applications shall support
        // multiple Application Instance Certificates if required by supported Security Polices and use
        // the certificate that is required for a given security endpoint.
    }
}

fn concat_data_and_nonce(data: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(data.len() + nonce.len());
    buffer.extend_from_slice(data);
    buffer.extend_from_slice(nonce);
    buffer
}

/// Verifies that cert matches the signed data
pub fn verify_signature(verifying_cert: &X509, signature_data: &SignatureData, data: &ByteString, nonce: &ByteString) -> StatusCode {
    if data.is_null() || nonce.is_null() {
        error!("Data or nonce are null");
        BAD_UNEXPECTED_ERROR
    } else if signature_data.algorithm.is_null() {
        error!("Signature data has no algorithm");
        BAD_UNEXPECTED_ERROR
    } else {
        // Get the pul
        if let Ok(public_key) = verifying_cert.public_key() {
            let data = concat_data_and_nonce(data.as_ref(), nonce.as_ref());
            let signature = signature_data.signature.as_ref();

            let security_policy_uri = signature_data.algorithm.as_ref();
            let security_policy = SecurityPolicy::from_uri(security_policy_uri);

            let verified = match security_policy {
                SecurityPolicy::Basic128Rsa15 | SecurityPolicy::Basic256 => {
                    public_key.verify_sha1(&data, signature)
                }
                SecurityPolicy::Basic256Sha256 => {
                    public_key.verify_sha256(&data, signature)
                }
                SecurityPolicy::None => {
                    error!("Cannot verify a signature with no security policy of None");
                    false
                }
                _ => {
                    error!("An unknown security policy uri {} was passed to signing function and rejected", security_policy_uri);
                    false
                }
            };
            if verified { GOOD } else { BAD_APPLICATION_SIGNATURE_INVALID }
        } else {
            error!("Public key cannot be obtained from cert");
            BAD_UNEXPECTED_ERROR
        }
    }
}

/// Creates a SignatureData object by signing the supplied certificate and nonce with a pkey
pub fn create_signature_data(pkey: &PKey, security_policy_uri: &str, data: &ByteString, nonce: &ByteString) -> SignatureData {
    let (algorithm, signature) = if data.is_null() || nonce.is_null() {
        (UAString::null(), ByteString::null())
    } else {
        let data = concat_data_and_nonce(data.as_ref(), nonce.as_ref());

        // Sign the bytes and return the algorithm, signature
        match SecurityPolicy::from_uri(security_policy_uri) {
            SecurityPolicy::Basic128Rsa15 => (
                UAString::from_str(consts::basic128rsa15::ASYMMETRIC_SIGNATURE_ALGORITHM),
                ByteString::from_bytes(&pkey.sign_sha1(&data))
            ),
            SecurityPolicy::Basic256 => (
                UAString::from_str(consts::basic256::ASYMMETRIC_SIGNATURE_ALGORITHM),
                ByteString::from_bytes(&pkey.sign_sha1(&data))
            ),
            SecurityPolicy::Basic256Sha256 => (
                UAString::from_str(consts::basic256sha256::ASYMMETRIC_SIGNATURE_ALGORITHM),
                ByteString::from_bytes(&pkey.sign_sha256(&data))
            ),
            SecurityPolicy::None => (
                UAString::null(), ByteString::null()
            ),
            _ => {
                error!("An unknown security policy uri {} was passed to signing function and rejected", security_policy_uri);
                (UAString::null(), ByteString::null())
            }
        }
    };
    SignatureData { algorithm, signature }
}