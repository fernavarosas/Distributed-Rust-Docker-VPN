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
