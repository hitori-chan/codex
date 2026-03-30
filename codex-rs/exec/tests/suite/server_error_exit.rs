#![cfg(not(target_os = "windows"))]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use codex_core::auth::CODEX_API_KEY_ENV_VAR;
use core_test_support::responses;
use core_test_support::test_codex_exec::test_codex_exec;
use std::process::Command;
use std::time::Duration;

/// Verify that when the server reports an error, `codex-exec` exits with a
/// non-zero status code so automation can detect failures.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exits_non_zero_when_server_reports_error() -> anyhow::Result<()> {
    let test = test_codex_exec();

    // Mock a simple Responses API SSE stream that immediately reports a
    // `response.failed` event with an error message.
    let server = responses::start_mock_server().await;
    let body = responses::sse(vec![serde_json::json!({
        "type": "response.failed",
        "response": {
            "id": "resp_err_1",
            "error": {"code": "rate_limit_exceeded", "message": "synthetic server error"}
        }
    })]);
    responses::mount_sse_once(&server, body).await;

    test.cmd_with_server(&server)
        .arg("--skip-git-repo-check")
        .arg("tell me something")
        .arg("--experimental-json")
        .assert()
        .code(1);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn autonomous_exec_submits_follow_up_prompt_before_exiting_on_error() -> anyhow::Result<()> {
    let test = test_codex_exec();

    let server = responses::start_mock_server().await;
    let request_log = responses::mount_sse_sequence(
        &server,
        vec![
            responses::sse(vec![
                responses::ev_response_created("resp_auto_1"),
                responses::ev_assistant_message("msg_auto_1", "first"),
                responses::ev_completed("resp_auto_1"),
            ]),
            responses::sse(vec![serde_json::json!({
                "type": "response.failed",
                "response": {
                    "id": "resp_auto_2",
                    "error": {"code": "rate_limit_exceeded", "message": "stop after follow-up"}
                }
            })]),
        ],
    )
    .await;

    let mut child = Command::new(codex_utils_cargo_bin::cargo_bin("codex-exec")?)
        .current_dir(test.cwd_path())
        .env("CODEX_HOME", test.home_path())
        .env(CODEX_API_KEY_ENV_VAR, "dummy")
        .arg("-c")
        .arg(format!(
            "openai_base_url={}",
            serde_json::to_string(&format!("{}/v1", server.uri()))?
        ))
        .arg("--skip-git-repo-check")
        .arg("--autonomous")
        .arg("keep going")
        .arg("hello")
        .spawn()?;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    loop {
        if request_log.requests().len() >= 2 {
            break;
        }
        if let Some(status) = child.try_wait()? {
            anyhow::bail!("codex-exec exited before autonomous follow-up request: {status}");
        }
        if tokio::time::Instant::now() >= deadline {
            anyhow::bail!(
                "timed out waiting for autonomous follow-up request; saw {} request(s)",
                request_log.requests().len()
            );
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    let _ = child.kill();
    let _ = child.wait();

    let requests = request_log.requests();
    assert!(
        requests.len() >= 2,
        "expected an autonomous follow-up request"
    );
    assert!(
        requests[0].has_message_with_input_texts("user", |texts| {
            texts.last().map(String::as_str) == Some("hello")
        }),
        "expected initial request to contain the original prompt as the final user text span"
    );
    assert!(
        requests[1].has_message_with_input_texts("user", |texts| {
            texts.last().map(String::as_str) == Some("keep going")
        }),
        "expected follow-up request to contain the autonomous prompt as the final user text span"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn autonomous_exec_does_not_submit_follow_up_after_fatal_rate_limit_error()
-> anyhow::Result<()> {
    let test = test_codex_exec();

    let server = responses::start_mock_server().await;
    let request_log = responses::mount_sse_once(
        &server,
        responses::sse(vec![serde_json::json!({
            "type": "response.failed",
            "response": {
                "id": "resp_auto_quota",
                "error": {"code": "insufficient_quota", "message": "quota exhausted"}
            }
        })]),
    )
    .await;

    test.cmd_with_server(&server)
        .arg("--skip-git-repo-check")
        .arg("--autonomous")
        .arg("keep going")
        .arg("hello")
        .assert()
        .code(1);

    let requests = request_log.requests();
    assert_eq!(
        requests.len(),
        1,
        "expected no autonomous follow-up request after a fatal rate limit error"
    );
    assert!(
        requests[0].has_message_with_input_texts("user", |texts| {
            texts.last().map(String::as_str) == Some("hello")
        }),
        "expected the initial request to contain the original prompt as the final user text span"
    );

    Ok(())
}
