# Sprint 1 - Daily Log

## 2026-04-01

- Se creó la estructura base del repositorio.
- Se definieron las carpetas para documentación técnica y gestión del proyecto.
- Se preparó el proyecto para comenzar con la creación de la primera VM Ubuntu Server.

## 2026-04-02

- Se descargó la ISO oficial de Ubuntu Server 24.04.4 LTS para la creación de las VMs del proyecto.
- Se creó la VM `coord-01` en VirtualBox.
- Se instaló Ubuntu Server 24.04.4 LTS sin entorno gráfico.
- Se habilitó OpenSSH Server.
- Se verificó la base del nodo coordinador.
- Se completó la instalación y configuración base de `coord-01`.
- Se habilitó el servicio SSH para administración por terminal.
- Se actualizó el sistema y se dejó la VM lista para clonación.
- Se clonó correctamente la VM base `coord-01` para crear `worker-01` y `worker-02`.
- Se corrigieron hostnames y el archivo `/etc/hosts` en ambos clones.
- Se verificó conectividad básica, SSH activo y operación correcta por NAT.
