use soroban_sdk::{Address, Env};

use crate::{events, DataKey};

/// Returns the referrer for a given subscriber, if one was recorded.
pub fn get_referrer(env: &Env, user: &Address) -> Option<Address> {
    env.storage()
        .persistent()
        .get(&DataKey::Referral(user.clone()))
}

/// Stores the referrer for a subscriber. No-op if referrer is None.
pub fn store_referral(env: &Env, user: &Address, referrer: &Option<Address>) {
    if let Some(ref r) = referrer {
        env.storage()
            .persistent()
            .set(&DataKey::Referral(user.clone()), r);

        events::publish_referred(env, user, r);
    }
}
