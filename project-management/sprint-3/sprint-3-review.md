# Sprint 3 Review

## Sprint

**Sprint 3: Rust distribuido + cierre del proyecto**

## Objetivo del sprint

Implementar la arquitectura distribuida final del proyecto utilizando contenedores Docker y una aplicación en Rust bajo un modelo coordinador-workers, ejecutando el algoritmo Mandelbrot sobre los nodos conectados mediante ZeroTier.

## Trabajo realizado

Durante este sprint se completaron las siguientes actividades:

- Se integraron los nodos funcionales del proyecto dentro de una misma red ZeroTier.
- Se definió la arquitectura final del sistema con:
  - un contenedor `mandel-coordinator`
  - ocho contenedores `mandel-worker`
- Se diseñó el protocolo JSON entre coordinador y workers para:
  - registro
  - confirmación
  - asignación de tareas
  - devolución de resultados
- Se creó la estructura base del proyecto Rust:
  - `main.rs`
  - `coordinator.rs`
  - `worker.rs`
  - `protocol.rs`
  - `mandelbrot.rs`
- Se validó la comunicación TCP inicial entre coordinador y worker.
- Se validó el intercambio `register -> ack -> task -> result`.
- Se creó el `Dockerfile` de la aplicación distribuida en Rust.
- Se construyó la imagen Docker y se validó la ejecución contenerizada del coordinador y los workers.
- Se modificó la lógica del coordinador para asignar bloques distintos de filas a cada worker.
- Se implementó una versión final funcional del cálculo distribuido de Mandelbrot.
- Se desplegó el sistema sobre los nodos disponibles del proyecto.
- Se ejecutó el cálculo distribuido final y se integró el resultado en el archivo `mandelbrot_result.txt`.

## Arquitectura final utilizada

La arquitectura final del sistema quedó definida de la siguiente manera:

- `coord-01`
  - `mandel-coordinator`
  - `mandel-worker-01`
  - `mandel-worker-02`
- `worker-02`
  - `mandel-worker-03`
  - `mandel-worker-04`
- `omadebian`
  - `mandel-worker-05`
  - `mandel-worker-06`
- `wsl`
  - `mandel-worker-07`
  - `mandel-worker-08`

Con esta distribución se ejecutó un total de:

- **1 coordinador**
- **8 workers**

## Ejecución final del algoritmo

Para la versión final funcional del proyecto se utilizó la siguiente configuración:

- Resolución: `800x800`
- Iteraciones máximas: `1000`
- Total de workers: `8`
- Asignación: `100 filas por worker`

Distribución de bloques:

- `mandel-worker-01` → filas `0-99`
- `mandel-worker-02` → filas `100-199`
- `mandel-worker-03` → filas `200-299`
- `mandel-worker-04` → filas `300-399`
- `mandel-worker-05` → filas `400-499`
- `mandel-worker-06` → filas `500-599`
- `mandel-worker-07` → filas `600-699`
- `mandel-worker-08` → filas `700-799`

Cada worker devolvió un bloque de `80000` píxeles y el coordinador integró los ocho bloques en el archivo final `mandelbrot_result.txt`.

## Resultados obtenidos

El sistema distribuido logró completar la ejecución del algoritmo Mandelbrot sobre la infraestructura final del proyecto.

Los resultados principales fueron:

- El coordinador recibió correctamente los ocho bloques esperados.
- Se completó la integración total de resultados con un avance final de `8/8 bloques`.
- El archivo `mandelbrot_result.txt` fue generado correctamente por el coordinador.
- Se verificaron los metadatos del resultado final:
  - `width=800`
  - `height=800`
  - `max_iter=1000`
  - `total_pixels=640000`

Esto confirmó que el sistema distribuido sí logró procesar el problema completo y consolidar el resultado final.

## Incidencias y decisiones técnicas

Durante el sprint se presentaron algunas incidencias relevantes:

### 1. Incidencia en `worker-01`

El nodo `worker-01` presentó errores persistentes al ejecutar la imagen Docker del proyecto, incluyendo fallos de entrada/salida dentro del contenedor.  
Se descartaron problemas de arquitectura (`amd64/linux`) y se concluyó que el nodo presentaba una inconsistencia local derivada de un incidente previo de almacenamiento en el host Windows.

### 2. Ajuste de despliegue final

Para no bloquear el avance del proyecto, se definió como arquitectura activa final el uso de:

- `coord-01`
- `worker-02`
- `omadebian`
- `wsl`

Además, se decidió que `coord-01` alojara no solo el coordinador, sino también dos contenedores worker, documentado como parte de la arquitectura final.

### 3. Ajuste de red en `omadebian`

Los workers desplegados en `omadebian` presentaron inicialmente errores de conectividad tipo `No route to host` al intentar alcanzar la IP ZeroTier del coordinador desde la red interna por defecto de Docker.

La situación se resolvió ejecutando dichos contenedores con `--network host`, lo que permitió que utilizaran directamente la pila de red del host Linux, incluyendo su interfaz de ZeroTier. De esta forma, la comunicación siguió realizándose sobre la VPN del proyecto.

## Estado del sprint

**Sprint 3 completado.**

## Entregables logrados

- Arquitectura final coordinador-workers definida y documentada.
- Implementación distribuida de Mandelbrot en Rust.
- Protocolo JSON implementado.
- Contenerización de coordinador y workers con Docker.
- Ejecución distribuida real sobre los nodos conectados por ZeroTier.
- Integración del resultado final en `mandelbrot_result.txt`.
- Documentación técnica del despliegue y pruebas actualizada.
- Bitácora y evidencias del sprint actualizadas.

## Conclusión

Sprint 3 permitió cerrar el objetivo principal del proyecto: ejecutar una aplicación distribuida real en Rust sobre una infraestructura conectada mediante ZeroTier y contenerizada con Docker.

El sistema final logró coordinar múltiples workers distribuidos en distintos nodos, asignarles bloques distintos del algoritmo Mandelbrot, recibir resultados parciales y consolidarlos en un resultado final verificable. Con ello, el proyecto quedó completado tanto en su parte técnica como en su documentación principal.
