# Sprint 2 - Daily Log

## 2026-11-02

## Avance: instalación inicial de ZeroTier en nodos locales

- Se instaló ZeroTier CLI en `coord-01`, `worker-01` y `worker-02`.
- Se verificó la versión instalada mediante `zerotier-cli -v`.
- Se comprobó que el servicio `zerotier-one` quedó habilitado y en ejecución en las tres VMs.
- Con esto, los tres nodos locales quedaron listos para unirse a la red virtual del proyecto en ZeroTier Central.

## Avance: creación de red ZeroTier y unión de nodos locales

- Se creó la red virtual del proyecto en ZeroTier Central con el nombre `proyecto1-vpn-docker-rust`.
- Se unieron correctamente los nodos `coord-01`, `worker-01` y `worker-02` a la red.
- Se autorizaron los nodos desde ZeroTier Central.
- En `coord-01` se verificó estado `OK` en `zerotier-cli listnetworks`.
- Se confirmó la creación de la interfaz virtual ZeroTier `zttqhvhdhn` y la asignación de una IP privada administrada por la red virtual.

## Avance: validación de conectividad entre nodos por ZeroTier

- Se verificó la conectividad entre `coord-01`, `worker-01` y `worker-02` usando sus direcciones IP asignadas por ZeroTier.
- Inicialmente se observó una falta temporal de respuesta en pruebas ICMP entre nodos.
- Después de ejecutar verificaciones de peers y estado de firewall, la conectividad quedó funcional sin requerir cambios adicionales.
- Se confirmó que los tres nodos establecieron comunicación directa dentro de la red virtual del proyecto.
- Con esto quedó completado el bloque base de ZeroTier para los nodos locales del Sprint 2.

## Avance: instalación y validación inicial de Docker

- Se instaló Docker en `coord-01`, `worker-01` y `worker-02`.
- Se verificó la versión instalada y el estado del servicio Docker en las tres VMs.
- Se ejecutó una prueba inicial con `hello-world` para confirmar funcionamiento básico.
- Posteriormente se desplegaron dos contenedores por cada VM usando la imagen `nginx`.
- Se validó la ejecución de los contenedores mediante `docker ps`.

## Avance: validación de comunicación entre contenedores dentro de la misma VM

- Se inspeccionaron las redes Docker y las direcciones IP internas de los contenedores desplegados.
- En cada VM se utilizó un contenedor auxiliar temporal para probar acceso HTTP a los contenedores `nginx`.
- Se confirmó comunicación exitosa entre contenedores dentro de `coord-01`, `worker-01` y `worker-02`.
- Con ello quedó validada la conectividad intra-VM a nivel de contenedores.

## Avance: validación de comunicación entre contenedores en distintas VMs

- Se desplegaron contenedores `nginx` con puertos publicados en nodos worker.
- Se verificó localmente que cada contenedor publicado respondiera por `localhost` en su respectiva VM.
- Posteriormente se realizaron pruebas desde `coord-01` hacia servicios publicados en `worker-01` y `worker-02` usando sus direcciones ZeroTier.
- Se confirmó que una VM puede acceder a servicios ejecutados dentro de contenedores alojados en otra VM a través de la red virtual del proyecto.
