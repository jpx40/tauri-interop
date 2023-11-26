use api::model::TestState;
use gloo_timers::callback::Timeout;

fn main() {
    console_log::init_with_level(log::Level::Trace).expect("no errors during logger init");
    console_error_panic_hook::set_once();

    api::cmd::empty_invoke();

    wasm_bindgen_futures::spawn_local(async {
        log::info!("{}", api::cmd::greet("frontend").await);
    });

    wasm_bindgen_futures::spawn_local(async move {
        let handle_foo = TestState::listen_to_foo(|echo| log::info!("foo: {echo}"))
            .await
            .unwrap();

        let handle_bar = TestState::listen_to_bar(|echo| log::info!("bar: {echo}"))
            .await
            .unwrap();

        Timeout::new(1000, api::cmd::emit).forget();
        // with the move here, we hold "handle" in scope... if we wouldn't do that
        // handle would be dropped already and we get errors in the ui
        //
        // it can be fixed with `handle.closure.take().unwrap().forget()`
        // see the `Closure::forget` docs, why this isn't the recommended way
        Timeout::new(2000, move || {
            handle_foo.detach_listen();
            handle_bar.detach_listen();
        })
        .forget();
        Timeout::new(3000, api::cmd::emit).forget();
    });
}
