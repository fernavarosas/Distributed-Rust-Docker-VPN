# Validación de conectividad entre nodos ZeroTier

## Objetivo

Comprobar que los nodos locales `coord-01`, `worker-01` y `worker-02` pudieran comunicarse entre sí mediante la red privada virtual creada con ZeroTier.

## Direcciones IP identificadas

- `coord-01`: `10.236.62.155`
- `worker-01`: `10.236.62.109`
- `worker-02`: `10.236.62.176`

## Verificaciones realizadas

En los tres nodos se utilizó:

```bash
sudo zerotier-cli peers
sudo ufw status
ping -c 4 <IP_DEL_NODO_DESTINO>
```

## Observaciones

Durante la primera prueba, algunos pings devolvieron Destination Host Unreachable. Sin embargo, tras unos momentos y sin aplicar cambios adicionales, los nodos comenzaron a responder correctamente.

Esto indica que la red ZeroTier ya había sido creada y autorizada correctamente, pero el establecimiento completo de conectividad entre peers tomó un breve tiempo adicional.

## Resultado

Se validó conectividad exitosa entre los tres nodos locales a través de ZeroTier.
