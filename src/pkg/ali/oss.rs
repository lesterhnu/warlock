use hmac::{Hmac, Mac};
use serde::Serialize;
use sha1::Sha1;
use base64::Engine;
use time::format_description;
use time::Duration;
use time::OffsetDateTime;
use serde_json::json;
use crate::Result;
use crate::CONFIG;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug, Serialize)]
pub struct UploadSignInfo {
    pub access_key_id: String,
    pub policy: String,
    pub signature: String,
    pub host: String,
}

fn generate_policy_and_signature(
    access_key_secret: &str,
    bucket: &str
) -> (String, String) {
    // 1. 创建策略(Policy)
    let expiration = OffsetDateTime::now_utc() + Duration::minutes(30);
    let format_desc = format_description::parse(
        "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
    ).unwrap();
    let max_size = CONFIG.get_int("aliyun.oss_max_size").unwrap_or(100) * 1024 * 1024;
    let policy = json!({
        "expiration": expiration.format(&format_desc).unwrap(),
        "conditions": [
            {"bucket": bucket},
            ["starts-with", "$key", CONFIG.get_string("aliyun.oss_dir").unwrap_or("test".to_string())],
            {"success_action_status": "200"},
            ["content-length-range", 100, max_size] // 100MB
        ]
    });

    // 2. 将Policy转换为Base64字符串
    let policy_str = policy.to_string();
    let policy_b64 = base64::engine::general_purpose::STANDARD.encode(policy_str.as_bytes());

    // 3. 生成签名
    let mut mac = HmacSha1::new_from_slice(access_key_secret.as_bytes())
        .expect("HMAC initialization failed");
    mac.update(policy_b64.as_bytes());
    let signature = mac.finalize().into_bytes();
    let signature_b64 = base64::engine::general_purpose::STANDARD.encode(signature);

    (policy_b64, signature_b64)
}

pub fn get_sign() ->Result<UploadSignInfo>{
    let access_key_id = CONFIG.get_string("aliyun.oss_access_key_id")?;
    let access_key_secret = CONFIG.get_string("aliyun.oss_access_key_secret")?;
    let bucket = "zergoss";
    let endpoint = "oss-cn-beijing.aliyuncs.com";

    let (policy, signature) = generate_policy_and_signature(
        access_key_secret.as_str(),
        bucket
    );
    let info = UploadSignInfo {
        access_key_id: access_key_id,
        policy: policy,
        signature: signature,
        host: format!("https://{}.{}", bucket,endpoint),
    };
    Ok(info)
}
#[test]
fn test_get_sign(){
    let result = get_sign();
    assert!(result.is_ok());
    println!("Signature: {:?}", result.unwrap());
}