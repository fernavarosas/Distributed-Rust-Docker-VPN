# Sprint 2 Review

## Sprint

**Sprint 2: ZeroTier + Docker**

## Objetivo del sprint

Conectar las máquinas virtuales locales mediante ZeroTier, instalar Docker en cada nodo, desplegar dos contenedores por VM y validar la comunicación básica tanto entre nodos como entre contenedores.

## Trabajo realizado

Durante este sprint se completaron las siguientes actividades técnicas:

- Se instaló **ZeroTier CLI** en `coord-01`, `worker-01` y `worker-02`.
- Se creó la red virtual del proyecto en **ZeroTier Central**.
- Se unieron y autorizaron los tres nodos locales dentro de la red.
- Se verificó la asignación de direcciones IP de ZeroTier en cada VM.
- Se validó la conectividad entre nodos mediante pruebas de `ping`.
- Se instaló **Docker** en las tres máquinas virtuales.
- Se verificó el funcionamiento de Docker con pruebas básicas.
- Se desplegaron **2 contenedores por VM** utilizando `nginx`.
- Se validó la comunicación **intra-VM** entre contenedores usando la red interna de Docker.
- Se validó la comunicación **inter-VM** accediendo a servicios publicados por contenedores a través de las IPs de ZeroTier.

## Resultados obtenidos

Al finalizar el sprint, el entorno distribuido local quedó funcional en su capa de red y contenedores. Los nodos `coord-01`, `worker-01` y `worker-02` pudieron comunicarse entre sí por ZeroTier, y cada uno quedó preparado para ejecutar servicios en contenedores Docker.

También se comprobó que los contenedores pueden comunicarse dentro de la misma VM y que un servicio ejecutado en un contenedor de una máquina virtual puede ser alcanzado desde otra VM mediante la red privada del proyecto.

## Incidencias y observaciones

Durante la validación de conectividad con ZeroTier, inicialmente se presentó una falta temporal de respuesta entre nodos, aunque después la comunicación quedó funcional sin necesidad de cambios adicionales. Esto sugiere que el enlace entre peers terminó de estabilizarse unos momentos después de la unión a la red.

Como parte de las pruebas inter-VM, fue necesario publicar puertos en contenedores de prueba para exponer servicios al host y permitir el acceso desde otras VMs por ZeroTier. Esta configuración se utilizó únicamente como validación técnica del sprint y no representa todavía la arquitectura final del sistema distribuido en Rust.

## Estado del sprint

**Sprint 2 completado.**

## Entregables logrados

- Red ZeroTier creada y operativa entre nodos locales.
- Docker instalado y validado en `coord-01`, `worker-01` y `worker-02`.
- Dos contenedores desplegados por cada VM.
- Pruebas de conectividad intra-VM documentadas.
- Pruebas de conectividad inter-VM documentadas.
- Bitácora y documentación técnica actualizadas en el repositorio.

## Pendientes para el siguiente sprint

- Integrar las máquinas virtuales del compañero a la red del proyecto.
- Definir la estructura final del sistema **coordinador-workers**.
- Sustituir los contenedores de prueba por servicios alineados con la implementación real.
- Comenzar la implementación del programa distribuido en **Rust** para el algoritmo de **Mandelbrot**.
- Documentar la arquitectura final y las pruebas distribuidas del proyecto.

## Conclusión

Sprint 2 permitió dejar lista la base de comunicación y ejecución distribuida del proyecto. La infraestructura local ya cuenta con conectividad privada mediante ZeroTier y con soporte de contenedores mediante Docker, lo que habilita el inicio de la siguiente fase: la implementación del coordinador y los workers en Rust sobre un entorno ya validado.
