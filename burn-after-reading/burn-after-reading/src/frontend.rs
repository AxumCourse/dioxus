use std::sync::Arc;

use axum::{extract::State, Json};
use sqlx::PgPool;
use validator::Validate;

use crate::{db, form, model, resp, util, ArcAppState, Result};

#[derive(serde::Serialize)]
pub struct CreateMessageResp {
    pub id: String,
    pub url: String,
}

pub async fn create_message(
    State(state): State<ArcAppState>,
    Json(message): Json<form::message::Create>,
) -> Result<resp::JsonResp<CreateMessageResp>> {
    message.validate()?;

    let message = model::Message::build(message.content, message.password)?;
    let id = db::message::create(&state.pool, &message).await?;
    let url = format!("{}/{}", &state.cfg.view_url_prefix, id);

    Ok(resp::ok(CreateMessageResp {
        id: id.to_string(),
        url,
    })
    .to_json())
}

#[derive(serde::Serialize)]
pub struct MessageResp {
    pub need_password: bool,
    #[serde(flatten)]
    pub message: Option<model::Message>,
}
pub async fn read_message(
    State(state): State<ArcAppState>,
    Json(frm): Json<form::message::Read>,
) -> Result<resp::JsonResp<MessageResp>> {
    frm.validate()?;

    let m = db::message::get(&state.pool, &frm.id).await?;
    let m = match m {
        Some(v) => v,
        None => return Err(anyhow::anyhow!("不存在的消息").into()),
    };
    let (need_password, password) = m.if_has_password();

    // 需要密码
    if need_password {
        // 未提供密码
        if frm.password.is_none() {
            return Ok(resp::ok(MessageResp {
                need_password: true,
                message: None,
            })
            .to_json());
        }

        let pwd = frm.password.unwrap();
        // 密码错误
        if !util::verify_pwd(&pwd, &password)? {
            return Err(anyhow::anyhow!("密码错误").into());
        }
        // 密码正确
        tokio::spawn(delete_viewed_msg(
            state.pool.clone(),
            Arc::new(frm.id),
            state.cfg.delete_interval,
        ));
        return Ok(resp::ok(MessageResp {
            need_password: false,
            message: Some(m),
        })
        .to_json());
    }

    // 不需要密码
    tokio::spawn(delete_viewed_msg(
        state.pool.clone(),
        Arc::new(frm.id),
        state.cfg.delete_interval,
    ));
    Ok(resp::ok(MessageResp {
        need_password: false,
        message: Some(m),
    })
    .to_json())
}

async fn delete_viewed_msg(p: PgPool, id: Arc<String>, delete_interval: u32) {
    tokio::time::sleep(std::time::Duration::from_secs(delete_interval as u64)).await;
    match db::message::del(&p, &id).await {
        Ok(aff) => tracing::debug!(
            "已删除消息：{}，间隔时间：{}，受影响的行数：{}",
            id,
            delete_interval,
            aff
        ),
        Err(e) => tracing::error!("删除消息 #{} 失败：{}", id, e),
    };
}
