# Validación de comunicación entre contenedores en distintas VMs

## Objetivo

Comprobar que los servicios ejecutados dentro de contenedores Docker en una máquina virtual pueden ser accedidos desde otra VM mediante la red ZeroTier.

## Estrategia

Dado que las IP internas de Docker (`172.17.x.x`) no son accesibles directamente desde otras VMs, se publicaron puertos de los contenedores al host para exponer el servicio `nginx`.

## Ejemplo de despliegue

En `worker-01`:

```bash
sudo docker run -d --name w1-pub -p 8081:80 nginx
```

En `worker-02`:

```bash
sudo docker run -d --name w2-pub -p 8082:80 nginx
```

## Verificación local

Se probó primero en cada VM:

```bash
wget -qO- http://localhost:8081
wget -qO- http://localhost:8082
```

## Verificación entre VMs

Desde `coord-01` se accedió a los servicios publicados utilizando las IPs ZeroTier de los workers:

```bash
wget -qO- http://10.236.62.109:8081
wget -qO- http://10.236.62.176:8082
```

## Resultado

Se validó comunicación inter-VM hacia servicios ejecutados dentro de contenedores Docker sobre la red ZeroTier.
