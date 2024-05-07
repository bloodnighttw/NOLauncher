/* the code is edit from https://github.com/KernelFreeze/minecraft-msa-auth
 * Licence is below:
 *
 */
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Apache License
// Version 2.0, January 2004
// http://www.apache.org/licenses/
//
// TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
//
// 1. Definitions.
//
// "License" shall mean the terms and conditions for use, reproduction,
// and distribution as defined by Sections 1 through 9 of this document.
//
// "Licensor" shall mean the copyright owner or entity authorized by
// the copyright owner that is granting the License.
//
// "Legal Entity" shall mean the union of the acting entity and all
// other entities that control, are controlled by, or are under common
// control with that entity. For the purposes of this definition,
// "control" means (i) the power, direct or indirect, to cause the
// direction or management of such entity, whether by contract or
// otherwise, or (ii) ownership of fifty percent (50%) or more of the
// outstanding shares, or (iii) beneficial ownership of such entity.
//
// "You" (or "Your") shall mean an individual or Legal Entity
// exercising permissions granted by this License.
//
// "Source" form shall mean the preferred form for making modifications,
// including but not limited to software source code, documentation
// source, and configuration files.
//
// "Object" form shall mean any form resulting from mechanical
// transformation or translation of a Source form, including but
// not limited to compiled object code, generated documentation,
// and conversions to other media types.
//
// "Work" shall mean the work of authorship, whether in Source or
// Object form, made available under the License, as indicated by a
// copyright notice that is included in or attached to the work
// (an example is provided in the Appendix below).
//
// "Derivative Works" shall mean any work, whether in Source or Object
// form, that is based on (or derived from) the Work and for which the
// editorial revisions, annotations, elaborations, or other modifications
// represent, as a whole, an original work of authorship. For the purposes
// of this License, Derivative Works shall not include works that remain
// separable from, or merely link (or bind by name) to the interfaces of,
// the Work and Derivative Works thereof.
//
// "Contribution" shall mean any work of authorship, including
// the original version of the Work and any modifications or additions
// to that Work or Derivative Works thereof, that is intentionally
// submitted to Licensor for inclusion in the Work by the copyright owner
// or by an individual or Legal Entity authorized to submit on behalf of
// the copyright owner. For the purposes of this definition, "submitted"
// means any form of electronic, verbal, or written communication sent
// to the Licensor or its representatives, including but not limited to
// communication on electronic mailing lists, source code control systems,
// and issue tracking systems that are managed by, or on behalf of, the
// Licensor for the purpose of discussing and improving the Work, but
// excluding communication that is conspicuously marked or otherwise
// designated in writing by the copyright owner as "Not a Contribution."
//
// "Contributor" shall mean Licensor and any individual or Legal Entity
// on behalf of whom a Contribution has been received by Licensor and
// subsequently incorporated within the Work.
//
// 2. Grant of Copyright License. Subject to the terms and conditions of
// this License, each Contributor hereby grants to You a perpetual,
// worldwide, non-exclusive, no-charge, royalty-free, irrevocable
// copyright license to reproduce, prepare Derivative Works of,
// publicly display, publicly perform, sublicense, and distribute the
// Work and such Derivative Works in Source or Object form.
//
// 3. Grant of Patent License. Subject to the terms and conditions of
// this License, each Contributor hereby grants to You a perpetual,
// worldwide, non-exclusive, no-charge, royalty-free, irrevocable
// (except as stated in this section) patent license to make, have made,
// use, offer to sell, sell, import, and otherwise transfer the Work,
// where such license applies only to those patent claims licensable
// by such Contributor that are necessarily infringed by their
// Contribution(s) alone or by combination of their Contribution(s)
// with the Work to which such Contribution(s) was submitted. If You
// institute patent litigation against any entity (including a
// cross-claim or counterclaim in a lawsuit) alleging that the Work
// or a Contribution incorporated within the Work constitutes direct
// or contributory patent infringement, then any patent licenses
// granted to You under this License for that Work shall terminate
// as of the date such litigation is filed.
//
// 4. Redistribution. You may reproduce and distribute copies of the
// Work or Derivative Works thereof in any medium, with or without
// modifications, and in Source or Object form, provided that You
// meet the following conditions:
//
// (a) You must give any other recipients of the Work or
// Derivative Works a copy of this License; and
//
// (b) You must cause any modified files to carry prominent notices
// stating that You changed the files; and
//
// (c) You must retain, in the Source form of any Derivative Works
// that You distribute, all copyright, patent, trademark, and
// attribution notices from the Source form of the Work,
// excluding those notices that do not pertain to any part of
// the Derivative Works; and
//
// (d) If the Work includes a "NOTICE" text file as part of its
// distribution, then any Derivative Works that You distribute must
// include a readable copy of the attribution notices contained
// within such NOTICE file, excluding those notices that do not
// pertain to any part of the Derivative Works, in at least one
// of the following places: within a NOTICE text file distributed
// as part of the Derivative Works; within the Source form or
// documentation, if provided along with the Derivative Works; or,
// within a display generated by the Derivative Works, if and
// wherever such third-party notices normally appear. The contents
// of the NOTICE file are for informational purposes only and
// do not modify the License. You may add Your own attribution
// notices within Derivative Works that You distribute, alongside
// or as an addendum to the NOTICE text from the Work, provided
// that such additional attribution notices cannot be construed
// as modifying the License.
//
// You may add Your own copyright statement to Your modifications and
// may provide additional or different license terms and conditions
// for use, reproduction, or distribution of Your modifications, or
// for any such Derivative Works as a whole, provided Your use,
// reproduction, and distribution of the Work otherwise complies with
// the conditions stated in this License.
//
// 5. Submission of Contributions. Unless You explicitly state otherwise,
// any Contribution intentionally submitted for inclusion in the Work
// by You to the Licensor shall be under the terms and conditions of
// this License, without any additional terms or conditions.
// Notwithstanding the above, nothing herein shall supersede or modify
// the terms of any separate license agreement you may have executed
// with Licensor regarding such Contributions.
//
// 6. Trademarks. This License does not grant permission to use the trade
// names, trademarks, service marks, or product names of the Licensor,
// except as required for reasonable and customary use in describing the
// origin of the Work and reproducing the content of the NOTICE file.
//
// 7. Disclaimer of Warranty. Unless required by applicable law or
// agreed to in writing, Licensor provides the Work (and each
// Contributor provides its Contributions) on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
// implied, including, without limitation, any warranties or conditions
// of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
// PARTICULAR PURPOSE. You are solely responsible for determining the
// appropriateness of using or redistributing the Work and assume any
// risks associated with Your exercise of permissions under this License.
//
// 8. Limitation of Liability. In no event and under no legal theory,
// whether in tort (including negligence), contract, or otherwise,
// unless required by applicable law (such as deliberate and grossly
// negligent acts) or agreed to in writing, shall any Contributor be
// liable to You for damages, including any direct, indirect, special,
// incidental, or consequential damages of any character arising as a
// result of this License or out of the use or inability to use the
// Work (including but not limited to damages for loss of goodwill,
// work stoppage, computer failure or malfunction, or any and all
// other commercial damages or losses), even if such Contributor
// has been advised of the possibility of such damages.
//
// 9. Accepting Warranty or Additional Liability. While redistributing
// the Work or Derivative Works thereof, You may choose to offer,
// and charge a fee for, acceptance of support, warranty, indemnity,
// or other liability obligations and/or rights consistent with this
// License. However, in accepting such obligations, You may act only
// on Your own behalf and on Your sole responsibility, not on behalf
// of any other Contributor, and only if You agree to indemnify,
// defend, and hold each Contributor harmless for any liability
// incurred by, or claims asserted against, such Contributor by reason
// of your accepting any such warranty or additional liability.
//
// END OF TERMS AND CONDITIONS


/*  To gain Minecraft Access Token,You do these thing:
 *      1. Gain the key to access XBOX LIVE API         <--- Not here.
 *      2. Obtain XSTS token for Minecraft              <--- You are here.
 *      3. Authenticate with Minecraft                  <--- and here.
 *
 *  You can find more details on https://wiki.vg/Microsoft_Authentication_Scheme
 */


use std::collections::HashMap;
use std::fmt::{Debug, Display};

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

const MINECRAFT_LOGIN_WITH_XBOX: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
const XBOX_USER_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XBOX_XSTS_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";

/// Represents a Minecraft access token
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct MinecraftAccessToken(String);

impl Debug for MinecraftAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MinecraftAccessToken").field(&"[redacted]").finish()
    }
}

impl Display for MinecraftAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

/// Represents the token type of a Minecraft access token
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum MinecraftTokenType {
    Bearer,
}

/// Represents an error that can occur when authenticating with Minecraft.
#[derive(Error, Debug)]
pub enum MinecraftAuthorizationError {
    /// An error occurred while sending the request
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// Account belongs to a minor who needs to be added to a microsoft family
    #[error("Minor must be added to microsoft family")]
    AddToFamily,

    /// Account does not have xbox, user must create an xbox account to continue
    #[error("Account does not have xbox")]
    NoXbox,

    /// Claims were missing from the response
    #[error("missing claims from response")]
    MissingClaims,
}

/// The response from Minecraft when attempting to authenticate with an xbox
/// token
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MinecraftAuthenticationResponse {
    /// UUID of the Xbox account.
    /// Please note that this is not the Minecraft player's UUID
    username: String,

    /// The minecraft JWT access token
    pub access_token: MinecraftAccessToken,

    /// The type of access token
    pub token_type: MinecraftTokenType,

    /// How many seconds until the token expires
    expires_in: u32,
}

/// The response from Xbox when authenticating with a Microsoft token
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveAuthenticationResponse {
    /// The xbox authentication token to use
    token: String,

    /// An object that contains a vec of `uhs` objects
    /// Looks like { "xui": [{"uhs": "xbl_token"}] }
    display_claims: HashMap<String, Vec<HashMap<String, String>>>,
}

/// The error response from Xbox when authenticating with a Microsoft token
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxLiveAuthenticationResponseError {
    /// Always zero
    identity: String,

    /// Error id
    /// 2148916238 means <18 and needs to be added to microsoft family
    /// 2148916233 means xbox account needs to be created
    x_err: i64,

    /// Message about error
    message: String,

    /// Where to go to fix the error as a user
    redirect: String,
}

/// The flow for authenticating with a Microsoft access token and getting a
/// Minecraft access token.
pub struct MinecraftAuthorizationFlow {
    http_client: Client,
}

impl MinecraftAuthorizationFlow {
    /// Creates a new [MinecraftAuthorizationFlow] using the given
    /// [Client].
    pub const fn new(http_client: Client) -> Self {
        Self { http_client }
    }

    /// Authenticates with the Microsoft identity platform using the given
    /// Microsoft access token and returns a [MinecraftAuthenticationResponse]
    /// that contains the Minecraft access token.
    #[tokio::main]
    pub async fn exchange_microsoft_token(
        &self, user_hash: String,xbox_security_token:XboxLiveAuthenticationResponse
    ) -> Result<MinecraftAuthenticationResponse, MinecraftAuthorizationError> {

        let response = self
            .http_client
            .post(MINECRAFT_LOGIN_WITH_XBOX)
            .json(&json!({
                "identityToken":
                    format!(
                        "XBL3.0 x={user_hash};{xsts_token}",
                        user_hash = user_hash,
                        xsts_token = xbox_security_token.token
                    )
            }))
            .send()
            .await?;
        response.error_for_status_ref()?;

        let response = response.json().await?;
        Ok(response)
    }

    #[tokio::main]
    pub async fn xbox_security_token(
        &self, xbox_token: String,
    ) -> Result<XboxLiveAuthenticationResponse, MinecraftAuthorizationError> {
        let response = self
            .http_client
            .post(XBOX_XSTS_AUTHORIZE)
            .json(&json!({
                "Properties": {
                    "SandboxId": "RETAIL",
                    "UserTokens": [xbox_token]
                },
                "RelyingParty": "rp://api.minecraftservices.com/",
                "TokenType": "JWT"
            }))
            .send()
            .await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            let xbox_security_token_err_resp_res = response.json().await;
            if xbox_security_token_err_resp_res.is_err() {
                return Err(MinecraftAuthorizationError::MissingClaims);
            }
            let xbox_security_token_err_resp: XboxLiveAuthenticationResponseError =
                xbox_security_token_err_resp_res.expect("This should succeed always");
            match xbox_security_token_err_resp.x_err {
                2148916238 => Err(MinecraftAuthorizationError::AddToFamily),
                2148916233 => Err(MinecraftAuthorizationError::NoXbox),
                _ => Err(MinecraftAuthorizationError::MissingClaims),
            }
        } else {
            response.error_for_status_ref()?;
            let xbox_security_token_resp: XboxLiveAuthenticationResponse = response.json().await?;
            Ok(xbox_security_token_resp)
        }
    }

    #[tokio::main]
    pub async fn xbox_token(
        &self, microsoft_access_token: impl AsRef<str>,
    ) -> Result<(String, String), MinecraftAuthorizationError> {
        let xbox_authenticate_json = json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": &format!("d={}", microsoft_access_token.as_ref())
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        });
        let response = self
            .http_client
            .post(XBOX_USER_AUTHENTICATE)
            .json(&xbox_authenticate_json)
            .send()
            .await?;
        response.error_for_status_ref()?;
        let xbox_resp: XboxLiveAuthenticationResponse = response.json().await?;
        let xbox_token = xbox_resp.token;
        let user_hash = xbox_resp
            .display_claims
            .get("xui")
            .ok_or(MinecraftAuthorizationError::MissingClaims)?
            .first()
            .ok_or(MinecraftAuthorizationError::MissingClaims)?
            .get("uhs")
            .ok_or(MinecraftAuthorizationError::MissingClaims)?
            .to_owned();
        Ok((xbox_token, user_hash))
    }
}