use std::io::ErrorKind;
use std::num::ParseIntError;
use std::path::{PathBuf};

use anyhow::Result;
use thiserror::Error;
use crate::utils::data::{TimeSensitiveData};
use crate::utils::minecraft::metadata::{PackageList};
use sha1::{Digest as d1, Sha1};
use sha2::{Digest, Sha256};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct Metadata{
    pub api_path:String,
    pub cached_path: PathBuf,
    pub package_list: TimeSensitiveData<PackageList>
}

#[derive(Error, Debug, PartialEq)]
pub enum MetadataFileError{
    #[error("the cached file is not found")]
    IO(ErrorKind),
    #[error("the cached file is not found")]
    Invalid,
}

pub enum SHAType{
    SHA1(Vec<u8>),
    SHA256(Vec<u8>)
}


impl Metadata{

    /// # How we handle cached file
    /// First we will have [PackageList] in [Metadata], we will fetch data
    /// from [api_path], which contain package info.
    /// Then we will check [PackageDetails] is in [cached_path] folder and sha of [PackageDetails],
    /// if not download or re-download it from [api_path].
    /// [VersionDetails] is also work like this, so this is the function to handle it.
    pub async fn get_cached_file_content(&self,path:PathBuf, sha:SHAType) -> Result<String, MetadataFileError> {
        let i = tokio::fs::read_to_string(path).await;

        let content = match i {
            Ok(content) => {content}
            Err(e) =>{
               return Err(MetadataFileError::IO(e.kind()))
            }
        };

        match sha {
            SHAType::SHA1(sha1) => {
                assert_eq!(sha1.len(),20);
                let mut hasher = Sha1::new();
                sha1::digest::Update::update(&mut hasher, &content.clone().as_bytes());
                let result = &hasher.finalize()[..];
                if &sha1 != result{
                    return Err(MetadataFileError::Invalid)
                }
            }
            SHAType::SHA256(sha256) => {
                assert_eq!(sha256.len(),32);
                let mut hasher = Sha256::new();
                hasher.update(&content.clone().as_bytes());
                let result = &hasher.finalize()[..];
                if &sha256 != result{
                    return Err(MetadataFileError::Invalid)
                }
            }
        }

        Ok(content)

    }
}


#[cfg(test)]
mod test{
    use std::{env, fs};
    use std::io::ErrorKind;
    use crate::command::metadata::{decode_hex, Metadata, MetadataFileError};
    use crate::command::metadata::SHAType::{SHA1, SHA256};
    use crate::utils::data::TimeSensitiveData;
    use crate::utils::minecraft::metadata::PackageList;

    #[tokio::test]
    async fn test_metadata_get_cached_file_content_not_found(){
        let path = env::current_dir().unwrap();
        let package_list = PackageList{
            format_version: 0,
            packages: Default::default(),
        };
        let metadata = Metadata{
            api_path: "".to_string(),
            cached_path: Default::default(),
            package_list: TimeSensitiveData::new(
                package_list
            )
        };

        let test_sha1 = "a6b0f24b706870b0e0c1813f0805850a2a2988bf";
        let decode_from_hex = decode_hex(test_sha1).unwrap();

        let test = metadata.get_cached_file_content(path.join("not found"), SHA1(decode_from_hex)).await;
        let data = Err(MetadataFileError::IO(ErrorKind::NotFound));
        assert_eq!(test,data)
    }

    #[tokio::test]
    async fn test_metadata_get_cached_file_content_sha1(){
        let path = env::current_dir().unwrap();
        let package_list = PackageList{
            format_version: 0,
            packages: Default::default(),
        };
        let metadata = Metadata{
            api_path: "".to_string(),
            cached_path: Default::default(),
            package_list: TimeSensitiveData::new(
                package_list
            )
        };

        let test_str = "it's is a str";
        let test_sha1 = "a6b0f24b706870b0e0c1813f0805850a2a2988bf";

        tokio::fs::write(path.join("test1.txt"),test_str.to_string()).await.unwrap();
        let decode_from_hex = decode_hex(test_sha1).unwrap();
        let _ = metadata.get_cached_file_content(path.join("test1.txt"), SHA1(decode_from_hex)).await.unwrap();
        
        fs::remove_file(path.join("test1.txt")).unwrap();
    }

    #[tokio::test]
    async fn test_metadata_get_cached_file_content_sha256(){
        let path = env::current_dir().unwrap();
        let package_list = PackageList{
            format_version: 0,
            packages: Default::default(),
        };
        let metadata = Metadata{
            api_path: "".to_string(),
            cached_path: Default::default(),
            package_list: TimeSensitiveData::new(
                package_list
            )
        };

        let test_str = "it's is a str";
        let test_sha1 = "5541938b005426931bd062af5b23d0ed69ca5cf577ae80dc441e3d1d7f38c072";

        tokio::fs::write(path.join("test2.txt"),test_str.to_string()).await.unwrap();
        let decode_from_hex = decode_hex(test_sha1).unwrap();
        let _ = metadata.get_cached_file_content(path.join("test2.txt"), SHA256(decode_from_hex)).await.unwrap();

        fs::remove_file(path.join("test2.txt")).unwrap();
    }

}
