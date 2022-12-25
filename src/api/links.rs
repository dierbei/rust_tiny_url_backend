use nanoid::nanoid;
use actix_web::{ get, post, http::header, web::{self, Json, Path }, HttpResponse, Responder };
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::api::ApiResult;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
struct Link {
    tiny_code: String,
    origin_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiAddLink {
    origin_url: String,
}

impl ApiAddLink {
    fn to_new_link(self) -> Link {
        Link {
            tiny_code: nanoid!(5),
            origin_url: self.origin_url,
        }
    }
}

// create_link
// 创建一个短连接
#[post("/")]
pub async fn create_link(link: Json<ApiAddLink>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let new_link = link.0.to_new_link();
    let new_code = new_link.tiny_code.clone();

    if let Err(e) = db_insert_tiny_link(data.as_ref().clone(), new_link).await {
        return Json(ApiResult::error(e));
    }

    Json(ApiResult::success(Some(new_code)))
}


// get_from_link
// 根据 code 查询对应的源 url
#[get("/{code}")]
pub async fn get_from_link(path: Path<String>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let code: String = path.into_inner();
    let origin_url = db_get_origin_url(data.as_ref().clone(), code).await;
    let origin_url = match origin_url {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e.to_string());
            return HttpResponse::NotFound().finish();
        }
    };

    HttpResponse::Found().append_header((header::LOCATION, origin_url)).finish()
}

// list_link
// 查询所有的 url 列表
#[get("/")]
pub async fn list_link(data: web::Data<Pool<MySql>>) -> impl Responder {
    let links = db_list_tiny_url(data.as_ref().clone()).await;
    let links = match links {
        Ok(v) => v,
        Err(e) => {
            println!("err: {}", e.to_string());
            return Json(ApiResult::error(e));
        }
    };

    Json(ApiResult::success(Some(links)))
}

async fn db_get_origin_url(pool: Pool<MySql>, code: String) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("select origin_url from tiny_link where tiny_code = ?")
        .bind(code)
        .fetch_one(&pool)
        .await?;

    Ok(row.0)
}

async fn db_list_tiny_url(pool: Pool<MySql>) -> Result<Vec<Link>, sqlx::Error> {
    let links = sqlx::query_as::<_, Link>("SELECT tiny_code, origin_url from tiny_link")
        .fetch_all(&pool)
        .await?;
    Ok(links)
}

async fn db_insert_tiny_link(pool: Pool<MySql>, new_link: Link) -> Result<u64, sqlx::Error> {
    let id = sqlx::query(r#"insert into tiny_link(tiny_code, origin_url) values(?, ?)"#)
        .bind(new_link.tiny_code)
        .bind(new_link.origin_url)
        .execute(&pool)
        .await?
        .last_insert_id();

    Ok(id)
}