🌍 Otros idiomas → [English](../README.md)

# AI Model Mentor (Español)

> **Convierte a tu asistente de codificación con IA en un mentor full-stack prudente con 10 años de experiencia — solo prompts, cero dependencias.**

---

## ¿Qué es esto?

Un **framework de solo prompts (pure-prompt)** que moldea a tu asistente de codificación con IA en un **arquitecto full-stack y mentor de desarrollo con 10 años de experiencia**, diseñado para quienes empiezan a programar desde cero.

Obliga a la IA a seguir una serie de "reglas de hierro" que convierten en su comportamiento por defecto: *Seguridad ante todo, Lógica transparente, Documentación primero, Eficiencia de Token y Ejecución por fases*. El resultado: una IA que no solo *escribe código*, sino que escribe código **seguro, mantenible y documentado**.

> ⚠️ Actualmente compatible con: **Xiaomi MIMO CLI**. Se planean versiones optimizadas para otros productos (Claude Code, Cursor, etc.): deja un comentario si necesitas una.

## Módulos principales (versión Xiaomi MIMO)

| Módulo | Archivo | Propósito |
|--------|---------|-----------|
| 🧑‍🏫 Rol de mentor | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | Persona de arquitecto-mentor + 6 reglas de hierro + lista de verificación de seguridad ★ núcleo, de uso obligatorio |
| 🛡️ Especificación de seguridad | [security.md](./xiaomi-mimo/security.md) | 8 dominios de seguridad: secretos / validación de entradas / base de datos / XSS / sistema de archivos / solicitudes externas / manejo de errores / rendimiento |
| 🎨 Estilo de interacción | [style.md](./xiaomi-mimo/style.md) | Analogías cotidianas, etiquetas de fase, confirmar antes de ejecutar, complejidad progresiva |
| 📋 Flujo de trabajo de desarrollo | [workflow.md](./xiaomi-mimo/workflow.md) | Sistema de documentación / protocolo de mapeo del frontend / despliegue y reversión / bucle de pruebas / anclas de versión |

## 📦 Más documentos

- [Prompt-Completo-del-Mentor.md](./Prompt-Completo-del-Mentor.md) — prompt completo consolidado (todos los módulos fusionados)

## 📖 Guía de uso (MIMO CLI)

### Comandos de un vistazo

| Escenario | Operación |
|------|------|
| Desarrollo diario | Entra en el proyecto → `/skill AGENTS.md` → conversa con normalidad |
| Proyectos de ciclo largo | Después de la primera carga, usa `/dream` para asentar las reglas en MEMORY.md |
| Desconexión inesperada | Recupera la sesión con `mimo --continue`; las reglas del skill siguen ahí |
| Abrir una sesión nueva a propósito | Después de ejecutar `/new`, recuerda volver a cargar `/skill AGENTS.md` |

### Estructura de archivos del proyecto

```
📁 my-project/
├── 📄 AGENTS.md          ← prompt principal
├── 📄 security.md        ← normas de seguridad
├── 📄 workflow.md        ← normas del flujo de trabajo
├── 📄 style.md           ← estilo de interacción
└── 📁 src/
```

---

### Demostración de escenarios concretos

#### Escenario 1: escribir código a diario (cargar solo AGENTS.md)

> Tú: "Ayúdame a escribir una API para obtener la lista de usuarios"

Hay que cargar: AGENTS.md (ya se carga automáticamente, no hace falta hacer nada)

La IA hará automáticamente:

- Código con comentarios en español
- Marcar la lista de seguridad antes de emitir el código
- Ejecución por fases (≤300 líneas)
- Máximo 500 líneas por archivo

#### Escenario 2: escribir las interfaces de inicio de sesión/registro (cargar AGENTS.md + security.md)

> Tú: "Ayúdame a escribir la función de inicio de sesión de usuarios, siguiendo los requisitos de security.md"

Hay que cargar:

```bash
/skill security.md
```

La IA hará además:

- Almacenar las contraseñas con hash bcrypt
- Definir un tiempo de expiración para el JWT Token
- Prevenir los ataques de fuerza bruta (límite de intentos de inicio de sesión fallidos)
- Prevenir la inyección SQL (consultas parametrizadas)

#### Escenario 3: iniciar un proyecto desde cero (cargar AGENTS.md + workflow.md)

> Tú: "Quiero hacer un sistema de blog; siguiendo workflow.md, ayúdame a montar el esqueleto del proyecto"

Hay que cargar:

```bash
/skill workflow.md
```

La IA hará además:

- Crear docs/architecture.md (selección de la pila tecnológica + diagrama de arquitectura)
- Crear docs/dev_log.md (plantilla del registro de desarrollo)
- Crear docs/api_interface.md (plantilla del contrato de interfaces)
- Crear docs/SNAPSHOT.md (instantánea del proyecto)
- Generar los scripts backup.sh y rollback.sh

#### Escenario 4: las explicaciones de la IA son demasiado confusas (cargar style.md)

> Tú: "Siguiendo el estilo de style.md, explícame con analogías de la vida cotidiana qué es JWT"

Hay que cargar:

```bash
/skill style.md
```

La IA hará además:

- Explicar JWT con la analogía de la "tarjeta de socio del restaurante"
- Añadir la etiqueta de fase [📋 Análisis de requisitos]
- Dar primero la conclusión y luego los detalles
- Ofrecer 2-3 opciones alternativas

#### Escenario 5: desplegar y poner en producción (cargar AGENTS.md + workflow.md)

> Tú: "Siguiendo las normas de despliegue de workflow.md, ayúdame a escribir la configuración de despliegue con Docker"

Hay que cargar:

```bash
/skill workflow.md
```

La IA hará además:

- Distinguir entre la configuración de los entornos de desarrollo y de producción
- Generar docker-compose.yml
- Generar health_check.sh
- Recordar los pasos de copia de seguridad y de reversión

### ⚠️ ¿Cuándo no hace falta cargarlo?

| Situación en la que no hace falta cargarlo | Motivo |
|---------------|------|
| Hacer una pregunta puramente técnica (por ejemplo, "¿cómo se usa useEffect en React?") | AGENTS.md ya es suficiente; añadir workflow solo estorba |
| Modificar un estilo CSS | No se necesitan las normas de seguridad ni el proceso de despliegue |
| Pedir a la IA que traduzca un texto | No se necesita ningún skill |
| Refactorizar de forma sencilla el código existente | La lista de seguridad de AGENTS.md ya lo cubre |

### 💡 Resumen en una frase

> AGENTS.md es la piel por defecto; los otros tres son complementos de efectos especiales: actívalos solo cuando los necesites y mantenlos apagados el resto del tiempo; ahorras Token y todo queda más limpio.

### Las 6 reglas de hierro

1. **El código es documentación** — todo el código lleva comentarios que explican el "porqué"
2. **Seguridad ante todo** — sin secretos codificados, validación estricta de entradas, consultas parametrizadas, prevención de XSS
3. **Cambios sin efectos destructivos** — analizar primero las dependencias, etiquetar las ediciones como 【Modificación obligatoria】 / 【Optimización opcional】
4. **Ejecución por fases** — nunca más de 300 líneas por salida, esperar confirmación en cada paso
5. **Aislamiento modular** — máximo 500 líneas por archivo, reservar interfaces de extensión
6. **Eficiencia de Token** — generar un resumen de contexto y una contraseña de reanudación después de cada conversación

## Inicio rápido (3 pasos)

```bash
# 1. Copia el rol de mentor a tu proyecto (cámbiale el nombre)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (Recomendado) Añade también las especificaciones de seguridad, estilo y flujo de trabajo
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. Inicia Xiaomi MIMO y di:

> "Soy un completo principiante. Esta es mi Especificación de Requisitos del Proyecto: nombre del proyecto ____, objetivos principales ____, roles de usuario ____, flujos de trabajo principales ____, datos que deben persistirse ____. Empieza desde la Fase 0: Preparación del Entorno y Selección de la Pila Tecnológica y guíame paso a paso."

La IA avanzará a través de "Diseño → Lógica principal → Interfaz → Pruebas", esperando tu confirmación en cada fase.

## Estructura de archivos

```
AI_Model_Development_Mentor/
├── README.md            # Página de entrada bilingüe
├── LICENSE              # Licencia MIT
├── zh-CN/               # Chino
│   ├── README.md        # Entrada en chino
│   └── xiaomi-mimo/     # Versión Xiaomi MIMO
│       ├── AGENTS.md    # Rol de mentor (ZH)
│       ├── security.md  # Especificación de seguridad (ZH)
│       ├── style.md     # Estilo de interacción (ZH)
│       └── workflow.md  # Flujo de trabajo de desarrollo (ZH)
├── en-US/               # Inglés
│   ├── README.md        # Entrada en inglés
│   └── xiaomi-mimo/     # Versión Xiaomi MIMO
│       ├── AGENTS.md    # Rol de mentor (EN)
│       ├── security.md  # Especificación de seguridad (EN)
│       ├── style.md     # Estilo de interacción (EN)
│       └── workflow.md  # Flujo de trabajo de desarrollo (EN)
└── es-ES/               # Español
    ├── README.md        # Entrada en español (este archivo)
    └── xiaomi-mimo/     # Versión Xiaomi MIMO
        ├── AGENTS.md    # Rol de mentor (ES)
        ├── security.md  # Especificación de seguridad (ES)
        ├── style.md     # Estilo de interacción (ES)
        └── workflow.md  # Flujo de trabajo de desarrollo (ES)
```

> 📦 Las nuevas versiones para productos se añaden como directorios hermanos dentro de cada carpeta de idioma, p. ej. `zh-CN/claude-code/`, `en-US/cursor/`.

## Preguntas frecuentes

**P: ¿Necesito los 4 módulos?**
R: No. `AGENTS.md` es el único imprescindible. Añade `security.md` para una protección más sólida y `style.md` para una experiencia de conversación más agradable.

**P: ¿Funciona con otros productos de IA?**
R: Por ahora solo se admite Xiaomi MIMO. Las versiones optimizadas para otros productos están en proceso — deja un comentario para contarnos qué necesitas.

## Licencia

[Licencia MIT](../LICENSE) © 2026 guapimm
