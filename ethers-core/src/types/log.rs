// Adapted from https://github.com/tomusdrw/rust-web3/blob/master/src/types/log.rs
use crate::types::{Address, Bytes, H256, U256, U64};
use ethabi::RawLog;
use serde::{Deserialize, Serialize};

/// A log produced by a transaction.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Log {
    /// H160. the contract that emitted the log
    pub address: Address,

    /// topics: Array of 0 to 4 32 Bytes of indexed log arguments.
    /// (In solidity: The first topic is the hash of the signature of the event
    /// (e.g. `Deposit(address,bytes32,uint256)`), except you declared the event
    /// with the anonymous specifier.)
    pub topics: Vec<H256>,

    /// Data
    pub data: Bytes,

    /// Block Hash
    #[serde(rename = "blockHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<H256>,

    /// Block Number
    #[serde(rename = "blockNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<U64>,

    /// Transaction Hash
    #[serde(rename = "transactionHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<H256>,

    /// Transaction Index
    #[serde(rename = "transactionIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_index: Option<U64>,

    /// Integer of the log index position in the block. None if it's a pending log.
    #[serde(rename = "logIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_index: Option<U256>,

    /// Integer of the transactions index position log was created from.
    /// None when it's a pending log.
    #[serde(rename = "transactionLogIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_log_index: Option<U256>,

    /// Log Type
    #[serde(rename = "logType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,

    /// True when the log was removed, due to a chain reorganization.
    /// false if it's a valid log.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
}

impl rlp::Encodable for Log {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data.0);
    }
}

impl Into<RawLog> for Log {
    fn into(self) -> RawLog {
        let data: Vec<u8> = self.data.iter().map(|bytes| u8::from_be(*bytes)).collect();
        let topics: Vec<H256> = self.topics;
        let raw: (Vec<H256>, Vec<u8>) = (topics, data);
        RawLog::from(raw)
    }
}

// TODO: Implement more common types - or adjust this to work with all Tokenizable items

#[cfg(test)]
mod tests {
    use super::Log;
    use ethabi::RawLog;

    #[test]
    fn log_to_rawlog() {
        let topic0: &str = "0x0559884fd3a460db3073b7fc896cc77986f16e378210ded43186175bf646fc5f";
        let topic1: &str = "0x0000000000000000000000000000000000000000000000000000000005f46d7d";
        let topic2: &str = "0x0000000000000000000000000000000000000000000000000000000000002648";
        let data: &str = "0x00000000000000000000000000000000000000000000000000000000637b833f";

        let json: String = format!(
            r#"
        {{
            "address": "0x1d244648d5a63618751d006886268ae3550d0dfd",
            "transactionHash": "0x5ba8546f95bd43c4b0f98e2a7dcecdbe7d15c826dac44f4d61d689f6b8e5dbf2",
            "blockHash": "0x3bbb9ebdd1d2c9b33ba572707d43f26f800e50a873c2a1817ff04551f4cce06d",
            "blockNumber": "0xf46d83",
            "transactionIndex": "0x35",
            "logIndex": "0x40",
            "removed": false,
            "topics": ["{topic0}","{topic1}","{topic2}"],
            "data": "{data}",
        }}
        "#
        );

        let log: Log = serde_json::from_str(&json).unwrap();
        let log_data: Vec<u8> = log.data.iter().map(|b| b.clone()).collect();
        let raw_log: RawLog = log.into();
        assert_eq!(format!("{:?}", raw_log.topics[0]), topic0);
        assert_eq!(format!("{:?}", raw_log.topics[1]), topic1);
        assert_eq!(format!("{:?}", raw_log.topics[2]), topic2);
        assert_eq!(log_data, raw_log.data);
    }
}
