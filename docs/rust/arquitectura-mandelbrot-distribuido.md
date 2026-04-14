# Arquitectura de Mandelbrot distribuido

## Objetivo

Definir la arquitectura funcional del sistema distribuido del proyecto utilizando un modelo coordinador-workers en Rust, ejecutado dentro de contenedores Docker sobre una red ZeroTier.

## Nodos del sistema

- `coord-01`: host del coordinador y de los workers `mandel-worker-01` y `mandel-worker-02`
- `worker-02`: host de `mandel-worker-03` y `mandel-worker-04`
- `omadebian`: host de `mandel-worker-05` y `mandel-worker-06`
- `wsl`: host de `mandel-worker-07` y `mandel-worker-08`

## Modelo de trabajo

El nodo `coord-01` actuará como coordinador del sistema. Su responsabilidad será dividir el problema, asignar tareas a los workers, recibir los resultados parciales y reconstruir el resultado final.

Cada una de las VMs worker alojará dos contenedores con rol de worker, de manera que el sistema final contará con ocho workers distribuidos.

## Estrategia de partición

La imagen de Mandelbrot se dividirá por rangos de filas. Cada worker recibirá un intervalo de filas definido por `start_row` y `end_row`, junto con los parámetros globales necesarios para calcular su fragmento de la imagen.

## Ejecución en Docker

El coordinador se ejecutará dentro de un contenedor Docker en `coord-01`. En cada VM worker se desplegarán dos contenedores Docker con la implementación worker del sistema.

## Comunicación

La comunicación entre coordinador y workers se realizará mediante sockets TCP. El contenedor del coordinador expondrá un puerto al host para aceptar conexiones desde los contenedores workers ubicados en los demás nodos conectados por ZeroTier.

## Flujo general

1. El contenedor del coordinador inicia y queda a la espera de conexiones.
2. Los contenedores workers se conectan al coordinador usando la dirección ZeroTier del host coordinador y el puerto publicado.
3. El coordinador divide la imagen en bloques.
4. El coordinador asigna un bloque a cada worker.
5. Cada worker calcula su bloque de Mandelbrot.
6. Cada worker devuelve resultados parciales.
7. El coordinador integra los resultados y genera la salida final.
