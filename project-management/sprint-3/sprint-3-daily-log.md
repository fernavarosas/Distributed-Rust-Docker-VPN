## Avance: definición de arquitectura Docker + Rust

- Se acordó utilizar un solo proyecto Rust con dos modos de operación: `coordinator` y `worker`.
- Se definió que el coordinador se ejecutará en `coord-01` y escuchará en un puerto fijo para recibir conexiones de los workers.
- Se estableció que la comunicación entre nodos se realizará mediante TCP sobre la red ZeroTier.
- Se decidió dividir el cálculo de Mandelbrot por rangos de filas y utilizar JSON como formato de intercambio de mensajes.

## Ajuste de arquitectura del Sprint 3

- Se corrigió la distribución final de contenedores del proyecto.
- Se estableció que `coord-01` tendrá únicamente un contenedor con rol de coordinador.
- Se definió que solo las VMs worker alojarán dos contenedores Docker con rol de worker.
- Con esta decisión, la arquitectura final del sistema queda compuesta por 1 contenedor coordinador y 8 contenedores worker.

## Avance: diseño del protocolo JSON

- Se definió el protocolo JSON entre el contenedor coordinador y los contenedores worker.
- Se establecieron mensajes para registro, confirmación, asignación de tarea, devolución de resultados, finalización y error.
- Se acordó que el intercambio de mensajes será implementado en Rust usando serialización JSON.
- El diseño del protocolo quedó alineado con la arquitectura final del proyecto: 1 coordinador y 8 workers distribuidos en 4 VMs worker.

## Avance: creación de la estructura base del proyecto Rust

- Se creó el proyecto `mandelbrot-distributed`.
- Se definieron los módulos `main`, `coordinator`, `worker`, `protocol` y `mandelbrot`.
- Se agregaron las dependencias iniciales para serialización JSON con `serde` y `serde_json`.
- Se dejó funcionando la selección de modo de ejecución entre `coordinator` y `worker`.
- Se integró una primera versión de la función de cálculo por bloque para Mandelbrot.

## Avance: validación inicial de comunicación TCP

- Se implementó una primera comunicación TCP entre el coordinador y un worker en Rust.
- El worker logró conectarse al puerto del coordinador y enviar un mensaje de registro.
- El coordinador procesó el mensaje y respondió con una confirmación (`ack`).
- Con ello quedó validado el handshake básico sobre TCP usando mensajes JSON.

## Avance: validación de intercambio task-result

- Se amplió la comunicación TCP inicial entre el coordinador y el worker.
- El coordinador logró enviar una tarea de prueba en formato JSON.
- El worker procesó el bloque asignado usando la función de Mandelbrot y devolvió un resultado válido.
- Se confirmó el ciclo básico `register -> ack -> task -> result`.

## Avance: contenerización inicial de la aplicación Rust

- Se creó el Dockerfile de la aplicación distribuida en Rust.
- Se construyó la imagen `mandelbrot-rust`.
- Se validó la ejecución del coordinador y un worker dentro de contenedores Docker.
- Se confirmó que la lógica base del sistema distribuido funciona también en entorno contenerizado.

## Ajuste de despliegue por incidencia en worker-01

- Durante las pruebas distribuidas en VMs reales, `worker-01` presentó una incidencia persistente al ejecutar la imagen Docker de la aplicación Rust.
- Se descartaron problemas de arquitectura (`amd64/linux`) y se identificó que la falla correspondía al estado local del nodo.
- Para no bloquear el avance del Sprint 3, se habilitaron dos contenedores worker adicionales en `coord-01`.
- Con esta medida se continuó la validación del modelo coordinador-workers mientras se dejaba documentada la incidencia del nodo afectado.

## Avance: integración de la versión final funcional

- Se reemplazó la lógica de prueba por una versión final funcional del sistema distribuido.
- El coordinador ahora asigna bloques reales de 100 filas a cada worker.
- Cada worker calcula una porción real del conjunto de Mandelbrot con resolución 800x800.
- El coordinador recibe e integra los bloques devueltos por los workers.
- Se añadió generación de un archivo final consolidado `mandelbrot_result.txt`.

## Avance: integración completa del resultado final

- Se completó la ejecución distribuida final del algoritmo Mandelbrot.
- El coordinador recibió correctamente los 8 bloques esperados de los workers.
- Cada worker procesó un rango distinto de 100 filas sobre una resolución total de 800x800.
- Al alcanzar 8/8 bloques recibidos, el coordinador integró los resultados en memoria y generó el archivo final `mandelbrot_result.txt`.
- Con ello quedó validada la ejecución final del sistema distribuido sobre ZeroTier, Docker y Rust.
