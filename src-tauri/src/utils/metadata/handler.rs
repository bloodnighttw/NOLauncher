//! this file handle metadata of each package
use core::panic;
use std::{ops::DerefMut, path::PathBuf, time::Duration};

use async_recursion::async_recursion;
use reginleif::metadata::client::{library::Platform, package::PackageList};
use reginleif_macro::{NoRefresh, Storage};
use reginleif_utils::{expiring_data::{Expirable, ExpiringData}, save_path::{BaseStorePoint, Store}};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::utils::base_store::MetadataStorePoint;
use anyhow::Result;

#[derive(NoRefresh,Serialize,Deserialize,Debug,Clone)]
struct InnerTimer;

impl Expirable for InnerTimer {    
    fn get_duration(&self) -> std::time::Duration {
        Duration::from_secs(3600)
    }
}

#[derive(Serialize,Deserialize,Debug,Clone,Storage)]
#[base_on(MetadataStorePoint)] #[filepath(&["timer.json"])]
struct Timer{
    inner:ExpiringData<InnerTimer>
}


pub type NLPackageList = PackageList<MetadataStorePoint>;

/// mutex is to ensure only on command update package list at the same time
/// timer is to ensure that data need to refetch or not.
pub struct PackageListHandler(Mutex<Timer>,MetadataStorePoint);

impl From<MetadataStorePoint> for PackageListHandler{

    fn from(base:MetadataStorePoint) -> Self{
        
        let timer = match Timer::load(&base){
            Ok(config) => config,
            Err(_) => {
                Timer{
                    inner:ExpiringData::from(InnerTimer)
                }
            }
        };

        Self(Mutex::from(timer), base)

    }
}

#[derive(Debug)]
pub enum FileType{
    Client,
    Installer, // for forge and neoforge
    Asset,
    Library,
}

#[derive(Debug)]
pub struct DownloadInfo{
    pub url:String,
    pub to:PathBuf,
    pub file_type:FileType,
}


impl PackageListHandler {
    pub async fn fetch(&self,client:Client,url:&str) -> Result<NLPackageList>{

        let mut _lock = self.0.lock().await;
        let timer = _lock.deref_mut();

        timer.save(&self.1)?; 
        let temp = if timer.inner.is_expired(){
            timer.inner = ExpiringData::from(InnerTimer); // reset timer
            PackageList::refresh(&self.1,client,url).await?
        }else{
            PackageList::fetch(&self.1,client,url).await?
        };

        Ok(temp)
    }

    pub async fn refresh(&self,client:Client,url:&str) -> Result<NLPackageList>{
        let mut _lock = self.0.lock().await;
        let timer = _lock.deref_mut();

        timer.inner = ExpiringData::from(InnerTimer); // reset timer
        timer.save(&self.1)?;

        PackageList::refresh(&self.1,client,url).await
    }

    pub async fn download(&self,client:Client,url:&str,mc_version:&String,version:&String,uid:&String) -> Result<Vec<DownloadInfo>>{
        let root = self.fetch(client.clone() , url).await?;
        self.dep(client.clone(), url, root, uid, version, mc_version).await
    }

    #[async_recursion]
    async fn dep(&self,client:Client,url:&str,pkgs:PackageList<MetadataStorePoint>,uid:&String,version:&String,mc_version:&String) -> Result<Vec<DownloadInfo>>{


        let pkg = pkgs.iter().find(|x| x.uid == uid.as_str()).ok_or(anyhow::anyhow!("Package not found"))?;
        let p_details = pkg.get_details(&self.1, client.clone(), url).await?;
        let v_info = p_details.iter().find(|x| x.version == version.as_str()).ok_or(anyhow::anyhow!("Version not found"))?;
        let v_details = v_info.get_details(&self.1, client.clone(), url, uid).await?;

        let mut data = vec![];

        for i in v_details.requires.iter(){
            let suggest = i.suggests.as_ref();
            let version = i.equals.as_ref().unwrap_or(suggest.unwrap_or(mc_version));
            let d = self.dep(client.clone(),url,pkgs.clone(),&i.uid,version,mc_version).await?;
            data.extend(d);
        }

        println!("{} {} {}",uid,version,mc_version);

        let run = |name:String| -> Result<_>{

            let spilt = name.as_str().split(':').collect::<Vec<&str>>();
            
            let orgs = spilt.first().ok_or(anyhow::anyhow!("Invalid library name"))?.to_string();
            let pkg = spilt.get(1).ok_or(anyhow::anyhow!("Invalid library name"))?.to_string();
            let ver = spilt.get(2).ok_or(anyhow::anyhow!("Invalid library name"))?.to_string();
            let rtype = spilt.get(3).map(|s| s.to_string());
            
            let mut base = self.1.get_base();
            orgs.split('/').for_each(|x|{
                base = base.join(x);
            });  
            base = base.join(format!("{}-{}.jar",pkg,ver));
            
            Ok((base,rtype,orgs,pkg,ver))
        };
        
        let me = Platform::me();

        for i in v_details.libraries{

            match i {
                reginleif::metadata::client::library::Library::Common(c) => {

                    if !me.allow_rule(c.rules){
                        continue; // skip package since it's not for this platform
                    }

                    let (base,_,_,_,_) = run(c.name)?;
                    
                    if let Some(artifact) = c.downloads.artifact {

                        data.push(DownloadInfo{
                            url:artifact.url,
                            to:base,
                            file_type:FileType::Library
                        });
                    }

                    //TODO handle classifier
                },
                reginleif::metadata::client::library::Library::Maven(m) => {

                    let (base,_,orgs,pkg,ver) = run(m.name)?;

                    data.push(
                        DownloadInfo{
                            url:format!("{}/{}/{pkg}/{ver}/{pkg}-{ver}.jar",m.url,orgs.replace([':', '.'], "/")),
                            to:base,
                            file_type:FileType::Library
                        }
                    )

                },
            } 
        }

        for i in v_details.maven_files{
            match i {
                reginleif::metadata::client::library::Library::Common(c) =>{


                    if !me.allow_rule(c.rules){
                        continue; // skip package since it's not for this platform
                    }

                    let (base,rtype,_,_,_) = run(c.name)?;

                    let file_type = match rtype.as_deref(){
                        Some("installer") => FileType::Installer,
                        _ => FileType::Library
                    };
                    

                    if let Some(artifact) = c.downloads.artifact{
                        data.push(DownloadInfo{
                            url:artifact.url,
                            to:base,
                            file_type
                        });
                    }
                },
                reginleif::metadata::client::library::Library::Maven(_) => {},
            }
                
        }

        if let Some(client) = v_details.main_jar{
            let (base,_,_,_,_) = run(client.name)?;
            // let base_url = base.to_str();

            data.push(DownloadInfo{
                url:client.downloads.artifact.unwrap().url,
                to:base,
                file_type:FileType::Client
            });
        }

        Ok(data)
    }

}
