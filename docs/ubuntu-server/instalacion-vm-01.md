# Instalación de la VM coordinadora `coord-01`

## Objetivo

Crear la primera máquina virtual del proyecto con Ubuntu Server 24.04.4 LTS, sin entorno gráfico, para usarla como nodo coordinador base del entorno distribuido.

## Configuración usada

- **Nombre de la VM:** `coord-01`
- **Host físico:** Windows
- **Hipervisor:** VirtualBox
- **Sistema operativo invitado:** Ubuntu Server 24.04.4 LTS
- **Arquitectura:** 64-bit PC (AMD64)
- **Memoria RAM:** 2048 MB
- **Procesadores:** 2 vCPU
- **Disco duro virtual:** 20 GB dinámico
- **Adaptador de red:** NAT
- **Acceso remoto:** OpenSSH Server

## Procedimiento de instalación

1. Se descargó la imagen oficial **Ubuntu Server 24.04.4 LTS** en su versión **64-bit PC (AMD64) server install image**.
2. Se creó una nueva máquina virtual en VirtualBox con el nombre `coord-01`.
3. Se asignaron 2 GB de RAM, 2 CPU y un disco duro virtual de 20 GB.
4. Se montó la ISO de Ubuntu Server y se inició la instalación.
5. En la configuración de almacenamiento se eligió:
   - usar todo el disco
   - partición raíz `/`
   - formato `ext4`
   - sin LVM
   - sin cifrado
6. Se configuró el nombre del servidor como `coord-01`.
7. Se creó el usuario administrador del sistema `fernando`
8. Se instaló **OpenSSH Server** durante el proceso de instalación.
9. Al terminar, se reinició la VM y se verificó el sistema con:
   - `hostnamectl`
   - `ip a`
   - `systemctl status ssh`

## Verificaciones realizadas

- El hostname quedó configurado correctamente como `coord-01`.
- La interfaz de red `enp0s3` obtuvo dirección IP por NAT.
- El sistema operativo quedó instalado correctamente sin entorno gráfico.
- El servicio SSH quedó instalado y posteriormente se habilitó para administración por terminal.

## Resultado

La VM `coord-01` quedó creada e instalada correctamente como base del nodo coordinador.  
Se validó el hostname, la conectividad de red y la disponibilidad de SSH para administración remota. Esta VM servirá como plantilla base para clonar y preparar los nodos `worker-01` y `worker-02`.
