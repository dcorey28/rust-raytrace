# Raytrace
A Rust implementation of a simple ray tracer based on the book
[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

### Purpose
This ray tracer is simply a fun project I am making to practice Rust and to brush up on ray tracing
fundamentals.

## Learnings
- Generic numbers are surprisingly difficult
- Deriving the Copy trait makes things easier because you don't have to have separate implementations for borrow vs move and you can use a value after move. (Let's your type act more like a primitive number type)
