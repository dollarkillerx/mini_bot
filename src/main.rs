use std::process::Command;

use poem::{error::ParseJsonError, handler, Result, route, RouteMethod, Server, web::Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Shell {
    cmd: String,
}

#[handler]
fn cmd(res: Result<Json<Shell>, ParseJsonError>) -> Json<serde_json::Value> {
    let res = match res {
        Ok(Json(req)) => {
            let cmd = req.cmd.clone();
            let mut r =Command::new("sh")
                .arg("-c").arg(cmd);
            r.output().unwrap();

            serde_json::json!({
            "code": 0,
            "message": req.cmd,
        })
        }
        Err(err) => serde_json::json!({
            "code": 1,
            "message": err.to_string()
        }),
    };
    Json(res)
}

// right:
// curl -d '{"name": "Jack"}' http://127.0.0.1:3030/hello
// {"code": 0, "message": "hello: Jack"}

#[tokio::main]
async fn main() {
    let app = route().at("/cmd", RouteMethod::new().post(cmd));
    let addr = "0.0.0.0:3030";
    println!("mini bot run: {}", addr);
    let server = Server::bind(addr).await.unwrap();
    server.run(app).await.unwrap();
}
