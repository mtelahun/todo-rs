# ToDo-rs
![Rust](https://github.com/mtelahun/todo-rs/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/mtelahun/todo-rs/branch/main/graph/badge.svg?token=A1P9I5E2LU)](https://codecov.io/gh/mtelahun/todo-rs)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

A backend for a simple ToDo application written in Rust using a Clean Architecture, CQRS and Axum.

This project started as a tutorial project for using [SurrealDB](https:://github.com/surrealdb/surrealdb). I initially followed along this [two](https://blog.devgenius.io/creating-an-api-with-rust-clean-architecture-axum-and-surrealdb-2a95b1b72e0f) [part](https://blog.devgenius.io/creating-an-api-with-rust-clean-architecture-cqrs-axum-and-surrealdb-part-2-99a48b2d10bc) tutorial. However, after implementing the tutorial there were some things about it that I wasn't very happy with, like:
    * using literal strings as Ids
    * not being Idiomatic enough
    * using CQRS but not event-sourcing
    * no tests

These points aren't a reflection on the tutorial, which I found to be delightfully short and enlightening, but on my own need to see this silly Todo app to a satisfying conclusion. Also, it didn't help that I was in the middle of a DDD and event-sourcing kick at the time.