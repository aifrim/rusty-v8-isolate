use v8;

pub fn register_js_fns(
    scope: &mut v8::HandleScope,
    object_template: v8::Local<v8::ObjectTemplate>,
) {
    register_js_fn(scope, object_template, "print", print);
    register_js_fn(scope, object_template, "sum", sum);
}

fn register_js_fn(
    scope: &mut v8::HandleScope,
    object_template: v8::Local<v8::ObjectTemplate>,
    name: &str,
    callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) {
    let function_template = v8::FunctionTemplate::new(scope, callback);
    let function_name = v8::String::new(scope, &name).unwrap();
    object_template.set(function_name.into(), function_template.into());
}

fn print(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    let mut str = "".to_string();

    for index in 0..args.length() {
        let value = args.get(index).to_rust_string_lossy(scope);

        str = str + " " +  &value;
    }

    println!("{}", str)
}

fn sum(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, mut rv: v8::ReturnValue) {
    let a = args.get(0).int32_value(scope).unwrap();
    let b = args.get(1).int32_value(scope).unwrap();

    let ab = a + b;

    rv.set(v8::Integer::new(scope, ab).into());
}
