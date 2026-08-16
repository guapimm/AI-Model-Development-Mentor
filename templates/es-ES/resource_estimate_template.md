# Tabla de estimación de recursos del proyecto (obligatoria en la Fase 0)

> Se rellena al inicio del proyecto, guiado por la IA mentora, como base para la selección de la pila tecnológica y la planificación del despliegue.
> Tras rellenarla, archiva esta tabla en `docs/architecture.md` y mantenla actualizada en fases posteriores.

## 1. Información básica del proyecto

| Elemento | Valor |
|----------|-------|
| Nombre del proyecto | |
| Líneas de código estimadas | (menos de 500 líneas habilita el «modo ligero», manteniendo solo un README.md) |
| Escala de usuarios objetivo | Uso personal / equipo pequeño / producto público |
| Pico de usuarios concurrentes | |
| Tipo de datos | Texto plano / imágenes / audio-vídeo / archivos grandes |

## 2. Estimación de recursos en tres niveles

| Dimensión | Mínimo (dev/demo) | Recomendado (lanzamiento pequeño) | Alta disponibilidad (producto público) |
|-----------|-------------------|-----------------------------------|----------------------------------------|
| Memoria | | | |
| Disco | | | |
| Núcleos de CPU | | | |
| Ancho de banda | | | |
| Base de datos | SQLite / en memoria | MySQL / PostgreSQL | Clúster + separación lectura/escritura |

## 3. Dependencias de servicios de terceros

| Servicio | Propósito | ¿Requerido? | ¿Suficiente el plan gratuito? |
|----------|-----------|-------------|-------------------------------|
| Servidor en la nube | | | |
| Almacenamiento de objetos (archivos/imágenes) | | | |
| SMS / correo | | | |
| Pago | | | |
| Otro | | | |

## 4. Plan de rendimiento y recursos

- [ ] Los endpoints de listado paginan por defecto; sin escaneos de tabla completa
- [ ] El diseño de la base de datos incluye un plan de índices
- [ ] Las operaciones con archivos/datos grandes usan streaming
- [ ] Las operaciones con memoria grande tienen un mecanismo de liberación explícito
- [ ] Las peticiones externas establecen políticas de tiempo de espera y reintentos

## 5. Estimación de coste mensual

| Elemento | Mínimo | Recomendado |
|----------|--------|-------------|
| Servidor | | |
| Almacenamiento | | |
| Servicios de terceros | | |
| **Total** | | |
