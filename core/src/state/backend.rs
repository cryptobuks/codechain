// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! A minimal "state backend" trait: an abstraction over the sources of data
//! a blockchain state may draw upon.
//!
//! Currently assumes a very specific DB + cache structure, but
//! should become general over time to the point where not even a
//! merkle trie is strictly necessary.

use ctypes::Address;
use hashdb::{AsHashDB, HashDB};

use super::{Account, Asset, AssetAddress, AssetScheme, AssetSchemeAddress};

/// State backend. See module docs for more details.
pub trait Backend: Send {
    /// Treat the backend as a read-only hashdb.
    fn as_hashdb(&self) -> &HashDB;

    /// Treat the backend as a writeable hashdb.
    fn as_hashdb_mut(&mut self) -> &mut HashDB;

    /// Add an account entry to the cache.
    fn add_to_account_cache(&mut self, addr: Address, data: Option<Account>, modified: bool);
    /// Add an asset entry to the cache.
    fn add_to_asset_scheme_cache(&mut self, addr: AssetSchemeAddress, asset: Option<AssetScheme>, modified: bool);
    /// Add an asset entry to the cache.
    fn add_to_asset_cache(&mut self, addr: AssetAddress, asset: Option<Asset>, modified: bool);

    /// Get basic copy of the cached account. Not required to include storage.
    /// Returns 'None' if cache is disabled or if the account is not cached.
    fn get_cached_account(&self, addr: &Address) -> Option<Option<Account>>;

    fn get_cached_asset_scheme(&self, hash: &AssetSchemeAddress) -> Option<Option<AssetScheme>>;

    fn get_cached_asset(&self, hash: &AssetAddress) -> Option<Option<Asset>>;

    /// Get value from a cached account.
    /// `None` is passed to the closure if the account entry cached
    /// is known not to exist.
    /// `None` is returned if the entry is not cached.
    fn get_cached_account_with<F, U>(&self, a: &Address, f: F) -> Option<U>
    where
        F: FnOnce(Option<&mut Account>) -> U;
}

/// A basic backend. Just wraps the given database, directly inserting into and deleting from
/// it. Doesn't cache anything.
pub struct Basic<H>(pub H);

impl<H: AsHashDB + Send + Sync> Backend for Basic<H> {
    fn as_hashdb(&self) -> &HashDB {
        self.0.as_hashdb()
    }

    fn as_hashdb_mut(&mut self) -> &mut HashDB {
        self.0.as_hashdb_mut()
    }

    fn add_to_account_cache(&mut self, _: Address, _: Option<Account>, _: bool) {}

    fn add_to_asset_scheme_cache(&mut self, _addr: AssetSchemeAddress, _data: Option<AssetScheme>, _: bool) {}

    fn add_to_asset_cache(&mut self, _addr: AssetAddress, _asset: Option<Asset>, _: bool) {}

    fn get_cached_account(&self, _: &Address) -> Option<Option<Account>> {
        None
    }

    fn get_cached_asset_scheme(&self, _addr: &AssetSchemeAddress) -> Option<Option<AssetScheme>> {
        None
    }

    fn get_cached_asset(&self, _addr: &AssetAddress) -> Option<Option<Asset>> {
        None
    }

    fn get_cached_account_with<F, U>(&self, _: &Address, _: F) -> Option<U>
    where
        F: FnOnce(Option<&mut Account>) -> U, {
        None
    }
}
