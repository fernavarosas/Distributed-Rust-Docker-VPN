# Validación de comunicación entre contenedores de la misma VM

## Objetivo

Comprobar que los contenedores desplegados dentro de una misma máquina virtual pueden comunicarse correctamente a través de la red interna de Docker.

## Procedimiento

Primero se consultó la IP interna de los contenedores:

```bash
sudo docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <nombre_contenedor>
```

Posteriormente se ejecutó un contenedor auxiliar temporal para realizar una petición HTTP al contenedor destino:

```bash
sudo docker run --rm alpine sh -c "wget -qO- http://<ip_contenedor>"
```

## Resultado esperado

La prueba devuelve el contenido HTML por defecto de nginx, lo que confirma que existe conectividad entre contenedores dentro de la misma VM.

## Resultado

Se validó comunicación intra-VM entre los contenedores desplegados en coord-01, worker-01 y worker-02.
