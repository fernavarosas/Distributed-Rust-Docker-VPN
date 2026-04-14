# Proyecto 1: VPN + Docker + Rust Distribuido

## Materia

Uso, Adaptación y Explotación de Sistemas Operativos

## Descripción general

Este proyecto implementa un entorno distribuido utilizando máquinas virtuales Ubuntu Server conectadas mediante **ZeroTier**, contenedores **Docker** y una aplicación en **Rust** bajo un modelo **coordinador-workers** para ejecutar el algoritmo de **Mandelbrot**.

El sistema final distribuye el cálculo entre múltiples contenedores worker, recibe los resultados en un contenedor coordinador y genera un archivo final consolidado con los valores calculados.

---

## Objetivo del proyecto

Construir y documentar una infraestructura distribuida real que permita:

- conectar nodos remotos mediante una VPN
- ejecutar contenedores Docker en múltiples máquinas virtuales
- desplegar una aplicación distribuida en Rust
- aplicar un modelo coordinador-workers
- distribuir el cálculo del algoritmo Mandelbrot
- integrar los resultados finales en el nodo coordinador

---

## Arquitectura final

### Hosts utilizados

- `coord-01`
- `worker-02`
- `omadebian`
- `wsl`

### Contenedores desplegados

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

### Resumen

- **1 coordinador**
- **8 workers**

---

## Configuración final del cálculo

- Resolución: `800x800`
- Iteraciones máximas: `1000`
- Total de workers: `8`
- Asignación: `100 filas por worker`

### Distribución de bloques

- `mandel-worker-01` → filas `0-99`
- `mandel-worker-02` → filas `100-199`
- `mandel-worker-03` → filas `200-299`
- `mandel-worker-04` → filas `300-399`
- `mandel-worker-05` → filas `400-499`
- `mandel-worker-06` → filas `500-599`
- `mandel-worker-07` → filas `600-699`
- `mandel-worker-08` → filas `700-799`

---

## Resultado final

El coordinador recibió correctamente los 8 bloques esperados y generó el archivo:

```text
mandelbrot_result.txt
```

---

## Metadatos verificados

- width=800
- height=800
- max_iter=1000
- total_pixels=640000

Esto confirmó que el sistema distribuido completó el cálculo total de Mandelbrot y consolidó los resultados en un único archivo final.

## Tecnologías utilizadas

- Ubuntu Server 24.04
- VirtualBox
- ZeroTier CLI
- Docker
- Rust
- GitHub
- GitHub Pages

## Contenido de la documentación

- [Instalación de Ubuntu Server](./ubuntu-server/instalacion-vm-01.md)
- [Clonación de workers](./ubuntu-server/clonacion-workers.md)
- [Instalación de ZeroTier](./zerotier/instalacion-zerotier.md)
- [Unión de nodos a la red](./zerotier/union-red.md)
- [Validación de conectividad ZeroTier](./zerotier/validacion-conectividad.md)
- [Instalación de Docker](./docker/instalacion-docker.md)
- [Despliegue de contenedores](./docker/despliegue-contenedores.md)
- [Conectividad intra-VM](./docker/conectividad-intra-vm.md)
- [Conectividad inter-VM](./docker/conectividad-inter-vm.md)
- [Dockerfile de Rust](./docker/dockerfile-rust.md)
- [Arquitectura de Mandelbrot distribuido](./rust/arquitectura-mandelbrot-distribuido.md)
- [Diseño Docker + Rust](./rust/diseno-docker-rust.md)
- [Protocolo JSON](./rust/protocolo-json.md)
- [Estructura base del proyecto Rust](./rust/estructura-proyecto-rust.md)
- [Comunicación TCP inicial](./rust/comunicacion-tcp-inicial.md)
- [Intercambio task-result](./rust/intercambio-task-result.md)
- [Asignación de bloques](./rust/asignacion-de-bloques.md)
- [Versión final funcional de Mandelbrot](./rust/version-final-mandelbrot.md)
- [Despliegue inicial en VMs reales](./rust/despliegue-inicial-vms.md)

## Estado del proyecto

Proyecto completado a nivel funcional y documental principal.

Se logró:

- infraestructura distribuida funcional
- VPN operativa con ZeroTier
- despliegue de contenedores Docker
- implementación distribuida en Rust
- ejecución coordinador-workers
- integración del resultado final de Mandelbrot

## Integrantes

- Fernando Navarro — Product Owner / Development Team
- Bryan Omar Gomez Romero — Scrum Master / Development Team
