# Dockerfile de la aplicación Rust distribuida

## Objetivo

Empaquetar la aplicación distribuida de Mandelbrot en Rust dentro de una imagen Docker reutilizable para el coordinador y los workers.

## Estrategia

Se utilizó un `Dockerfile` multi-stage:

- una fase de compilación con la imagen oficial de Rust
- una fase final ligera basada en Debian slim

## Resultado

Se construyó la imagen `mandelbrot-rust`, capaz de ejecutar el modo `coordinator` y el modo `worker`.
