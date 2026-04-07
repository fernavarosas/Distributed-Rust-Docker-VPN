# Sprint 1 - Review

## Objetivo del sprint

Definir la arquitectura base del proyecto, crear las máquinas virtuales Ubuntu Server sin entorno gráfico necesarias en el host local de Fernando, verificar su administración por terminal y dejar documentada la evidencia inicial del entorno.

## Trabajo completado

Durante este sprint se logró establecer la base de infraestructura del proyecto en el host local de Fernando. Se creó y validó la máquina virtual `coord-01` con Ubuntu Server 24.04.4 LTS, sin entorno gráfico, con acceso por terminal y servicio SSH habilitado. A partir de esta máquina base se clonaron correctamente `worker-01` y `worker-02`, realizando los ajustes necesarios de hostname y del archivo `/etc/hosts` para que cada nodo funcionara de manera independiente.

También se dejó preparada la estructura del repositorio del proyecto en GitHub, incluyendo carpetas para documentación técnica, gestión del proyecto, evidencias e inventario de infraestructura. Además, se configuró la organización inicial del trabajo en GitHub mediante milestones, labels, board e issues, alineando el avance técnico con el enfoque Scrum definido para el proyecto.

## Evidencia generada

Se documentó la instalación de la VM base `coord-01`, la clonación de `worker-01` y `worker-02`, la activación de SSH y las verificaciones iniciales de hostname, red y estado de servicio. También se actualizó la bitácora del sprint y el inventario de máquinas virtuales.

## Incidencias o ajustes realizados

Durante la instalación se observó que el servicio SSH estaba instalado pero inactivo en `coord-01`, por lo que fue necesario habilitarlo manualmente. En los clones también se corrigió la referencia heredada del hostname anterior dentro de `/etc/hosts`. Asimismo, se verificó que la dirección `10.0.2.15` mostrada en `enp0s3` en los workers correspondía al comportamiento normal del modo NAT de VirtualBox y no a un conflicto de red.

## Resultado del sprint

El Sprint 1 quedó cumplido de forma satisfactoria en el host local de Fernando, ya que se consiguió la infraestructura base requerida para continuar con la integración de red del proyecto. Las máquinas `coord-01`, `worker-01` y `worker-02` quedaron listas para la siguiente etapa de trabajo.

## Trabajo pendiente

Queda pendiente que el host del compañero complete la creación de sus máquinas virtuales `worker-03` y `worker-04` para contar con los cinco nodos planeados en la arquitectura general del proyecto.

## Siguiente paso

El siguiente bloque de trabajo corresponde al inicio del Sprint 2, enfocado en instalar ZeroTier CLI en las máquinas virtuales, crear la red privada virtual, unir los nodos, autorizar los dispositivos y validar la conectividad entre ellos.
