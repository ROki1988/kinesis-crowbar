#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

lambda!(|event, context| {
    println!("hi cloudwatch logs, this is {}", context.function_name());
    // return the event without doing anything with it
    Ok(event)
});