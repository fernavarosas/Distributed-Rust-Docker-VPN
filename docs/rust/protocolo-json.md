# Protocolo JSON entre coordinador y workers

## Objetivo

Definir el formato de intercambio de mensajes entre el contenedor coordinador y los contenedores worker para ejecutar el algoritmo Mandelbrot distribuido en Rust.

## Mensajes definidos

### 1. Registro del worker

Este mensaje es enviado por el worker al conectarse al coordinador.

```json
{
  "type": "register",
  "worker_id": "mandel-worker-01",
  "host_id": "worker-01"
}
```

### 2. Confirmación del coordinador

El coordinador responde al registro exitoso del worker.

```json
{
  "type": "ack",
  "message": "worker registered"
}
```

### 3. Tarea asignada

El coordinador envía al worker el bloque de filas que debe calcular.

```json
{
  "type": "task",
  "task_id": 1,
  "start_row": 0,
  "end_row": 99,
  "width": 800,
  "height": 800,
  "max_iter": 1000,
  "x_min": -2.0,
  "x_max": 1.0,
  "y_min": -1.5,
  "y_max": 1.5
}
```

### 4. Resultado del worker

El worker devuelve los valores calculados para el bloque asignado.

```json
{
  "type": "result",
  "task_id": 1,
  "worker_id": "mandel-worker-01",
  "start_row": 0,
  "end_row": 99,
  "pixels": [0, 1, 3, 8, 12, 255]
}
```

### 5. Finalización

Mensaje opcional para indicar que el worker terminó la tarea.

```json
{
  "type": "done",
  "task_id": 1,
  "worker_id": "mandel-worker-01"
}
```

### 6. Error

Mensaje enviado en caso de falla durante el procesamiento.

```json
{
  "type": "error",
  "worker_id": "mandel-worker-01",
  "task_id": 1,
  "message": "failed to compute assigned rows"
}
```

## Flujo general

1. El worker se conecta y envía register.
2. El coordinador responde con ack.
3. El coordinador envía task.
4. El worker procesa y devuelve result.
5. Opcionalmente, el worker envía done.

## Observación

El protocolo está diseñado para ser implementado en Rust mediante serde y serde_json.
