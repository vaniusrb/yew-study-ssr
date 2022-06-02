# Yew Server-side Rendering Study

This example demonstrates server-side rendering.

Based in yew simple_ssr and ssr_router examples.

# How to run this example

Run the script:

`./script/run.sh`

Open Browser:

Navigate to http://localhost:8080/ to view results.

# The problem

Occurs the error `function not implemented on non-wasm32 targets`
when I try to use `<Switch<AppRoute> render={switch} />`
