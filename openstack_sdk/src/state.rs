//! SDK State
//!
//! A session object keeps certain information that may need to be cached. This module implements caching of the authentication/authorization information with certain functionality to manage cache data.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::PathBuf;
use tracing::{trace, warn};

use crate::auth::{AuthState, AuthToken, AuthorizationScope};
use thiserror::Error;

/// Errors which may occur when creating connection state data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StateError {
    #[error("failed to deserialize config: {}", source)]
    Parse {
        /// The source of the error.
        #[from]
        source: config::ConfigError,
    },
    #[error("IO error: {}", source)]
    IO {
        /// The source of the error.
        #[from]
        source: std::io::Error,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error), // source and Display delegate to anyhow::Error
}

#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub(crate) struct ScopeAuths(HashMap<AuthorizationScope, AuthToken>);

impl ScopeAuths {
    /// Filter out all invalid auth data keeping only valid ones
    fn filter_invalid_auths(&mut self) -> &mut Self {
        self.0.retain(|_, v| AuthState::Valid == v.get_state());
        self
    }

    /// Find valid unscoped authz
    fn find_valid_unscoped_auth(&self) -> Option<AuthToken> {
        for (k, v) in self.0.iter() {
            if let AuthorizationScope::Unscoped = k {
                if let AuthState::Valid = v.get_state() {
                    return Some(v.clone());
                }
            }
        }
        None
    }

    /// Find first matching unscoped authz
    fn find_first_valid_auth(&self) -> Option<AuthToken> {
        for (_, v) in self.0.iter() {
            if let AuthState::Valid = v.get_state() {
                return Some(v.clone());
            }
        }
        None
    }
}

/// OSC state
#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct State {
    /// Auth/Authz state
    auth_state: ScopeAuths,
    base_dir: PathBuf,
    auth_hash: u64,
    auth_cache_enabled: bool,
}

impl State {
    pub fn new() -> Self {
        let state = Self {
            auth_hash: 0,
            auth_state: Default::default(),
            auth_cache_enabled: false,
            base_dir: dirs::home_dir()
                .expect("Cannot determine users XDG_HOME")
                .join(".osc"),
        };
        DirBuilder::new()
            .recursive(true)
            .create(state.base_dir.clone())
            .ok();
        state
    }
    /// Set the unique authentication hash key
    pub fn set_auth_hash_key(&mut self, auth_hash: u64) -> &mut Self {
        self.auth_hash = auth_hash;
        self
    }

    /// Enable/Disable authorization caching
    pub fn enable_auth_cache(&mut self, val: bool) -> &mut Self {
        self.auth_cache_enabled = val;
        self
    }

    /// Set authz into the state
    pub fn set_scope_auth(&mut self, scope: &AuthorizationScope, authz: &AuthToken) {
        self.auth_state.filter_invalid_auths();
        self.auth_state.0.insert(scope.clone(), authz.clone());
        if self.auth_cache_enabled {
            self.save_scope_auth_to_file(scope, authz);
        }
    }

    /// Get authz for requested scope from the state
    pub fn get_scope_auth(&mut self, scope: &AuthorizationScope) -> Option<AuthToken> {
        trace!("Get authz information for {:?}", scope);
        self.auth_state.filter_invalid_auths();
        match self.auth_state.0.get(scope) {
            Some(authz) => Some(authz.clone()),
            None => {
                if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled) {
                    if let Some((scope, authz)) = self.find_scope_authz(&state, scope) {
                        trace!("Found valid authz in the state file");
                        self.auth_state.0.insert(scope, authz.clone());
                        return Some(authz);
                    }
                }
                None
            }
        }
    }

    pub fn find_valid_auth(&self, state: &ScopeAuths) -> Option<AuthToken> {
        if let Some(unscoped) = state.find_valid_unscoped_auth() {
            return Some(unscoped);
        }
        if let Some(scoped) = state.find_first_valid_auth() {
            return Some(scoped);
        }
        None
    }

    pub fn get_any_valid_auth(&mut self) -> Option<AuthToken> {
        if let Some(auth) = self.find_valid_auth(&self.auth_state) {
            return Some(auth);
        }
        if let (Some(state), true) = (self.load_auth_state(None), self.auth_cache_enabled) {
            if let Some(auth) = self.find_valid_auth(&state) {
                return Some(auth);
            }
        }
        None
    }

    /// Locate authz for requested scope in the state
    fn find_scope_authz(
        &self,
        state: &ScopeAuths,
        scope: &AuthorizationScope,
    ) -> Option<(AuthorizationScope, AuthToken)> {
        trace!("Searching requested scope authz in state");
        for (k, v) in state.0.iter() {
            trace!("Analyse known auth for scope {:?}", k);
            match scope {
                AuthorizationScope::Project(project) => {
                    if let AuthorizationScope::Project(cached) = k {
                        // Scope type matches
                        if project.id == cached.id {
                            // Match by ID is definite
                            return Some((k.clone(), v.clone()));
                        } else if project.name == cached.name {
                            // Match by Name requires verifying domain match
                            if let (Some(requested_domain), Some(state_domain)) =
                                (&project.domain, &cached.domain)
                            {
                                if (requested_domain.id.is_some()
                                    && requested_domain.id == state_domain.id)
                                    || (requested_domain.id.is_none()
                                        && requested_domain.name == state_domain.name)
                                {
                                    return Some((k.clone(), v.clone()));
                                }
                            }
                        }
                    }
                }
                AuthorizationScope::Domain(domain) => {
                    if let AuthorizationScope::Domain(cached) = k {
                        // Scope type matches
                        if domain.id == cached.id
                            || (domain.id.is_none() && domain.name == cached.name)
                        {
                            return Some((k.clone(), v.clone()));
                        }
                    }
                }
                AuthorizationScope::Unscoped => {
                    if let AuthorizationScope::Unscoped = k {
                        return Some((k.clone(), v.clone()));
                    }
                }
            }
        }
        None
    }

    /// Get filename of the auth state file
    fn get_auth_state_filename(&self, auth_hash: u64) -> PathBuf {
        let mut fname_buf = self.base_dir.clone();
        fname_buf.push(auth_hash.to_string());
        fname_buf
    }

    /// Load auth state from the FS
    fn load_auth_state(&self, path: Option<PathBuf>) -> Option<ScopeAuths> {
        let fname = path.unwrap_or(self.get_auth_state_filename(self.auth_hash));
        match File::open(fname.as_path()) {
            Ok(mut file) => {
                let mut contents = vec![];
                match file.read_to_end(&mut contents) {
                    Ok(_) => match bincode::deserialize::<ScopeAuths>(&contents) {
                        Ok(mut auth) => {
                            auth.filter_invalid_auths();
                            trace!("Cached Auth info: {:?}", auth);
                            Some(auth)
                        }
                        Err(x) => {
                            warn!(
                                "Corrupted cache file {}: {:?}. Removing ",
                                fname.display(),
                                x
                            );
                            let _ = std::fs::remove_file(fname);
                            None
                        }
                    },
                    _ => {
                        warn!("Error reading file {}", fname.display());
                        None
                    }
                }
            }
            _ => {
                warn!("Error opening file {}", fname.display());
                None
            }
        }
    }

    /// Save auth state to the FS
    pub fn save_scope_auth_to_file(&self, scope: &AuthorizationScope, data: &AuthToken) {
        let fname = self.get_auth_state_filename(self.auth_hash);
        let mut state = self
            .load_auth_state(Some(fname.clone()))
            .unwrap_or_default();

        let _ = state.0.insert(scope.clone(), data.clone());

        match bincode::serialize(&state) {
            Ok(ser_data) => match File::create(fname.as_path()) {
                Ok(mut file) => {
                    let _ = file.write_all(&ser_data);
                }
                _ => {
                    warn!("Error writing state file");
                }
            },
            Err(e) => {
                warn!("Error serializing state, {:?}", e);
            }
        }
    }
}