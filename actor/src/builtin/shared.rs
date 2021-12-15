// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use std::error::Error as StdError;

use blockstore::Blockstore;
use num_traits::Zero;

use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::encoding::RawBytes;
use fvm_shared::error::ActorError;
use fvm_shared::METHOD_SEND;

use crate::miner::{GetControlAddressesReturn, Method};
use crate::runtime::Runtime;

pub const HAMT_BIT_WIDTH: u32 = 5;

pub(crate) fn request_miner_control_addrs<BS, RT>(
    rt: &mut RT,
    miner_addr: Address,
) -> Result<(Address, Address, Vec<Address>), ActorError>
where
    BS: Blockstore,
    RT: Runtime<BS>,
{
    let ret = rt.send(
        miner_addr,
        Method::ControlAddresses as u64,
        RawBytes::default(),
        TokenAmount::zero(),
    )?;
    let addrs: GetControlAddressesReturn = ret.deserialize()?;

    Ok((addrs.owner, addrs.worker, addrs.control_addresses))
}

/// ResolveToIDAddr resolves the given address to it's ID address form.
/// If an ID address for the given address dosen't exist yet, it tries to create one by sending
/// a zero balance to the given address.
pub(crate) fn resolve_to_id_addr<BS, RT>(
    rt: &mut RT,
    address: &Address,
) -> Result<Address, Box<dyn StdError>>
where
    BS: Blockstore,
    RT: Runtime<BS>,
{
    // if we are able to resolve it to an ID address, return the resolved address
    if let Some(addr) = rt.resolve_address(address)? {
        return Ok(addr);
    }

    // send 0 balance to the account so an ID address for it is created and then try to resolve
    rt.send(
        *address,
        METHOD_SEND,
        Default::default(),
        Default::default(),
    )
    .map_err(|e| {
        e.wrap(&format!(
            "failed to send zero balance to address {}",
            address
        ))
    })?;

    rt.resolve_address(address)?.ok_or_else(|| {
        format!(
            "failed to resolve address {} to ID address even after sending zero balance",
            address,
        )
        .into()
    })
}
