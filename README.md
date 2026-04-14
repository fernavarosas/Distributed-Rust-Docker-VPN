# Proyecto 1: VPN + Docker + Rust Distribuido

## Materia

**Uso, Adaptación y Explotación de Sistemas Operativos**

## Descripción general

Este proyecto implementa un entorno distribuido utilizando máquinas virtuales Ubuntu Server conectadas mediante **ZeroTier**, contenedores **Docker** y una aplicación en **Rust** bajo un modelo **coordinador-workers** para ejecutar el algoritmo de **Mandelbrot**.

El sistema final distribuye el cálculo del problema entre múltiples contenedores worker, recibe los resultados en un contenedor coordinador y genera un archivo final consolidado con los valores calculados.

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

## Tecnologías utilizadas

- **Ubuntu Server 24.04**
- **VirtualBox**
- **ZeroTier CLI**
- **Docker**
- **Rust**
- **GitHub**
- **GitHub Pages**

---

## Arquitectura final

### Hosts y nodos utilizados

- `coord-01`
- `worker-02`
- `omadebian`
- `wsl`

### Distribución final de contenedores

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

## Modelo de ejecución

El sistema usa un modelo **coordinador-workers**:

- el **coordinador** escucha conexiones de los workers
- cada worker se registra
- el coordinador asigna un bloque de filas del algoritmo Mandelbrot a cada worker
- cada worker calcula su bloque
- cada worker devuelve sus resultados
- el coordinador integra todos los bloques y genera el archivo final

---

## Configuración final del cálculo

- **Resolución:** `800x800`
- **Iteraciones máximas:** `1000`
- **Total de workers:** `8`
- **Asignación:** `100 filas por worker`

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

### Metadatos verificados

- width=800
- height=800
- max_iter=1000
- total_pixels=640000

Esto confirma que el sistema distribuido completó el cálculo total de Mandelbrot y consolidó los resultados en un único archivo final.

## Estructura del repositorio

Distributed-Rust-Docker-VPN/
├─ README.md
├─ docs/
│ ├─ index.md
│ ├─ ubuntu-server/
│ ├─ zerotier/
│ ├─ docker/
│ └─ rust/
├─ project-management/
│ ├─ sprint-1/
│ ├─ sprint-2/
│ ├─ sprint-3/
│ └─ evidencias/
├─ infra/
│ └─ vm-inventory.md
├─ reports/
└─ mandelbrot-distributed/
├─ Cargo.toml
├─ Dockerfile
└─ src/
├─ main.rs
├─ coordinator.rs
├─ worker.rs
├─ protocol.rs
└─ mandelbrot.rs

## Aplicación Rust

La aplicación distribuida fue desarrollada en Rust y organizada en los siguientes módulos:

- main.rs → selección de modo coordinator o worker
- coordinator.rs → lógica del coordinador
- worker.rs → lógica de los workers
- protocol.rs → estructuras del protocolo JSON
- mandelbrot.rs → lógica del cálculo del algoritmo

## Imagen Docker

La aplicación se empaquetó en una imagen Docker llamada:

mandelbrot-rust

## Construcción de la imagen

```bash
docker build -t mandelbrot-rust .
```

## Ejecución del coordinador

En coord-01:

```bash
sudo docker run -d -p 9000:9000 --name mandel-coordinator mandelbrot-rust coordinator
```

## Ejecución de un worker

Ejemplo:

```bash
sudo docker run -d --name mandel-worker-01 mandelbrot-rust worker mandel-worker-01 10.236.62.155:9000
```

## Comunicación de red

La comunicación entre nodos se realizó sobre ZeroTier.

En algunos casos, como omadebian, fue necesario ejecutar ciertos contenedores con:

```bash
--network host
```

para que los workers pudieran utilizar directamente la pila de red del host Linux, incluyendo su interfaz ZeroTier, y así alcanzar correctamente la IP del coordinador.

## Documentación técnica

La documentación detallada del proyecto se encuentra en la carpeta /docs e incluye:

- instalación de Ubuntu Server
- clonación de VMs
- y validación de ZeroTier
- instalación y validación de Docker
- conectividad intra-VM e inter-VM
- arquitectura de Mandelbrot distribuido
- protocolo JSON
- estructura del proyecto Rust
- despliegue final en VMs reales

## Gestión del proyecto

La planeación y seguimiento bajo enfoque Scrum se encuentra en:

- project-management/sprint-1/
- project-management/sprint-2/
- project-management/sprint-3/

Incluye:

- goals por sprint
- backlog
- daily log
- review
- evidencias

## Evidencias del proyecto

Completado a nivel funcional y documental principal.

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

## Enlace de documentación

Pendiente colocar URL de Github Pages.
