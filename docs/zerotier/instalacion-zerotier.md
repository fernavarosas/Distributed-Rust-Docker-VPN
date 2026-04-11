# Instalación de ZeroTier en nodos locales

## Objetivo

Preparar los nodos locales del proyecto para integrarse a una red privada virtual distribuida mediante ZeroTier CLI.

## Nodos intervenidos

- coord-01
- worker-01
- worker-02

## Procedimiento

En cada máquina virtual se ejecutó el instalador oficial:

```bash
curl -s https://install.zerotier.com | sudo bash
```

Después se verificó la instalación con:

```bash
zerotier-cli -v
sudo systemctl status zerotier-one
```

## Resultado

ZeroTier CLI quedó instalado correctamente en las tres VMs locales y el servicio zerotier-one quedó activo y habilitado para el arranque.
