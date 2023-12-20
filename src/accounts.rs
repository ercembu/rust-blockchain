#[allow(dead_code)]
pub(crate) mod accounts { 

    use bytes::Bytes;
    use ethereum_types as eth_types;

    type Account = eth_types::Address;

    struct AccountData {
        nonce: u64,
        balance: u64,
        code_hash: Option<Bytes>,
    }

    impl AccountData {
        fn is_contract(&self) -> bool {
            self.code_hash.is_some()
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::crypto_util::{random_keypair};
    use crate::crypto_util::AddressUtil;
    #[test]
    fn test_1() {
        let (private_key, public_key) = random_keypair();

        let address = public_key.to_address();

    }
}
