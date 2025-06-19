use crate::config::ConfigFilter;
use solana_pubkey::Pubkey;

const PUBKEY_SIZE: usize = 32;

pub struct Filter {
    pub update_account_topic: String,
    pub slot_status_topic: String,
    pub transaction_topic: String,
    program_ignores: Vec<Pubkey>,
    program_filters: Vec<Pubkey>,
    account_filters: Vec<Pubkey>,
    pub publish_all_accounts: bool,
    pub include_vote_transactions: bool,
    pub include_failed_transactions: bool,
    pub wrap_messages: bool,
}

impl Filter {
    pub fn new(config: &ConfigFilter) -> Self {
        Self {
            update_account_topic: config.update_account_topic.clone(),
            slot_status_topic: config.slot_status_topic.clone(),
            transaction_topic: config.transaction_topic.clone(),
            program_ignores: Self::parse_pubkeys_to_vec(&config.program_ignores),
            program_filters: Self::parse_pubkeys_to_vec(&config.program_filters),
            account_filters: Self::parse_pubkeys_to_vec(&config.account_filters),
            publish_all_accounts: config.publish_all_accounts,
            include_vote_transactions: config.include_vote_transactions,
            include_failed_transactions: config.include_failed_transactions,
            wrap_messages: config.wrap_messages,
        }
    }

    pub fn wants_program(&self, program_id_bytes: &[u8]) -> bool {
        if program_id_bytes.len() != PUBKEY_SIZE {
            return false; // Invalid pubkey length
        }
        
        let program_id_array: &[u8; PUBKEY_SIZE] = match program_id_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => return false, // Not a 32-byte pubkey, should not happen if len checked
        };
        let program_id = Pubkey::new_from_array(*program_id_array);

        if self.program_ignores.contains(&program_id) {
            return false;
        }

        if !self.program_filters.is_empty() {
            return self.program_filters.contains(&program_id);
        }

        true
    }

    pub fn wants_account(&self, account_id_bytes: &[u8]) -> bool {
        if account_id_bytes.len() != PUBKEY_SIZE {
            return false;
        }
        
        let account_id_array: &[u8; PUBKEY_SIZE] = match account_id_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };
        let account_id = Pubkey::new_from_array(*account_id_array);

        if !self.account_filters.is_empty() {
            return self.account_filters.contains(&account_id);
        }

        true
    }

    #[inline(always)]
    pub fn wants_vote_tx(&self) -> bool {
        self.include_vote_transactions
    }

    #[inline(always)]
    pub fn wants_failed_tx(&self) -> bool {
        self.include_failed_transactions
    }

    fn parse_pubkeys_to_vec(keys: &[String]) -> Vec<Pubkey> {
        keys.iter()
            .filter_map(|s| match s.parse::<Pubkey>() {
                Ok(pk) => Some(pk),
                Err(e) => {
                    log::error!("Failed to parse Pubkey '{}': {}", s, e);
                    None
                }
            })
            .collect()
    }
}