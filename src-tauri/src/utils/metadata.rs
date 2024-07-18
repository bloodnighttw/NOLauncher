#![allow(dead_code)]

use reginleif::metadata::client::package::{PackageDetails, PackageList};
use reginleif::metadata::client::version::VersionDetails;
use crate::utils::base_store::MetadataStorePoint;

pub type NLPackageList = PackageList<MetadataStorePoint>;
pub type NLPackageDetails = PackageDetails<MetadataStorePoint>;
pub type NLVersionDetails = VersionDetails<MetadataStorePoint>;
