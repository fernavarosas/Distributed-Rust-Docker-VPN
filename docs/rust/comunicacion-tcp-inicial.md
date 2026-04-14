# Comunicación TCP inicial entre coordinator y worker

## Objetivo

Validar la comunicación básica entre el coordinador y un worker usando sockets TCP y mensajes JSON en Rust.

## Implementación inicial

- El coordinador escucha en `0.0.0.0:9000`
- El worker se conecta al coordinador
- El worker envía un mensaje `register`
- El coordinador responde con un mensaje `ack`

## Resultado esperado

- El worker confirma que recibió la respuesta del coordinador
- El coordinador registra la conexión y el identificador del worker

## Resultado

Se validó correctamente el primer intercambio coordinador-worker usando TCP y JSON.
