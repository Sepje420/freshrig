//! macOS app catalog placeholder. The real version will map internal app IDs
//! to Homebrew formulae / casks and mas (Mac App Store) ids. Kept empty for
//! now — `apps::get_app_catalog` returns an `Err` stub instead of an empty
//! catalog so the frontend surfaces the "coming soon" message rather than a
//! blank app list.

#![allow(dead_code)]
