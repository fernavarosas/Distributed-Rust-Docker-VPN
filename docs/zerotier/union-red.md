# Unión de nodos a la red ZeroTier

## Objetivo

Integrar los nodos locales `coord-01`, `worker-01` y `worker-02` a la red virtual del proyecto para habilitar conectividad privada entre ellos.

## Red utilizada

- Nombre: `proyecto1-vpn-docker-rust`

## Procedimiento

Una vez creada la red en ZeroTier Central, cada nodo se unió con el comando:

```bash
sudo zerotier-cli join <NETWORK_ID>
```

Posteriormente se autorizó cada nodo desde la consola de administración web.

## Verificación

Para validar la unión se utilizó:

```bash
sudo zerotier-cli listnetworks
ip a
```

## Resultado observado

- El nodo coord-01 quedó con estado OK
- Se creó una interfaz virtual zt...
- Se asignó una IP privada de ZeroTier al nodo
- Los workers también fueron incorporados a la misma red y quedaron listos para validar conectividad
