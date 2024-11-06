use std::collections::BTreeMap;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug, Default)]
pub struct PaymentDetails {
    pub booking_id: String,
    pub payment_status: PaymentStatus,
}

impl PaymentDetails {
    pub fn new(booking_id: String) -> Self {
        Self {
            booking_id,
            payment_status: PaymentStatus::Unpaid(None),
        }
    }

    pub fn process_payment(&mut self, transaction_ref: String) -> Result<(), String> {
        self.payment_status = PaymentStatus::Paid(transaction_ref);
        Ok(())
    }

    pub fn mark_payment_failed(&mut self, error: String) {
        self.payment_status = PaymentStatus::Unpaid(Some(error));
    }

    pub fn get_status_display(&self) -> String {
        match &self.payment_status {
            PaymentStatus::Paid(ref_no) => format!("Payment confirmed (Ref: {})", ref_no),
            PaymentStatus::Unpaid(None) => "Awaiting payment".to_string(),
            PaymentStatus::Unpaid(Some(error)) => format!("Payment failed: {}", error),
        }
    }

    pub fn is_paid(&self) -> bool {
        matches!(self.payment_status, PaymentStatus::Paid(_))
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum PaymentStatus {
    /// transaction reference number from payments provider
    Paid(String),
    /// if the transaction failed, that would be here.
    Unpaid(Option<String>),
}

impl Default for PaymentStatus {
    fn default() -> Self {
        Self::Unpaid(None)
    }
}



