# Instalación de Docker en nodos locales

## Objetivo

Instalar Docker en los nodos locales del proyecto para preparar el entorno de contenedores distribuidos.

## Nodos intervenidos

- coord-01
- worker-01
- worker-02

## Procedimiento

En cada nodo se ejecutó:

```bash
sudo apt update
sudo apt install -y docker.io
sudo systemctl enable --now docker
```

Después se verificó con:

```bash
sudo docker --version
sudo systemctl status docker
sudo docker run hello-world
```

## Resultado

Docker quedó instalado y operativo en las tres VMs locales.
