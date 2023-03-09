use ethers::types::TransactionReceipt;

pub trait TransactionReceiptExt: Sized {
    fn status_is_ok(self) -> eyre::Result<Self>;
}
impl TransactionReceiptExt for TransactionReceipt {
    fn status_is_ok(self) -> eyre::Result<Self> {
        let status = self
            .status
            .ok_or_else(|| eyre!("No status found: {self:?}"))?;

        if status != 1.into() {
            Err(eyre!("Transaction status 0: {self:?}"))
        } else {
            Ok(self)
        }
    }
}
