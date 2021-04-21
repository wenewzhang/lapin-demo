```
error[E0433]: failed to resolve: use of undeclared crate or module `tracing_subscriber`
  --> src/main.rs:16:5
   |
16 |     tracing_subscriber::fmt::init();
   |     ^^^^^^^^^^^^^^^^^^ use of undeclared crate or module `tracing_subscriber`

error[E0599]: no method named `ack` found for tuple `(lapin::Channel, Delivery)` in the current scope
  --> src/main.rs:49:18
   |
49 |                 .ack(BasicAckOptions::default())
   |                  ^^^ method not found in `(lapin::Channel, Delivery)`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
error: could not compile `lapin-demo`
```
