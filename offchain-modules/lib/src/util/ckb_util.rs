use crate::util::settings::{OutpointConf, Settings};
use anyhow::Result;
use ckb_sdk::{Address, GenesisInfo, HttpRpcClient};
use ckb_types::core::{BlockView, DepType, TransactionView};
// use ckb_types::packed::WitnessArgs;
use ckb_types::packed::HeaderVec;
use ckb_types::prelude::{Builder, Entity, Pack};
use ckb_types::{
    bytes::Bytes,
    packed::{self, Byte32, CellDep, CellOutput, OutPoint, Script},
    H256,
};
use ethereum_types::H160;
use faster_hex::hex_decode;
use force_sdk::cell_collector::{get_live_cell_by_lockscript, get_live_cell_by_typescript};
use force_sdk::indexer::IndexerRpcClient;
use force_sdk::tx_helper::TxHelper;
use force_sdk::util::{get_live_cell, get_live_cell_with_cache};
use std::collections::HashMap;
use std::str::FromStr;

pub fn make_ckb_transaction(_from_lockscript: Script) -> Result<TransactionView> {
    todo!()
}

pub struct Generator {
    pub rpc_client: HttpRpcClient,
    pub indexer_client: IndexerRpcClient,
    _genesis_info: GenesisInfo,
    pub settings: Settings,
}

impl Generator {
    pub fn new(rpc_url: String, indexer_url: String, settings: Settings) -> Result<Self, String> {
        let mut rpc_client = HttpRpcClient::new(rpc_url);
        let indexer_client = IndexerRpcClient::new(indexer_url);
        let genesis_block: BlockView = rpc_client
            .get_block_by_number(0)?
            .expect("Can not get genesis block?")
            .into();
        let genesis_info = GenesisInfo::from_block(&genesis_block)?;
        Ok(Self {
            rpc_client,
            indexer_client,
            _genesis_info: genesis_info,
            settings,
        })
    }

    #[allow(clippy::mutable_key_type)]
    pub fn generate_eth_spv_tx(
        &mut self,
        from_lockscript: Script,
        eth_proof: &ETHSPVProofJson,
    ) -> Result<TransactionView, String> {
        let tx_fee: u64 = 10000;
        let mut helper = TxHelper::default();

        let lockscript = Script::new_builder()
            .code_hash(Byte32::from_slice(&self.settings.lockscript.code_hash.as_bytes()).unwrap())
            .hash_type(DepType::Code.into())
            // FIXME: add script args
            .args(ckb_types::packed::Bytes::default())
            .build();

        // input bridge cells
        {
            let rpc_client = &mut self.rpc_client;
            let indexer_client = &mut self.indexer_client;
            let mut live_cell_cache: HashMap<(OutPoint, bool), (CellOutput, Bytes)> =
                Default::default();
            let mut get_live_cell_fn = |out_point: OutPoint, with_data: bool| {
                get_live_cell_with_cache(&mut live_cell_cache, rpc_client, out_point, with_data)
                    .map(|(output, _)| output)
            };
            let cell = get_live_cell_by_lockscript(indexer_client, lockscript.clone())?;
            match cell {
                Some(cell) => {
                    helper.add_input(
                        OutPoint::from(cell.out_point),
                        None,
                        &mut get_live_cell_fn,
                        &self._genesis_info,
                        true,
                    )?;
                }
                None => {
                    return Err("the bridge cell is not found.".to_string());
                }
            }
        }

        // 1 bridge cells
        {
            let to_output = CellOutput::new_builder().lock(lockscript).build();
            helper.add_output_with_auto_capacity(to_output, ckb_types::bytes::Bytes::default());
        }

        // 2 xt cells
        {
            let recipient_lockscript = Script::from(
                Address::from_str(&eth_proof.ckb_recipient)
                    .unwrap()
                    .payload(),
            );

            let sudt_typescript_code_hash = hex::decode(&self.settings.sudt.code_hash)
                .expect("wrong sudt_script code hash config");
            let sudt_typescript = Script::new_builder()
                .code_hash(Byte32::from_slice(&sudt_typescript_code_hash).unwrap())
                .hash_type(DepType::Code.into())
                // FIXME: add script args
                .args(ckb_types::packed::Bytes::default())
                .build();

            let sudt_user_output = CellOutput::new_builder()
                .type_(Some(sudt_typescript).pack())
                .lock(recipient_lockscript)
                .build();

            let to_user_amount_data = eth_proof.lock_amount.to_le_bytes().to_vec().into();
            helper.add_output_with_auto_capacity(sudt_user_output, to_user_amount_data);
        }

        //FIXME: Wait for the work on the contract side to complete
        // add witness
        {
            // let witness_data = Default::default();
            // let witness = WitnessArgs::new_builder()
            //     .input_type(Some(witness_data.as_bytes()).pack())
            //     .build();
            //
            // helper.transaction = helper
            //     .transaction
            //     .as_advanced_builder()
            //     .set_witnesses(vec![witness.as_bytes().pack()])
            //     .build();
        }

        // build tx
        let tx = helper.supply_capacity(
            &mut self.rpc_client,
            &mut self.indexer_client,
            from_lockscript,
            &self._genesis_info,
            tx_fee,
        )?;
        Ok(tx)
        // Ok(TransactionView::)
    }

    fn _add_cell_deps(
        &mut self,
        helper: &mut TxHelper,
        outpoints: Vec<OutpointConf>,
    ) -> Result<(), String> {
        let mut builder = helper.transaction.as_advanced_builder();
        for conf in outpoints {
            let outpoint = OutPoint::new_builder()
                .tx_hash(
                    Byte32::from_slice(
                        &hex::decode(conf.tx_hash)
                            .map_err(|e| format!("invalid OutpointConf config. err: {}", e))?,
                    )
                    .map_err(|e| format!("invalid OutpointConf config. err: {}", e))?,
                )
                .index(conf.index.pack())
                .build();
            builder = builder.cell_dep(
                CellDep::new_builder()
                    .out_point(outpoint)
                    .dep_type(DepType::Code.into())
                    .build(),
            );
        }
        helper.transaction = builder.build();
        Ok(())
    }

    fn _get_ckb_cell(
        &mut self,
        helper: &mut TxHelper,
        cell_typescript: Script,
        add_to_input: bool,
    ) -> Result<(CellOutput, Bytes), String> {
        let genesis_info = self._genesis_info.clone();
        let cell = get_live_cell_by_typescript(&mut self.indexer_client, cell_typescript)?
            .ok_or("cell not found")?;
        let ckb_cell = CellOutput::from(cell.output);
        let ckb_cell_data = packed::Bytes::from(cell.output_data).raw_data();
        if add_to_input {
            let mut get_live_cell_fn = |out_point: OutPoint, with_data: bool| {
                get_live_cell(&mut self.rpc_client, out_point, with_data).map(|(output, _)| output)
            };

            helper.add_input(
                cell.out_point.into(),
                None,
                &mut get_live_cell_fn,
                &genesis_info,
                true,
            )?;
        }
        Ok((ckb_cell, ckb_cell_data))
    }
    pub fn get_ckb_headers(&mut self, block_numbers: Vec<u64>) -> Result<Vec<u8>> {
        let mut mol_header_vec: Vec<packed::Header> = Default::default();
        for number in block_numbers {
            let block_opt = self
                .rpc_client
                .get_block_by_number(number)
                .map_err(|e| anyhow::anyhow!("failed to get block: {}", e))?;

            if let Some(block) = block_opt {
                mol_header_vec.push(block.header.inner.into());
            }
        }
        let mol_headers = HeaderVec::new_builder().set(mol_header_vec).build();
        Ok(Vec::from(mol_headers.as_slice()))
    }
}

pub fn covert_to_h256(mut tx_hash: &str) -> Result<H256> {
    if tx_hash.starts_with("0x") || tx_hash.starts_with("0X") {
        tx_hash = &tx_hash[2..];
    }
    if tx_hash.len() % 2 != 0 {
        anyhow::bail!(format!("Invalid hex string length: {}", tx_hash.len()))
    }
    let mut bytes = vec![0u8; tx_hash.len() / 2];
    hex_decode(tx_hash.as_bytes(), &mut bytes)
        .map_err(|err| anyhow::anyhow!("parse hex string failed: {:?}", err))?;
    H256::from_slice(&bytes).map_err(|e| anyhow::anyhow!("failed to covert tx hash: {}", e))
}

#[derive(Default, Debug, Clone)]
pub struct ETHSPVProofJson {
    pub log_index: u64,
    pub log_entry_data: String,
    pub receipt_index: u64,
    pub receipt_data: String,
    pub header_data: String,
    pub proof: Vec<Vec<u8>>,
    pub token: H160,
    pub lock_amount: u128,
    pub ckb_recipient: String,
}
