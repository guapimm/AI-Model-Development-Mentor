# Normas del flujo de trabajo de desarrollo

## 1. Inicialización del proyecto y sistema de documentación

### Modo ligero (código < 500 líneas)
Solo se necesita `README.md`, que incluya: introducción del proyecto, pila tecnológica, estructura de las tablas principales, lista de interfaces, pasos de despliegue.

### Modo estándar (código ≥ 500 líneas)
Al iniciar el proyecto, crear la siguiente estructura de documentación:

```
📁 /docs/
├── architecture.md      # Justificación de la elección de la pila tecnológica (con analogías cotidianas), diagrama de arquitectura del sistema (Mermaid), estructura de directorios
├── dev_log.md           # Registro de desarrollo: fecha, cambios, resultados de las pruebas, problemas conocidos y sus soluciones
├── api_interface.md     # Contrato de interfaces frontend-backend (URL, parámetros, valores de retorno, escenarios de excepción)
└── SNAPSHOT.md          # Instantánea principal (≤ 200 líneas): versiones de la pila tecnológica, lista de nombres de tablas, rutas de la API, diagramas de flujo del negocio
```

Optimización de Token: al final de cada conversación, generar el【Resumen de contexto】(progreso, nombres de variables, tareas pendientes, contraseña de reanudación).

## 2. Protocolo de posicionamiento visual del frontend

Antes de escribir código frontend, emitir la siguiente información de posicionamiento:

### 1. Diagrama de distribución de la página
Usar diagramas de cableado (wireframes) ASCII o árboles de componentes de Mermaid para definir la estructura de la página.

### 2. Tabla de mapeo de elementos de la UI

| Posición visual | Nombre del componente | Ruta del archivo correspondiente | Clase/ID CSS | Descripción de la función |
|---------|---------|------------|------------|---------|
| Parte derecha de la barra de navegación superior | UserAvatar | /src/components/Header.tsx | .user-avatar | Avatar del usuario y menú desplegable |

### 3. Tabla de mapeo de eventos del frontend

| Nombre | Acción | Interfaz backend invocada | Efecto esperado |
|-------|------|------------|---------|
| Botón de inicio de sesión | Clic | POST /api/login | Redirige a la página de inicio y guarda el Token |

## 3. Mecanismos de despliegue y recuperación ante desastres

### Copia de seguridad local
- Ofrecer un script de copia de seguridad con un solo clic `backup.sh`, que exporte código + configuración + base de datos a `./local_backup/`
- Antes de cada despliegue, comprobar que existe la copia de seguridad local; si no existe, rechazar el despliegue

### Reversión gradual en el servidor en la nube
- Antes de desplegar el código nuevo, comprimir automáticamente la versión anterior como `backup_[marca_de_tiempo].zip`
- Los tres pasos de la reversión de emergencia:
  1. `./rollback.sh latest` — descomprime la copia de seguridad más reciente
  2. `docker-compose restart` (o `pm2 restart all`)
  3. `./health_check.sh` — muestra el estado del servicio
- Registrar en `dev_log.md` la hora de la copia de seguridad, la ruta y el historial de reversiones

### Aislamiento de entornos
- Diferenciar las configuraciones de los entornos de desarrollo y de producción
- Avisar con antelación de los elementos de configuración de seguridad que hay que modificar en producción

## 4. Expansión de requisitos y sugerencias

Tras completar las funciones solicitadas por el usuario, emitir la Tarjeta de sugerencias de mejora de funciones:

- ✅ **Resumen de las funciones completadas** — explicar con claridad qué funciones están disponibles
- 🔮 **Avisos de riesgos potenciales** — concurrencia, consistencia de los datos, dependencias de terceros, etc.
- 🚀 **Funciones de expansión recomendadas** — marcar prioridad P0/P1/P2, dificultad de implementación en estrellas ⭐, efecto esperado
- ⚠️ **Guía para evitar errores de principiantes** — malentendidos habituales, precauciones de operación

## 5. Bucle cerrado de pruebas y autocomprobación

### Casos de prueba mínimos verificables
Ofrecer pasos de verificación que el usuario pueda realizar manualmente, por ejemplo:
> "Haz clic en el botón de inicio de sesión, introduce un usuario y una contraseña correctos y comprueba si se redirige correctamente a la página de inicio"

### Declaración de coherencia lógica
Tras emitir el código, hay que declarar:
> "He comprobado: ①el alcance de las variables es correcto ②el procesamiento asíncrono está completo ③la captura de excepciones es completa ④no hay fugas de información sensible ⑤no hay cuellos de botella de rendimiento evidentes"

## 6. Ancla de versiones

Cada vez que se complete un hito, emitir un mensaje de Git Commit estándar:
```
feat: módulo de inicio de sesión de usuarios completado
- implementa la autenticación con JWT Token
- añade el almacenamiento de contraseñas con hash
- validación del formulario de inicio de sesión en el frontend
Author: AI Assistant
Date: 2026-08-08
```
