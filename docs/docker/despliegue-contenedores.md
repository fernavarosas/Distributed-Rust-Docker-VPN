# Despliegue inicial de contenedores

## Objetivo

Crear dos contenedores Docker por cada VM para establecer la base del entorno distribuido del proyecto.

## Contenedores desplegados

- `coord-01`: `coord-c1`, `coord-c2`
- `worker-01`: `w1-c1`, `w1-c2`
- `worker-02`: `w2-c1`, `w2-c2`

## Observación de arquitectura

Los contenedores desplegados en esta fase corresponden a una validación inicial del entorno Docker y no representan todavía la implementación final del sistema distribuido en Rust.

La VM `coord-01` conservará el rol de coordinador del proyecto, pero la lógica real de coordinación se implementará en la siguiente fase mediante el programa distribuido en Rust, el cual delegará trabajo a los workers locales y remotos.

## Comando utilizado

Ejemplo:

```bash
sudo docker run -d --name coord-c1 nginx
```

## Verificación

Se utilizó:

```bash
sudo docker ps
```

## Resultado

Cada nodo quedó con dos contenedores en ejecución.
