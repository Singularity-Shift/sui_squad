use async_trait::async_trait;
use crate::error::CoreError;

/// Type aliases for addresses, tokens, and transaction hashes.
pub type Address = String;
pub type Token = String;
pub type TxHash = String;

/// Trait defining blockchain gateway methods.
#[async_trait]
pub trait SuiGateway: Send + Sync + Clone + 'static {
    async fn new_wallet(&self, tg_user_id: i64) -> Result<Address, CoreError>;
    async fn balance_of(&self, addr: &Address, token: Token) -> Result<u64, CoreError>;
    async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u64,
        token: Token,
    ) -> Result<TxHash, CoreError>;
    async fn multi_pay(
        &self,
        from: &Address,
        outs: &[(Address, u64)],
        token: Token,
    ) -> Result<TxHash, CoreError>;
}

/// Dummy implementation of SuiGateway that logs calls.
#[derive(Clone)]
pub struct DummyGateway;

#[async_trait]
impl SuiGateway for DummyGateway {
    async fn new_wallet(&self, tg_user_id: i64) -> Result<Address, CoreError> {
        tracing::info!("Dummy new_wallet for {}", tg_user_id);
        Ok(format!("dummy-addr-{}", tg_user_id))
    }

    async fn balance_of(&self, addr: &Address, token: Token) -> Result<u64, CoreError> {
        tracing::info!("Dummy balance_of {} {}", addr, token);
        Ok(0)
    }

    async fn transfer(
        &self,
        from: &Address,
        to: &Address,
        amount: u64,
        token: Token,
    ) -> Result<TxHash, CoreError> {
        tracing::info!("Dummy transfer {}->{} {} {}", from, to, amount, token);
        Ok("dummy-txhash".to_string())
    }

    async fn multi_pay(
        &self,
        from: &Address,
        outs: &[(Address, u64)],
        token: Token,
    ) -> Result<TxHash, CoreError> {
        tracing::info!("Dummy multi_pay {} {:?} {}", from, outs, token);
        Ok("dummy-multipay-txhash".to_string())
    }
} 