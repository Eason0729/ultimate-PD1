#[allow(clippy::all, non_local_definitions)]
mod judger {
    tonic::include_proto!("oj.judger");
}

pub use judger::*;
use judger_client::JudgerClient;
use tonic::codegen::tokio_stream::StreamExt;

pub async fn run_code(code: Vec<u8>, io: (Vec<u8>, Vec<u8>)) -> JudgerCode {
    let mut judger = JudgerClient::connect(String::from("http://127.0.0.1:8082"))
        .await
        .unwrap();
    let res = judger
        .judge(tonic::Request::new(JudgeRequest {
            lang_uid: "7daff707-26b5-4153-90ae-9858b9fd9619".to_string(),
            code,
            memory: 64 * 1024 * 1024,
            time: 1000 * 1000 * 1000,
            rule: JudgeMatchRule::IgnoreSnl as i32,
            tests: vec![TestIo {
                input: io.0,
                output: io.1,
            }],
        }))
        .await
        .unwrap();

    let stream = res.into_inner();
    let results: Vec<_> = stream.collect().await;

    let code = results
        .into_iter()
        .filter_map(|x| x.ok().map(|x| JudgerCode::try_from(x.status)))
        .flatten()
        .last()
        .unwrap_or(JudgerCode::default());

    code
}
