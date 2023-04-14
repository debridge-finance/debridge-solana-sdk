use crate::sending::SendSubmissionParamsInput;
pub const UNWRAP_ETH: u8 = 0;
pub const REVERT_IF_EXTERNAL_FAIL: u8 = 1;
pub const PROXY_WITH_SENDER: u8 = 2;
pub const SEND_HASHED_DATA: u8 = 3;
pub const DIRECT_WALLET_FLOW: u8 = 31;

pub trait SetReservedFlag {
    fn set_unwrap_eth(&mut self) {
        self.set_flag::<UNWRAP_ETH>()
    }
    fn set_revert_if_external_call(&mut self) {
        self.set_flag::<REVERT_IF_EXTERNAL_FAIL>()
    }
    fn set_proxy_with_sender(&mut self) {
        self.set_flag::<PROXY_WITH_SENDER>()
    }
    fn set_send_hashed_data(&mut self) {
        self.set_flag::<SEND_HASHED_DATA>()
    }
    fn set_direct_flow(&mut self) {
        self.set_flag::<DIRECT_WALLET_FLOW>()
    }

    fn set_flag<const FLAG: u8>(&mut self);
}

impl SetReservedFlag for [u8; 32] {
    fn set_flag<const FLAG: u8>(&mut self) {
        self[31 - FLAG as usize / 8] |= 1 << (FLAG % 8);
    }
}

pub trait CheckReservedFlag {
    fn check_bit(self, bit: u8) -> bool;
    fn check_unwrap_eth(self) -> bool;
    fn check_revert_if_external_call(self) -> bool;
    fn check_proxy_with_sender(self) -> bool;
    fn check_send_hashed_data(self) -> bool;
    fn check_direct_flow(self) -> bool;
}
impl CheckReservedFlag for &[u8; 32] {
    fn check_bit(self, bit: u8) -> bool {
        self[31 - bit as usize / 8] & (1 << (bit % 8)) == (1 << (bit % 8))
    }
    fn check_unwrap_eth(self) -> bool {
        self.check_bit(UNWRAP_ETH)
    }
    fn check_revert_if_external_call(self) -> bool {
        self.check_bit(REVERT_IF_EXTERNAL_FAIL)
    }
    fn check_proxy_with_sender(self) -> bool {
        self.check_bit(PROXY_WITH_SENDER)
    }
    fn check_send_hashed_data(self) -> bool {
        self.check_bit(SEND_HASHED_DATA)
    }
    fn check_direct_flow(self) -> bool {
        self.check_bit(DIRECT_WALLET_FLOW)
    }
}
impl CheckReservedFlag for &SendSubmissionParamsInput {
    fn check_bit(self, bit: u8) -> bool {
        self.flags.check_bit(bit)
    }
    fn check_unwrap_eth(self) -> bool {
        self.flags.check_unwrap_eth()
    }
    fn check_revert_if_external_call(self) -> bool {
        self.flags.check_revert_if_external_call()
    }
    fn check_proxy_with_sender(self) -> bool {
        self.flags.check_proxy_with_sender()
    }
    fn check_send_hashed_data(self) -> bool {
        self.flags.check_send_hashed_data()
    }
    fn check_direct_flow(self) -> bool {
        self.flags.check_direct_flow()
    }
}

#[cfg(test)]
mod flag_test {
    use crate::flags::{CheckReservedFlag, SetReservedFlag};

    #[test]
    fn bit_test() {
        let expect = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let mut actual = [0; 32];
        actual.set_flag::<128>();

        assert!(actual.check_bit(128));
        assert_eq!(expect, actual);
    }

    #[test]
    fn unwrap_eth_test() {
        let expect = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ];

        let mut actual = [0; 32];
        actual.set_unwrap_eth();

        assert_eq!(expect, actual);
        assert!(actual.check_unwrap_eth());
    }

    #[test]
    fn unwrap_revert_if_external_call_test() {
        let expect = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2,
        ];

        let mut actual = [0; 32];
        actual.set_revert_if_external_call();

        assert_eq!(expect, actual);
        assert!(actual.check_revert_if_external_call());
    }

    #[test]
    fn unwrap_proxy_with_sender_test() {
        let expect = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4,
        ];

        let mut actual = [0; 32];
        actual.set_proxy_with_sender();

        assert_eq!(expect, actual);
        assert!(actual.check_proxy_with_sender());
    }

    #[test]
    fn unwrap_send_hashed_data_test() {
        let expect = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 8,
        ];

        let mut actual = [0; 32];
        actual.set_send_hashed_data();

        assert_eq!(expect, actual);
        assert!(actual.check_send_hashed_data());
    }

    #[test]
    fn direct_flow_test() {
        let expect = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            128, 0, 0, 0,
        ];

        let mut actual = [0; 32];
        actual.set_direct_flow();

        assert_eq!(expect, actual);
        assert!(actual.check_direct_flow());
    }
}
