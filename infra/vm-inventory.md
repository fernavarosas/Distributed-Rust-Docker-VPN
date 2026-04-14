# Inventario de nodos del proyecto

| Nodo / Host | Rol en el sistema             | Contenedores finales                                   | Estado                    |
| ----------- | ----------------------------- | ------------------------------------------------------ | ------------------------- |
| coord-01    | Coordinador + host de workers | mandel-coordinator, mandel-worker-01, mandel-worker-02 | Operativo                 |
| worker-02   | Host de workers               | mandel-worker-03, mandel-worker-04                     | Operativo                 |
| omadebian   | Host de workers               | mandel-worker-05, mandel-worker-06                     | Operativo / en validación |
| wsl         | Host de workers               | mandel-worker-07, mandel-worker-08                     | Operativo / en validación |
| worker-01   | Nodo con incidencia           | No forma parte del despliegue final activo             | Incidencia documentada    |
