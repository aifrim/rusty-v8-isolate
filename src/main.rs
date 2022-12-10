use v8;
use std::env;
use std::fs;
use std::thread;

mod register;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./rusty-v8-isolate /path/to/file");

        return;
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let handle = thread::spawn(|| {
        isolate_thread(contents)
    });

    handle.join().unwrap();
}

fn isolate_thread(contents: String) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let params = v8::Isolate::create_params().allow_atomics_wait(true);

        let isolate = &mut v8::Isolate::new(params);
        isolate.set_microtasks_policy(v8::MicrotasksPolicy::Explicit);

        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        let object_template = v8::ObjectTemplate::new(scope);

        register::register_js_fns(scope, object_template);

        let context = v8::Context::new_from_template(scope, object_template);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, &contents).unwrap();
        let script = v8::Script::compile(scope, code, None).unwrap();

        script.run(scope);
    }
}
