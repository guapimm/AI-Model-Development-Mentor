# Manual detallado de normas de seguridad

## 1. Gestión de claves secretas y configuración

- Prohibido codificar en el código cualquier clave secreta, contraseña o API Token
- Usar siempre variables de entorno; en el código solo se referencian los nombres de las variables
- Extraer todos los elementos de configuración al archivo `.env.example` (sin valores reales, solo los nombres de las variables)
- Añadir el archivo `.env` de producción al `.gitignore`

## 2. Validación de las entradas del usuario

- Todas las entradas del usuario deben pasar una validación de tipos (p. ej., rechazar cadenas de texto en campos numéricos)
- Establecer límites de longitud razonables (p. ej., nombre de usuario: 2-50 caracteres)
- Rechazar la inyección de caracteres especiales (palabras clave de SQL, etiquetas HTML, etc.)
- Restringir el tipo y el tamaño de los archivos subidos; validar el tipo MIME

## 3. Seguridad de la base de datos

- Usar obligatoriamente consultas parametrizadas o instrucciones precompiladas de ORM
- Prohibido concatenar cadenas de texto para construir SQL
- Los campos sensibles (contraseñas) deben almacenarse como hash (bcrypt/argon2)
- La contraseña de la cadena de conexión a la base de datos se define mediante una variable de entorno
- Forzar la configuración de un límite máximo para el pool de conexiones de la base de datos, evitando que el servicio colapse al agotarse las conexiones

## 4. Protección contra XSS en el frontend

- Todo el contenido renderizado dinámicamente debe pasar por un escape HTML
- Usar los mecanismos de escape integrados del framework (p. ej., `{}` de React, `{{}}` de Vue)
- Prohibido renderizar entradas del usuario directamente con `innerHTML` o `v-html`
- Las Cookies deben llevar los atributos `HttpOnly` y `Secure`

## 5. Seguridad del sistema de archivos

- Todas las operaciones con rutas de archivos deben validarse para evitar el cruce de directorios (`../`)
- Usar listas blancas para restringir el alcance de los directorios accesibles
- Renombrar los archivos subidos con un UUID aleatorio, sin conservar el nombre de archivo original
- Establecer un límite máximo estricto para el tamaño de cada archivo; los archivos muy grandes deben subirse obligatoriamente por fragmentos (chunking)

## 6. Seguridad de las solicitudes externas

- Todas las solicitudes HTTP deben tener un tiempo de espera definido (se recomienda 5-10 segundos)
- Implementar una estrategia de reintentos (máximo 3 veces, con retroceso exponencial)
- Verificar los certificados SSL; prohibido omitir la validación de certificados

## 7. Manejo de excepciones

- Todas las excepciones deben capturarse con try-catch
- En producción no se devuelve la pila de errores original al cliente
- Registrar los logs de errores (con fecha y hora, ID de solicitud, tipo de error)
- Registrar logs de auditoría para las operaciones sensibles (inicios de sesión fallidos, permisos insuficientes)

## 8. Seguridad del rendimiento y de los recursos

- Todas las interfaces de listas activan la paginación por defecto, con un límite máximo de elementos por página (por defecto 100); prohibido consultar el conjunto de datos completo
- Configurar la limitación de frecuencia (rate limiting) de las interfaces según el volumen de concurrencia previsto (a nivel de IP y de usuario), para prevenir ataques de agotamiento de recursos
- El procesamiento de archivos grandes o de grandes volúmenes de datos debe usar lectura y escritura por flujos (streaming), evitando cargarlo todo de una sola vez en memoria y provocar desbordamientos
- Los campos de consulta principales deben tener índices; prohibido el escaneo de tablas completas sin índices
- Limpiar periódicamente los logs caducados y los archivos temporales, para evitar que el uso del disco crezca sin límite
