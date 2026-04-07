# Clonación de `worker-01` y `worker-02` a partir de `coord-01`

## Objetivo

Aprovechar la configuración ya validada de la máquina virtual `coord-01` para generar rápidamente dos nodos adicionales del proyecto, `worker-01` y `worker-02`, conservando la misma base de Ubuntu Server y ajustando únicamente los parámetros necesarios para que cada máquina funcione de manera independiente.

## Máquina base utilizada

La máquina virtual `coord-01` se utilizó como plantilla base porque ya contaba con:

- Ubuntu Server 24.04.4 LTS instalado correctamente
- configuración sin entorno gráfico
- servicio SSH habilitado
- conectividad de red funcional por NAT
- sistema actualizado y verificado

Esto permitió evitar reinstalar el sistema operativo desde cero en cada worker y facilitó la preparación del entorno para las siguientes fases del proyecto.

## Proceso de clonación

La clonación se realizó desde VirtualBox a partir de la máquina `coord-01`, generando dos nuevas máquinas virtuales:

- `worker-01`
- `worker-02`

Durante el proceso de clonación se configuró la opción para generar nuevas direcciones MAC en los adaptadores de red, con el fin de evitar conflictos de identificación entre máquinas virtuales.

## Ajustes realizados después de la clonación

Una vez creados los clones, se iniciaron individualmente y se realizaron los siguientes cambios en cada uno:

### 1. Cambio de hostname

Se actualizó el nombre del sistema para que coincidiera con la función de cada nodo:

- `worker-01`
- `worker-02`

Para ello se utilizó el comando:

```bash
sudo hostnamectl set-hostname worker-01
sudo hostnamectl set-hostname worker-02
```

### 2. Corrección del archivo /etc/hosts

Después de clonar la máquina base, los nodos heredaron la referencia al hostname anterior (coord-01) en el archivo /etc/hosts.
Por ello, fue necesario editar este archivo y reemplazar la entrada correspondiente para reflejar el nuevo nombre del sistema.

Ejemplo de ajuste realizado:

Antes:

- 127.0.1.1 coord-01

Después en worker-01:

- 127.0.1.1 worker-01

Después en worker-02:

- 127.0.1.1 worker-02

## Verificaciones realizadas

Después de aplicar los cambios, se verificó en cada VM:

- hostname correcto mediante hostnamectl
- interfaz de red activa con ip a
- servicio SSH disponible con systemctl status ssh

También se comprobó que ambos nodos mostraban direcciones MAC distintas, confirmando que cada worker quedó identificado de manera independiente en la red virtual.

## Observación sobre la dirección IP

En ambos workers se observó la dirección IP 10.0.2.15 en la interfaz enp0s3.
Esto es normal en el modo NAT de VirtualBox y no representa un conflicto en esta etapa, ya que cada máquina virtual opera detrás de su propio entorno NAT. Más adelante, la identificación principal entre nodos se realizará mediante las direcciones virtuales asignadas por ZeroTier.

## Resultado

Las máquinas virtuales worker-01 y worker-02 quedaron creadas y configuradas correctamente a partir de coord-01, con hostnames independientes, archivo /etc/hosts corregido, direcciones MAC distintas y conectividad básica funcional. Con esto, el host local de Fernando quedó preparado con tres nodos listos para la siguiente fase de integración mediante ZeroTier CLI.
