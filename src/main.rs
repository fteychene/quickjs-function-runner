
use std::{time::Instant, sync::Arc};

use hirofa_utils::js_utils::{
    adapters::JsRealmAdapter,
    Script, facades::{JsRuntimeFacade, JsRuntimeBuilder},
};
use quickjs_runtime::{
    builder::QuickJsRuntimeBuilder,
    facades::QuickJsRuntimeFacade,
    quickjs_utils::primitives, quickjsrealmadapter::QuickJsRealmAdapter,
};

#[tokio::main]
async fn main() -> Result<(), String> {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let rt = QuickJsRuntimeBuilder::new().js_build();

    let start = Instant::now();
    let function_id = eval_function(include_str!("script.js").to_string(),&rt)?;
    log::info!("Function added in cache with id {} in {}μs", function_id, start.elapsed().as_micros());


    let rt_arc = Arc::new(rt);
    let start_filter = Instant::now();
    let payloads = 10000;
    for handle in (0..payloads)
        .map(|_| tokio::task::spawn(run_function(rt_arc.clone(), function_id, r#"{"test": 3}"#.to_string())))
    {
        handle.await.map_err(|_| "Error when joining tassks".to_string())??;
    }
    log::info!("Executed {} functions on payloads in {}μs", payloads, start_filter.elapsed().as_micros());
    Ok(())
}


fn eval_function(code: String, rt: &QuickJsRuntimeFacade) -> Result<i32, String> {
    rt.js_loop_realm_sync(None, move |_rt, realm_adapter| unsafe {
        QuickJsRealmAdapter::eval_ctx(
            realm_adapter.context,
            Script::new("my_tenant_function_name", &format!("({});", code)),
            None,
        )
        .map_err(|_| "Error while evaluating function".to_string())
        .map(|func| realm_adapter.js_cache_add(&func))
    })
}

async fn run_function(
    rt: Arc<QuickJsRuntimeFacade>,
    function_id: i32,
    payload: String,
) -> Result<bool, String> {
    rt.js_loop_realm_sync(None, move |_rt, realm_adapter| {
        let payload = realm_adapter.js_json_parse(payload.as_str()).unwrap();
        realm_adapter
            .js_cache_with(function_id, |func| {
                realm_adapter
                    .js_function_invoke(None, &func, &[&payload])
                    .and_then(|result| primitives::to_bool(&result))
            })
            .map_err(|_| "Error during execution of filter".to_string())
    })
}