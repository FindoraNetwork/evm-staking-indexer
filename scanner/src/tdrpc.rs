use crate::error;
use crate::error::ScannerError;
use crate::types::{JsonRpcResponse, RpcBlock, RpcTransaction};
use error::Result;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;
use url::Url;

#[derive(Debug)]
pub struct TendermintRpc {
    url: Url,
    client: Client,
}

impl TendermintRpc {
    pub fn new(url: Url, timeout: Duration) -> Self {
        let client = ClientBuilder::new().timeout(timeout).build().unwrap();
        Self { url, client }
    }

    pub async fn get_block_by_height(&self, height: u64) -> Result<RpcBlock> {
        let mut url = self.url.join("block")?;
        url.set_query(Some(format!("height={}", height).as_str()));
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            let resp_text = resp.text().await?;
            println!("{}", resp_text);
            if resp_text.contains("less than or equal to") {
                return Err(ScannerError::BlockNotFound(height));
            }
            return Err(resp_text.into());
        }

        let bytes = resp.bytes().await?;
        if let Ok(b) = serde_json::from_slice::<JsonRpcResponse<RpcBlock>>(&bytes) {
            Ok(b.result)
        } else {
            Err(ScannerError::BlockNotFound(height))
        }
    }

    pub async fn get_tx_by_hash(&self, tx_hash: &str) -> Result<RpcTransaction> {
        let mut url = self.url.join("tx")?;
        url.set_query(Some(format!("hash=0x{}", tx_hash).as_str()));
        let resp = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            let resp_text = resp.text().await?;
            if resp_text.contains("not found") {
                return Err(ScannerError::TxNotFound(tx_hash.to_string()));
            }
            return Err(resp_text.into());
        }

        let bytes = resp.bytes().await?;

        match serde_json::from_slice::<JsonRpcResponse<RpcTransaction>>(&bytes) {
            Ok(b) => Ok(b.result),
            Err(e) => Err(ScannerError::DeserializeTxError(format!(
                "tx: {}, error: {}",
                tx_hash.to_string(),
                e.to_string()
            ))),
        }

        // if let Ok(b) = serde_json::from_slice::<JsonRpcResponse<RpcTransaction>>(&bytes) {
        //     Ok(b.result)
        // } else {
        //     Err(ScannerError::DeserializeTxError(tx_hash.to_string()))
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_get_block_by_height() -> Result<()> {
        let rpc_url: Url = Url::from_str("https://prod-mainnet.prod.findora.org:26657/").unwrap();
        let rpc = TendermintRpc::new(rpc_url, Duration::from_secs(30));
        let res = rpc.get_block_by_height(10000).await?;

        assert_eq!(
            "04E49912BEE3626453C8DD57220FA7EE5859656CEA1A3F6C372CD1442766DAA5".to_string(),
            res.block.header.app_hash
        );
        assert_eq!(10000.to_string(), res.block.header.height);

        Ok(())
    }

    // #[tokio::test]
    // async fn test_get_tx_by_hash() -> Result<()> {
    //     let rpc_url: Url = Url::from_str("https://prod-mainnet.prod.findora.org:26657/").unwrap();
    //     let rpc = TendermintRpc::new(rpc_url, Duration::from_secs(30));
    //     let res = rpc
    //         .get_tx_by_hash("9c9a598f0464006b70cbbd990de19d16a24b37172bcbafc6b9506948c20d51ed")
    //         .await?;
    //
    //     assert_eq!(
    //         "9C9A598F0464006B70CBBD990DE19D16A24B37172BCBAFC6B9506948C20D51ED",
    //         res.hash
    //     );
    //
    //     Ok(())
    // }
}
