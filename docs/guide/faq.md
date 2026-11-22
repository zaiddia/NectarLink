# Preguntas frecuentes

## General y cuentas

### ¿Tengo que crear una cuenta o iniciar sesión para usar Panal?

No. Panal no tiene cuentas de usuario, ni pantallas de inicio de sesión, ni servidores en la nube. Cada dispositivo genera su propia clave de identidad criptográfica Ed25519 en local al arrancar por primera vez, y enlazas los dispositivos directamente escaneando un código QR o comparando un código de 6 dígitos.

### ¿Panal es gratuita y de código abierto?

Sí. Las aplicaciones de Windows y Android y la línea de comandos tienen licencia [GPL-3.0-or-later](../../LICENSES/GPL-3.0-or-later.txt) (con una excepción para la distribución en tiendas de apps), los crates compartidos del núcleo en Rust tienen licencia [MPL-2.0](../../LICENSES/MPL-2.0.txt) y las especificaciones del protocolo y la documentación tienen licencia [CC BY 4.0](../../LICENSES/CC-BY-4.0.txt). No hay niveles de pago, suscripciones ni anuncios.

### ¿Panal es lo mismo que NectarLink?

Panal es una versión derivada, en español y con su propio nombre, basada en NectarLink. No es la app oficial de NectarLink ni la publica su autor. Su ID de Android es `app.panal`, así que puede instalarse junto a la app oficial, pero cada app solo se empareja con su propia contraparte: instala Panal tanto en el PC como en el teléfono.

### ¿Panal solo funciona con teléfonos Samsung o Pixel?

No. Panal funciona con cualquier teléfono o tableta con Android 8.0 (API 26) o posterior, sea cual sea el fabricante (Samsung, Google Pixel, Xiaomi, OnePlus, Motorola, Sony, Nothing, Asus, Oppo, Realme, Vivo, Huawei y otros).

## Red y conectividad

### ¿El teléfono y el PC tienen que estar en la misma red Wi-Fi?

Por defecto, sí: Panal se conecta directamente por tu red local (Wi-Fi o Ethernet) o por un cable USB de datos (`Por USB`). Si quieres que el teléfono y el PC sigan conectados cuando sales de casa con datos móviles (LTE/5G) o usas otra red Wi-Fi, activa **Conectar con datos móviles** en **Ajustes › Conexión y Acerca de** del PC y en **Ajustes › Dispositivos** del teléfono.

### ¿Mis datos pasan por un servidor cuando uso **Conectar con datos móviles**?

Con **Conectar con datos móviles** activado y sin camino UDP directo entre tus redes, los paquetes QUIC cifrados se reenvían a través de un relé `iroh`. Como la sesión TLS 1.3 está cifrada de extremo a extremo entre la clave del teléfono y la del PC, el relé solo reenvía paquetes cifrados opacos y nunca puede leer tus notificaciones, mensajes, portapapeles ni archivos. Cuando los dispositivos están en la misma red local o conectados por USB, todo el tráfico entre ellos va directo sin salir de tu red local (la única petición a internet que hace Panal por defecto en las copias instaladas es una consulta HTTPS periódica a GitHub Releases para buscar actualizaciones, que puedes desactivar en el PC en **Ajustes › Conexión y Acerca de › Buscar actualizaciones**).

### ¿Puedo usar Panal sin Wi-Fi, solo con un cable USB?

Sí. El USB necesita `adb` de Android en el PC con su servidor en marcha: instala las platform-tools de Google y ejecuta `adb start-server` después de cada reinicio (Android Studio y scrcpy también lo inician). Panal no lo inicia por sí misma. Después activa la **Depuración USB** en las opciones de desarrollador del teléfono y conéctalo al PC con un cable USB de datos. Panal detecta sola la conexión USB, te deja emparejar con un código de confirmación de 6 dígitos y lleva toda la sesión cifrada (`Por USB`) por el cable.

### ¿Qué pasa si envío un archivo o copio un enlace mientras el otro dispositivo está dormido o desconectado?

No se pierde. Panal pone los archivos, carpetas, envíos del portapapeles, enlaces de `Handoff` y avisos de tareas `notify-when` en una cola persistente `Pendiente de enviar` guardada en SQLite. Puedes ver y cancelar los elementos en cola en la pantalla de Inicio de cualquiera de las dos apps, y se entregan solos y en orden en cuanto el otro dispositivo se reconecta.

### ¿Puedo emparejar varios teléfonos con un PC, o un teléfono con varios PC?

Sí. Puedes emparejar dos o más teléfonos (por ejemplo, uno personal y uno del trabajo, o un teléfono y una tableta Android) con el mismo PC, y puedes emparejar tu teléfono con un PC de sobremesa y un portátil a la vez. Los dispositivos conectados aparecen primero en el selector de dispositivos de Inicio, y **Ajustes › Dispositivos** te permite configurar los permisos de funciones de cada dispositivo emparejado por separado.

## Niveles de control y permisos

### ¿Tengo que rootear el teléfono para usar Panal?

No. Todas las funciones del día a día funcionan con el nivel de control predeterminado `Standard` y los permisos normales de Android, y las funciones avanzadas (`Elevated`) usan la depuración inalámbrica del propio Android, que ejecuta un pequeño ayudante como usuario shell (`UID 2000`).

Si tu teléfono ya tiene root (Magisk, KernelSU o APatch), puedes elegir **Usar root** al configurar Elevated. Panal inicia entonces el mismo ayudante a través de root, igualmente como usuario shell y con los mismos permisos, así que no necesita emparejamiento y vuelve solo tras un reinicio o fuera de la Wi-Fi. Panal nunca pide root salvo que elijas esta opción.

Si ya usas [Shizuku](https://shizuku.rikka.app/), puedes elegir **Usar Shizuku** (aparece cuando la app Shizuku está instalada). Shizuku pregunta una vez si Panal puede usarlo; después, Elevated arranca a través de Shizuku siempre que este funcione, sin emparejamiento propio, y sigue funcionando fuera de la Wi-Fi. El ayudante es el mismo y se ejecuta como usuario shell. Si Shizuku se ejecuta con root, Panal no lo usa: inicia Shizuku con la depuración inalámbrica o elige **Usar root**. Tras un reinicio, vuelve a iniciar Shizuku y abre Panal una vez: Shizuku solo avisa a las apps de que está en marcha cuando pasan a primer plano, y Elevated arranca en cuanto se entera.

### ¿Por qué Android 13+ pone en gris el interruptor de Acceso a notificaciones o Accesibilidad ("Ajustes restringidos")?

Android 13 y posterior impiden que los APK instalados a mano (fuera de una tienda de apps) activen servicios de escucha de notificaciones o de accesibilidad hasta que lo confirmes en **Información de la app**. Abre **Ajustes de Android › Aplicaciones › Panal**, toca el menú de tres puntos de la esquina superior derecha, toca **Permitir ajustes restringidos**, verifica con tu huella o PIN y vuelve a Panal para activar el interruptor.

### ¿Tengo que volver a emparejar la depuración inalámbrica (`Elevated`) cada vez que reinicio el teléfono?

No. Solo introduces el código de 6 dígitos de la depuración inalámbrica una vez, en la configuración inicial. Después, cada vez que el teléfono se reinicia o se reconecta a la Wi-Fi, Panal vuelve a activar sola la depuración inalámbrica e inicia el ayudante `Elevated` en segundo plano.

### ¿Puedo usar Panal sin conceder permisos de SMS, llamadas o fotos?

Sí. Todos los permisos de Android son opcionales, y la matriz de capacidades de Panal adapta la interfaz para mostrar solo lo que has activado. Por ejemplo, si solo quieres sincronizar el portapapeles, transferir archivos y duplicar la pantalla, puedes saltarte por completo los permisos de SMS, Llamadas, Contactos y Fotos.

## Batería, memoria y rendimiento

### ¿Cuánta batería gasta Panal en el teléfono?

Conectada e inactiva en segundo plano, Panal mantiene una única conexión QUIC ligera que solo se despierta con eventos y sin bucles de sondeo, y gasta menos del 1 % de batería por hora en pruebas de conexión continua. Las funciones de alto consumo como la duplicación de pantalla (`mirror`) y la transmisión de cámara (**Teléfono como webcam**) usan codificadores de vídeo por hardware solo mientras transmiten y se detienen al momento al cerrar la ventana.

### ¿Panal funciona en Linux o en Windows 7?

En Linux, sí, desde la 0.1.5: Ubuntu 24.04 o posterior, Debian 13, Fedora 40 o posterior y otras distribuciones de 2024 en adelante. Consulta [Primeros pasos](getting-started.md) y las diferencias en [Limitaciones conocidas](known-limitations.md#en-linux). Windows requiere Windows 10 versión 2004 o posterior, así que Windows 7 y 8 no son compatibles.

### ¿Cuánta RAM y CPU usa la app de Windows en segundo plano?

Iniciada minimizada en la bandeja del sistema de Windows, Panal usa aproximadamente **27 MB** de memoria privada comprometida (unos 5-10 MB de memoria activa) y **0,0 %** de CPU en reposo. Al cerrar la ventana principal (`Seguir en la bandeja del sistema`), Panal descarga el árbol de ventanas QML y las cachés de imágenes para mantener al mínimo la memoria en segundo plano, y sigue lista para mostrar notificaciones o abrir la `Bandeja` y la `Paleta de comandos` en menos de 50 milisegundos.

## Privacidad y gestión de datos

### ¿Panal envía el portapapeles cuando copio una contraseña de mi gestor de contraseñas?

No. Los gestores de contraseñas de Windows (como 1Password, Bitwarden y KeePassXC) y de Android 13+ marcan las contraseñas copiadas con marcas de privacidad del sistema (`ExcludeClipboardContentFromMonitorProcessing` en Windows y `ClipDescription.EXTRA_IS_SENSITIVE` en Android). Panal comprueba esas marcas antes de leer o enviar el portapapeles y se salta cualquier elemento sensible. Los códigos de verificación de un solo uso (OTP) también se excluyen del historial del portapapeles y de la `Línea de tiempo`.

### ¿Cómo borro mi historial local o desemparejo un dispositivo?

- Para borrar el historial y las cachés locales sin tocar los dispositivos emparejados ni los ajustes, abre **Ajustes › Almacenamiento y privacidad** en Windows (o **Ajustes › Funciones y datos › Datos y almacenamiento** en Android) y haz clic en **Borrar todo...** (o en **Borrar** junto a una categoría concreta como **Historial del portapapeles**, **Historial de notificaciones**, **Línea de tiempo entre dispositivos**, **Mensajes en caché** o **Miniaturas de fotos**).
- Para desemparejar un dispositivo, abre **Ajustes › Dispositivos** en cualquiera de las dos apps y haz clic en **Desemparejar**. Ambos dispositivos se quitan mutuamente de su almacén de confianza y descartan los elementos en cola para el otro.
