# Diseño inicial de Docker + Rust distribuido

## Objetivo

Definir la forma en que se desplegará la implementación distribuida de Mandelbrot en Rust utilizando contenedores Docker sobre la infraestructura conectada por ZeroTier.

## Decisiones de diseño

- Se utilizará un solo proyecto Rust.
- El programa tendrá dos modos de ejecución: `coordinator` y `worker`.
- El nodo `coord-01` alojará un único contenedor con rol de coordinador.
- Cada una de las VMs worker alojará dos contenedores con rol de worker.
- La comunicación se realizará mediante TCP.
- El coordinador expondrá un puerto al host para aceptar conexiones remotas.
- El intercambio de mensajes se realizará en formato JSON.

## Distribución de contenedores

- `coord-01` → `mandel-coordinator`, `mandel-worker-01`, `mandel-worker-02`
- `worker-02` → `mandel-worker-03`, `mandel-worker-04`
- `omadebian` → `mandel-worker-05`, `mandel-worker-06`
- `wsl` → `mandel-worker-07`, `mandel-worker-08`

Con esta distribución, el sistema final cuenta con 1 contenedor coordinador y 8 contenedores worker ejecutando Mandelbrot distribuido en Rust.

## Ejecución esperada

Ejemplo conceptual del coordinador:

```bash
docker run -d --name mandel-coordinator -p 9000:9000 mandelbrot-rust coordinator
```

Ejemplo conceptual de un worker:

```bash
docker run -d --name mandel-worker-01 mandelbrot-rust worker worker-01 10.236.62.155:9000
```

## Observación

Los contenedores utilizados en Sprint 2 correspondieron a pruebas de infraestructura y conectividad. En Sprint 3 se reemplazarán por contenedores con la implementación real del sistema distribuido en Rust.
