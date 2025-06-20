use {
    crate::ConfigFilter,
    solana_pubkey::Pubkey,
    std::{collections::HashSet, str::FromStr},
};

pub struct Filter {
    pub publish_all_accounts: bool,
    pub program_ignores: HashSet<[u8; 32]>,
    pub account_ignores: HashSet<[u8; 32]>,
    pub program_filters: HashSet<[u8; 32]>,
    pub account_filters: HashSet<[u8; 32]>,
    pub include_vote_transactions: bool,
    pub include_failed_transactions: bool,
    pub update_account_topic: String,
    pub slot_status_topic: String,
    pub transaction_topic: String,

    pub wrap_messages: bool,
}

impl Filter {
    pub fn new(config: &ConfigFilter) -> Self {
        Self {
            publish_all_accounts: config.publish_all_accounts,
            program_ignores: config
                .program_ignores
                .iter()
                .flat_map(|p| Pubkey::from_str(p).ok().map(|p| p.to_bytes()))
                .collect(),
            program_filters: config
                .program_filters
                .iter()
                .flat_map(|p| Pubkey::from_str(p).ok().map(|p| p.to_bytes()))
                .collect(),
            account_ignores: config
                .account_ignores
                .iter()
                .flat_map(|p| Pubkey::from_str(p).ok().map(|p| p.to_bytes()))
                .collect(),
            account_filters: config
                .account_filters
                .iter()
                .flat_map(|p| Pubkey::from_str(p).ok().map(|p| p.to_bytes()))
                .collect(),
            include_vote_transactions: config.include_vote_transactions,
            include_failed_transactions: config.include_failed_transactions,

            update_account_topic: config.update_account_topic.clone(),
            slot_status_topic: config.slot_status_topic.clone(),
            transaction_topic: config.transaction_topic.clone(),

            wrap_messages: config.wrap_messages,
        }
    }

    pub fn wants_program(&self, program: &[u8]) -> bool {
        match <&[u8; 32]>::try_from(program) {
            Ok(key) => {
                !self.program_ignores.contains(key)
                    && (self.program_filters.is_empty() || self.program_filters.contains(key))
            }
            Err(_error) => self.program_filters.is_empty(),
        }
    }

    pub fn wants_account(&self, account: &[u8]) -> bool {
        match <&[u8; 32]>::try_from(account) {
            Ok(key) => {
                !self.account_ignores.contains(key)
                    && (self.account_filters.is_empty() || self.account_filters.contains(key))
            }
            Err(_error) => self.account_filters.is_empty(),
        }
    }

    pub fn wants_vote_tx(&self) -> bool {
        self.include_vote_transactions
    }

    pub fn wants_failed_tx(&self) -> bool {
        self.include_failed_transactions
    }

    pub fn has_account_topic(&self) -> bool {
        !self.update_account_topic.is_empty()
    }

    pub fn has_slot_topic(&self) -> bool {
        !self.slot_status_topic.is_empty()
    }

    pub fn has_transaction_topic(&self) -> bool {
        !self.transaction_topic.is_empty()
    }
}
