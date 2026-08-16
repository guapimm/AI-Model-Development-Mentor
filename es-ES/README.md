🌍 Otros idiomas → [English](../README.md)

# AI Model Mentor (Español)

> **Convierte a tu asistente de codificación con IA en un mentor full-stack prudente con 10 años de experiencia — solo prompts, cero dependencias.**

---

## ¿Qué es esto?

Un **framework de solo prompts (pure-prompt)** que moldea a tu asistente de codificación con IA en un **arquitecto full-stack y mentor de desarrollo con 10 años de experiencia**, diseñado para quienes empiezan a programar desde cero.

Obliga a la IA a seguir una serie de "reglas de hierro" que convierten en su comportamiento por defecto: *Seguridad ante todo, Lógica transparente, Documentación primero, Eficiencia de Token, Ejecución por fases y Control de recursos*. El resultado: una IA que no solo *escribe código*, sino que escribe código **seguro, mantenible y documentado**.

> ⚠️ Compatible con varias herramientas de IA: **opencode, Claude Code, Codex, Cursor, etc.** El contenido de los prompts es independiente de la herramienta; la guía de carga de cada herramienta está en [COMPATIBILITY.md](./COMPATIBILITY.md).

## Módulos principales (compatibles con varias herramientas)

| Módulo | Archivo | Propósito |
|--------|---------|-----------|
| 🧑‍🏫 Rol de mentor | [AGENTS.md](./prompts/AGENTS.md) | Persona de arquitecto-mentor + 6 reglas de hierro + lista de verificación de seguridad y rendimiento ★ núcleo, de uso obligatorio |
| 🛡️ Especificación de seguridad | [security.md](./prompts/security.md) | 8 dominios de seguridad: gestión de secretos / validación de entradas / base de datos / XSS / sistema de archivos / solicitudes externas / manejo de errores / rendimiento y recursos |
| 🎨 Estilo de interacción | [style.md](./prompts/style.md) | Analogías cotidianas, etiquetas de fase, confirmar antes de ejecutar, complejidad progresiva |
| 📋 Flujo de trabajo de desarrollo | [workflow.md](./prompts/workflow.md) | Sistema de documentación / estimación de recursos / diseño de base de datos / protocolo de localización del frontend / despliegue y recuperación ante desastres / bucle de autocomprobación de pruebas / anclas de versión |

## 📦 Más documentos

- [COMPATIBILITY.md](./COMPATIBILITY.md) — guía de carga para cada herramienta de IA (opencode / Claude Code / Codex / Cursor, etc.)
- [Prompt-Completo-del-Mentor.md](./prompts/Prompt-Completo-del-Mentor.md) — prompt completo consolidado (todos los módulos fusionados)

## ⬇️ Instalación y uso de mentor CLI

**Opción A: binario de Go (recomendado, cero dependencias, multiplataforma)**

Descarga el ejecutable `mentor` de tu plataforma desde las Releases de GitHub (v0.1.0, compatible con Windows / Linux / macOS), colócalo en el PATH y ejecuta:

```bash
mentor install                        # asistente interactivo: elige idioma → elige módulos (agent por defecto) → detecta la herramienta automáticamente
mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
mentor add workflow                   # añade un módulo
mentor list                           # consulta los módulos instalados
mentor detect                         # detecta la herramienta de IA que usa el proyecto
mentor pack                           # genera un directorio de skills compatible
```

`mentor` escribe automáticamente el nombre y la ubicación de archivo correctos según la herramienta: opencode/Codex → `AGENTS.md`, Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`.

**Opción B: copia manual**

Según las instrucciones de [COMPATIBILITY.md](./COMPATIBILITY.md), copia los archivos de `prompts/` a las ubicaciones correspondientes del proyecto.

> Comandos soportados: `install` / `add` / `remove` / `list` / `detect` / `pack`; módulos: agent (por defecto) / security / style / workflow / complete; herramientas: opencode / claude-code / codex / cursor / other.

## 📖 Guía de uso (opencode)

### Comandos de un vistazo

| Escenario | Operación |
|------|------|
| Desarrollo diario | Entra en el proyecto → opencode carga AGENTS.md automáticamente → conversa con normalidad |
| Proyectos de ciclo largo | Mantén las reglas en AGENTS.md (actualízalas con `/init`) |
| Desconexión inesperada | Recupera la sesión con `opencode --continue`; las reglas siguen ahí |
| Abrir una sesión nueva a propósito | Solo inicia `opencode` — AGENTS.md se carga automáticamente |

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
@security.md
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
@workflow.md
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
@style.md
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
@workflow.md
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
| Pedir a la IA que traduzca un texto | No se necesita ningún módulo |
| Refactorizar de forma sencilla el código existente | La lista de seguridad de AGENTS.md ya lo cubre |

### 💡 Resumen en una frase

> AGENTS.md es la piel por defecto; los otros tres son complementos de efectos especiales: actívalos solo cuando los necesites y mantenlos apagados el resto del tiempo; ahorras Token y todo queda más limpio.

### Las 6 reglas de hierro

1. **El código es documentación** — todo el código lleva comentarios que explican el "porqué"
2. **Seguridad ante todo** — sin secretos codificados, validación estricta de entradas, consultas parametrizadas, prevención de XSS
3. **Cambios sin efectos destructivos** — analizar primero las dependencias, etiquetar las ediciones como 【Modificación obligatoria】 / 【Optimización opcional】
4. **Ejecución por fases** — nunca más de 300 líneas por salida, esperar confirmación en cada paso
5. **Aislamiento modular** — máximo 500 líneas por archivo, reservar interfaces de extensión
6. **Rendimiento y recursos por adelantado** — el diseño de la base de datos debe emitir a la vez el plan de índices; las consultas de listas llevan paginación por defecto; al iniciar el proyecto se hace la estimación de recursos en tres niveles (memoria/disco/CPU); las operaciones con gran consumo de memoria disponen de un mecanismo de liberación

## Inicio rápido (3 pasos)

```bash
# 1. Copia el rol de mentor a tu proyecto (cámbiale el nombre)
cp prompts/AGENTS.md AGENTS.md

# 2. (Recomendado) Añade también las especificaciones de seguridad, estilo y flujo de trabajo
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
```

3. Inicia opencode y di:

> "Soy un completo principiante. Esta es mi Especificación de Requisitos del Proyecto: nombre del proyecto ____, objetivos principales ____, roles de usuario ____, flujos de trabajo principales ____, datos que deben persistirse ____. Empieza desde la Fase 0: Preparación del Entorno y Selección de la Pila Tecnológica y guíame paso a paso."

La IA avanzará a través de "Diseño → Lógica principal → Interfaz → Pruebas", esperando tu confirmación en cada fase.

## Estructura de archivos

```
AI_Model_Development_Mentor/
├── README.md            # Página de entrada bilingüe
├── LICENSE              # Licencia MIT
├── zh-CN/               # Chino
│   ├── README.md        # Entrada en chino
│   ├── COMPATIBILITY.md # Guía de carga por herramienta
│   └── prompts/         # Contenido independiente de la herramienta
│       ├── AGENTS.md    # Rol de mentor (ZH)
│       ├── security.md  # Especificación de seguridad (ZH)
│       ├── style.md     # Estilo de interacción (ZH)
│       └── workflow.md  # Flujo de trabajo de desarrollo (ZH)
├── en-US/               # Inglés
│   ├── README.md        # Entrada en inglés
│   ├── COMPATIBILITY.md # Guía de carga por herramienta
│   └── prompts/         # Contenido independiente de la herramienta
│       ├── AGENTS.md    # Rol de mentor (EN)
│       ├── security.md  # Especificación de seguridad (EN)
│       ├── style.md     # Estilo de interacción (EN)
│       └── workflow.md  # Flujo de trabajo de desarrollo (EN)
└── es-ES/               # Español
    ├── README.md        # Entrada en español (este archivo)
    ├── COMPATIBILITY.md # Guía de carga por herramienta
    └── prompts/         # Contenido independiente de la herramienta
        ├── AGENTS.md    # Rol de mentor (ES)
        ├── security.md  # Especificación de seguridad (ES)
        ├── style.md     # Estilo de interacción (ES)
        └── workflow.md  # Flujo de trabajo de desarrollo (ES)
```

> 📦 Las herramientas nuevas se añaden como filas en [COMPATIBILITY.md](./COMPATIBILITY.md); ya no hace falta crear directorios por producto.

## Preguntas frecuentes

**P: ¿Necesito los 4 módulos?**
R: No. `AGENTS.md` es el único imprescindible. Añade `security.md` para una protección más sólida y `style.md` para una experiencia de conversación más agradable.

**P: ¿Funciona con otros productos de IA?**
R: Sí. El contenido de los prompts es independiente de la herramienta; cada herramienta solo cambia la forma de cargar los archivos (nombre, ubicación, comando). Consulta [COMPATIBILITY.md](./COMPATIBILITY.md) para ver la guía completa de carga de cada herramienta (opencode / Claude Code / Codex / Cursor, etc.).

## Licencia

[Licencia MIT](../LICENSE) © 2026 guapimm
