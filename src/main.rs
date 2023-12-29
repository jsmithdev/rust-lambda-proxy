use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt, Response };
use minreq;

async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {

    // Extract url from the request query string parameters
    let url: &str = match event
        .query_string_parameters_ref()
        .and_then(|params| params.first("url"))
    {
        Some(url) => url,
        None => {
            return Ok(
                Response::builder()
                    .status(400)
                    .body("Bad Request".into())
                    .expect("failed to render response"),
            )
        }
    };

    let body = get_body_as_string(url).await;

    Ok(
        match body {
            Ok(body) => format!("{}", body).into_response().await,
            Err(e) => format!("An error occurred: {}", e).into_response().await,
        }
    )
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}


async fn get_body_as_string(url: &str) -> Result<String, Error> {

    let response = minreq::get(url).send()?;

    let body = response.as_str()?;

    Ok(body.to_string())
}