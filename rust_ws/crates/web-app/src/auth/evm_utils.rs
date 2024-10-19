use alloy_primitives::Signature;
use anyhow::Result;
use std::str::FromStr;

// verify that the signature is valid
pub fn is_valid_signature(address: &str, message: &str, signed_message: &str) -> Result<bool> {
    let signature = match Signature::from_str(signed_message) {
        Ok(sig) => sig,
        _ => return Ok(false),
    };
    let recovered_address = match signature.recover_address_from_msg(message) {
        Ok(addr) => addr,
        _ => return Ok(false),
    };

    Ok(recovered_address.to_string().as_str() == address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_signature() -> Result<()> {
        let address = "0x20a4Cd6b9aF17c39BF406011c2732C39FeBF19f0";
        let message = "By joining, you agree to our Terms of Service ( https://tradingexec.xyz/legal/terms ) and Privacy Policy ( https://tradingexec.xyz/legal/policy ) .";
        let signed_message = "0x4a358b9f8b92d289469bd089f515a93db8ec3c0f053e8f7a0fd5a9cf100abcdb1b66413414865d92bb832c9d9fe57989cbc923ee9d5f7c042e1d596c19cce8881c";

        let is_valid = is_valid_signature(address, message, signed_message)?;
        Ok(())
    }
}
