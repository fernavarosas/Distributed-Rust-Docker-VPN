# Intercambio inicial de tarea y resultado

## Objetivo

Validar que el coordinador puede asignar una tarea de Mandelbrot a un worker y recibir el resultado calculado usando TCP y JSON.

## Flujo probado

1. El worker se conecta al coordinador.
2. El worker envía `register`.
3. El coordinador responde con `ack`.
4. El coordinador envía una `task`.
5. El worker ejecuta `compute_block`.
6. El worker devuelve `result`.

## Resultado

Se validó correctamente un ciclo básico de ejecución distribuida entre coordinador y worker.
