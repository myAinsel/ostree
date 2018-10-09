// This file was generated by gir (https://github.com/gtk-rs/gir @ ffda6f9)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RepoDevInoCache(Shared<ffi::OstreeRepoDevInoCache>);

    match fn {
        ref => |ptr| ffi::ostree_repo_devino_cache_ref(ptr),
        unref => |ptr| ffi::ostree_repo_devino_cache_unref(ptr),
        get_type => || ffi::ostree_repo_devino_cache_get_type(),
    }
}

impl RepoDevInoCache {
    /// OSTree has support for pairing `RepoExt::checkout_tree_at` using
    /// hardlinks in combination with a later
    /// `RepoExt::write_directory_to_mtree` using a (normally modified)
    /// directory. In order for OSTree to optimally detect just the new
    /// files, use this function and fill in the `devino_to_csum_cache`
    /// member of `OstreeRepoCheckoutAtOptions`, then call
    /// `ostree_repo_commit_set_devino_cache`.
    ///
    /// # Returns
    ///
    /// Newly allocated cache
    pub fn new() -> RepoDevInoCache {
        unsafe {
            from_glib_full(ffi::ostree_repo_devino_cache_new())
        }
    }
}

impl Default for RepoDevInoCache {
    fn default() -> Self {
        Self::new()
    }
}
