# Tabla de mapeo de elementos UI + tabla de mapeo de eventos (obligatoria antes de escribir el frontend)

> La IA mentora la emite antes de escribir el código del frontend, para que los usuarios sin conocimientos previos puedan reportar problemas con precisión.
> Archívala en `docs/` y úsala junto con el contrato de API (`docs/api_interface.md`).

## 1. Boceto de la página (ASCII o Mermaid)

```
┌──────────────────────────────────────────┐
│  Barra de navegación superior (logo / menú / avatar) │
├───────────────┬──────────────────────────┤
│               │                          │
│   Barra       │     Contenido principal  │
│   lateral     │                          │
└───────────────┴──────────────────────────┘
```

## 2. Tabla de mapeo de elementos UI

| Ubicación visual | Componente | Ruta del archivo | Clase/ID CSS | Descripción |
|------------------|------------|------------------|--------------|-------------|
| Barra superior, derecha | UserAvatar | src/components/Header.tsx | .user-avatar | Avatar de usuario y menú desplegable (cerrar sesión, perfil) |
| | | | | |

## 3. Tabla de mapeo de eventos del frontend

| Nombre | Acción (clic/deslizar/entrada) | Endpoint del backend llamado | Resultado esperado |
|--------|--------------------------------|------------------------------|--------------------|
| Botón de inicio de sesión | Clic | POST /api/login | Redirigir al inicio tras la validación; mostrar error en caso de fallo |
| | | | |

## 4. Guía de uso (para usuarios sin conocimientos previos)

1. Para reportar un problema de la página, di simplemente «**ubicación** + **qué pasó**», por ejemplo:
   > "El avatar de la parte superior derecha no responde al hacer clic"
2. La IA mentora localizará el componente y el endpoint exactos con las dos tablas anteriores, sin necesidad de describir código.
