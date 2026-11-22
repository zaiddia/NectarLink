# Solución de problemas

Si el teléfono y el PC no se encuentran, se desconectan cuando se apaga la pantalla del teléfono o tienes problemas con una función concreta, sigue los pasos de esta página.

## 1. Ejecuta primero el `Doctor de conexión`

Panal incluye en Windows una herramienta de diagnóstico que detecta los bloqueos de red más habituales y arregla lo que puede.

1. En el PC, abre **Ajustes › Conexión y Acerca de**.
2. Junto a **Doctor de conexión**, haz clic en **Comprobar**.
3. El `Doctor de conexión` revisa cuatro cosas y muestra los resultados en lenguaje sencillo, con botones de acción:
   - **Firewall de Windows o de Linux**: comprueba si `panal` puede recibir paquetes UDP (QUIC) y mDNS entrantes (Firewall de Windows en Windows, o `ufw` / `firewalld` en Linux). Si falta una regla o está bloqueada, haz clic en **Arreglar** para registrarla (requiere aprobación de administrador / Polkit).
   - **Perfil de red y descubrimiento local (`Público` o `Privado` en Windows, `avahi-daemon` en Linux)**: Windows 11 suele marcar por defecto las redes Wi-Fi de casa como **Públicas**, lo que bloquea el descubrimiento de dispositivos locales y las conexiones entrantes. Si tu Wi-Fi de casa está como **Pública**, cámbiala a **Privada** en **Configuración de Windows › Red e Internet › Wi-Fi › Propiedades de `<Tu red>` › Tipo de perfil de red**. En Linux, el Doctor de conexión también comprueba que estén disponibles `avahi-daemon` (mDNS), `fusermount3` (montaje del almacenamiento del teléfono) y PipeWire / PulseAudio.
   - **Interfaces VPN activas**: las VPN corporativas o de privacidad de túnel completo suelen enviar el tráfico de la subred local (`192.168.x.x` / `10.x.x.x`) por el túnel, así que el PC y el teléfono no pueden verse directamente. Activa el **túnel dividido** (o "Permitir tráfico de la red local") en los ajustes del cliente VPN, conecta el teléfono por cable USB (`Por USB`) o activa **Conectar con datos móviles** en ambos dispositivos.
   - **Posible aislamiento de puntos de acceso Wi-Fi (Wi-Fi de invitados o de campus)**: si el PC oye los anuncios mDNS del teléfono (en los últimos 2 minutos) pero todos los intentos de conexión QUIC directa caducan, tu router Wi-Fi tiene activado el **aislamiento de AP** (aislamiento de clientes). Es habitual en hoteles, campus, cafeterías y redes Wi-Fi de "Invitados". Para conectar en una Wi-Fi aislada, pasa ambos dispositivos a la Wi-Fi principal de casa, conecta un cable USB o activa **Conectar con datos móviles** en ambos dispositivos.

## 2. El teléfono se desconecta con la pantalla apagada (guías de batería por fabricante)

Si Panal sigue conectada mientras la app de Android está abierta en pantalla, pero pierde la conexión unos minutos después de bloquear el teléfono (y las respuestas rápidas, las notificaciones o las transferencias desde el PC esperan o caducan), Android o el gestor de batería del fabricante está durmiendo a Panal.

### Paso 1: permite el acceso estándar en segundo plano de Android

1. En el teléfono, abre **Panal › Ajustes › Permisos › Segundo plano y batería**.
2. Junto a **Seguir disponible en segundo plano**, toca **Permitir** y confirma el aviso de Android para que Panal funcione sin restricciones de batería (`Sin restricciones`).

### Paso 2: aplica el ajuste de batería extra de tu marca

Varios fabricantes de Android añaden sus propios cierres de tareas en segundo plano encima del Android estándar. Panal detecta el fabricante del teléfono y muestra la guía correspondiente dentro de **Ajustes › Permisos › Segundo plano y batería** (toca allí **Ajustes de batería** para ir directamente a la pantalla del sistema):

| Marca | Ajustes necesarios |
| :--- | :--- |
| **Samsung (One UI)** | 1. Abre **Ajustes de Android › Aplicaciones › Panal › Batería** y elige **Sin restricciones**.<br>2. Abre **Ajustes de Android › Mantenimiento del dispositivo › Batería › Límites de uso en segundo plano** y asegúrate de que Panal no está en **Aplicaciones inactivas** ni en **Aplicaciones en inactividad profunda** (añádela a **Aplicaciones que nunca estarán inactivas**). |
| **Google Pixel** | Abre **Ajustes de Android › Aplicaciones › Panal › Uso de batería de la aplicación**, toca **Permitir uso en segundo plano** (el texto de la fila, no solo el interruptor) y elige **Sin restricciones**. |
| **Xiaomi / Redmi / POCO (HyperOS / MIUI)** | 1. Abre **Ajustes de Android › Aplicaciones › Administrar aplicaciones › Panal** y activa **Inicio automático** (o **Inicio automático en segundo plano**).<br>2. Toca **Ahorro de batería** y elige **Sin restricciones**. |
| **OnePlus (OxygenOS)** | Abre **Ajustes de Android › Aplicaciones › Gestión de aplicaciones › Panal › Uso de batería** y activa **Permitir actividad en segundo plano** y **Permitir inicio automático**. |
| **Oppo / Realme (ColorOS / Realme UI)** | Abre **Ajustes de Android › Aplicaciones › Gestión de aplicaciones › Panal › Uso de batería** y activa **Permitir actividad en segundo plano** y **Permitir inicio automático**. |
| **Vivo / iQOO (Funtouch OS / OriginOS)** | Abre **Ajustes de Android › Aplicaciones › Panal › Batería**, elige **Sin restricciones** (o **Alto consumo en segundo plano**) y activa **Inicio automático** en la gestión del consumo en segundo plano. |
| **Huawei / Honor (EMUI)** | Abre **Gestor del teléfono (u Optimizador del sistema) › Inicio de aplicaciones**, desactiva **Gestionar automáticamente** para Panal y deja activados los tres interruptores (**Inicio automático**, **Inicio secundario** y **Ejecutar en segundo plano**). |

Los nombres exactos de los menús pueden variar un poco según la versión y el idioma del sistema.

## 3. Instalar el APK a mano: bloqueos de Play Protect y "Ajustes restringidos" de Android 13+

### Google Play Protect bloquea el APK sin botón "Instalar de todos modos"

Cuando descargas `Panal-<versión>-android.apk` en un navegador del móvil, Google Play Protect puede mostrar un aviso:

- **Si aparece "Más detalles"**: toca **Más detalles › Instalar de todos modos**.
- **Si no hay botón "Instalar de todos modos" (solo "Aceptar")**: la **Protección mejorada contra fraudes** de Google Play Protect, activa en 185 mercados (India entre ellos), bloquea solos los APK instalados desde el navegador que declaran `READ_SMS`, `BIND_NOTIFICATION_LISTENER_SERVICE` o `BIND_ACCESSIBILITY_SERVICE` en `AndroidManifest.xml`. Para instalarlo:
  1. **Opción A (en el teléfono)**: abre **Google Play Store**, toca el **icono de tu perfil** (arriba a la derecha) › **Play Protect** › **Ajustes** (icono de engranaje) y desactiva temporalmente **Analizar aplicaciones con Play Protect** (elige **Pausar**). Instala `Panal-<versión>-android.apk` y vuelve a activar **Analizar aplicaciones con Play Protect**.
  2. **Opción B (por USB desde el PC)**: conecta el teléfono con la depuración USB activada y ejecuta `adb install -r Panal-<versión>-android.apk`.

### "Ajustes restringidos" de Android 13+ (interruptor de notificaciones o accesibilidad en gris)

En Android 13 y posterior, si instalas un APK descargado desde un navegador o desde GitHub Releases, Android desactiva los interruptores de **Acceso a notificaciones** y **Accesibilidad** hasta que permitas expresamente los ajustes restringidos para esa app:

1. Toca **Permitir** en la fila **Mostrar notificaciones en tu PC** o **Usar el ratón y el teclado del PC** de Panal para que Android vea que intentaste activarlo una vez.
2. Vuelve a **Ajustes › Permisos** de Panal y toca **Información de la app** (o abre **Ajustes de Android › Aplicaciones › Panal**).
3. Toca el **menú de tres puntos** de la esquina superior derecha de la pantalla de información de la app y toca **Permitir ajustes restringidos**.
4. Identifícate con tu huella, tu cara o tu PIN.
5. Vuelve a Panal y toca otra vez **Permitir**. Ahora el interruptor se puede activar con normalidad.

## 4. Problemas con `Elevated` (depuración inalámbrica)

### La notificación del código de emparejamiento de 6 dígitos no aparece o falla

- **Deja el diálogo del código abierto en pantalla**: Android invalida el código de emparejamiento de 6 dígitos de la depuración inalámbrica en cuanto sales del diálogo **Vincular dispositivo con código de vinculación**. Deja ese diálogo visible, baja el panel de notificaciones encima, toca **Introducir código** en la notificación de Panal y escribe allí los 6 dígitos.
- **Comprueba que la Wi-Fi está conectada**: la depuración inalámbrica de Android solo funciona conectada a una red Wi-Fi.
- **Si el teléfono dice "El teléfono olvidó el emparejamiento de Panal"**: si tocaste **Olvidar todo** en Dispositivos vinculados de los ajustes de depuración inalámbrica de Android, abre **Ajustes › Permisos › Control desde tu PC** en Panal y toca **Configurar** para volver a emparejar la depuración inalámbrica.

### `Elevated` se detiene al desactivar las opciones de desarrollador

- Si desactivas el interruptor general de **Opciones de desarrollador** en los Ajustes de Android, Android detiene la depuración inalámbrica y cierra el ayudante de shell `Elevated`. Mantén activadas las **Opciones de desarrollador** si usas funciones `Elevated`. (Cuando reinicias el teléfono o te reconectas a la Wi-Fi con las opciones de desarrollador activadas, Panal vuelve a activar sola la depuración inalámbrica y reinicia `Elevated`.)

### Una app del banco no se abre

Muchas apps bancarias se niegan a funcionar con las **Opciones de desarrollador**, la depuración USB o inalámbrica o un servicio de accesibilidad activos, y `Elevated` y Assist usan todo eso. Panal puede desactivarlos mientras esas apps están abiertas:

1. Abre **Ajustes › Permisos › Control desde tu PC › Pausar para apps bancarias** y toca **Elegir apps**.
2. Elige tus apps bancarias y de pago. Panal necesita haber configurado `Elevated` una vez, y el acceso de uso (toca **Permitir** si lo pide), para saber qué app está abierta.
3. Abre la app del banco como siempre. Panal desactiva las opciones de desarrollador, la depuración y su servicio de accesibilidad mientras está abierta, y vuelve a activar exactamente lo que estaba activado cuando sales. Mientras está en pausa aparece una notificación con **Volver a activar**.

Si una app comprueba antes de que Panal la detecte, toca **Añadir acceso directo** a su lado y ábrela desde ese acceso directo de la pantalla de inicio: Panal se pausa primero y después abre la app.

Algunas apps bancarias tampoco funcionan en un teléfono que haya tenido root alguna vez (en Samsung, con el indicador Knox activado). Panal no puede cambiar eso.

## 5. Problemas con el emparejamiento automático por USB y el transporte `Por USB`

Si al conectar el teléfono al PC con un cable USB no se activa el emparejamiento automático por USB ni la conexión pasa a `Por USB`:

1. **Revisa el cable USB**: usa un cable que transmita datos (algunos cables solo de carga no tienen los hilos de datos).
2. **Activa la depuración USB en el teléfono**: abre **Ajustes de Android › Opciones de desarrollador** y activa **Depuración USB**.
3. **Aprueba el aviso de la clave RSA en el teléfono**: enciende la pantalla del teléfono al conectarlo al PC, marca **Permitir siempre desde este ordenador** y toca **Permitir** en el diálogo **¿Permitir depuración USB?** de Android.

## 6. Problemas con la duplicación de pantalla y el audio

### La duplicación es solo visual (los clics no hacen nada)

- Con el nivel de control `Standard`, Android no permite que las apps inyecten toques ni teclas en otras apps. Para controlar el teléfono con el ratón y el teclado del PC durante la duplicación, activa **`Assist`** (**Ajustes › Permisos › Control desde tu PC › Usar el ratón y el teclado del PC**) o **`Elevated`** (**Toque y teclas reales (depuración inalámbrica)**) en el teléfono.

### La ventana de duplicación se ve en negro

- Si toda la ventana se queda en negro solo mientras una app concreta (como una app bancaria, un gestor de contraseñas o un reproductor de vídeo con DRM) está abierta en el teléfono, esa app activa la marca de ventana `FLAG_SECURE` de Android para bloquear la captura de pantalla. Al volver a la pantalla de inicio del teléfono, la imagen vuelve al momento.

### No hay sonido en el PC durante la duplicación

- Asegúrate de que el teléfono tiene **Android 10 o posterior** (las versiones anteriores no admiten la captura de audio interno).
- Comprueba que el icono del altavoz de la barra de la ventana `mirror` no está silenciado y que el volumen multimedia del teléfono está subido.
- Ten en cuenta que algunas apps de Android bloquean expresamente en su manifiesto el `AudioPlaybackCapture` de terceros (consulta [Limitaciones conocidas](known-limitations.md)).

## 7. Problemas con el teléfono como webcam (`Panal Webcam`)

### `Panal Webcam` no aparece en Zoom, Teams, Meet ni OBS

1. Abre Panal en el PC y ve a **Ajustes › Cámara y audio**.
2. Mira la fila **Complemento de cámara virtual**. Si muestra **Configurar** en lugar de **Lista**, haz clic en **Configurar** y aprueba la petición de administrador de Windows.
3. Reinicia Zoom, Teams, el navegador u OBS para que la aplicación actualice su lista de dispositivos de captura de vídeo de Media Foundation.

### Al hacer clic en "Iniciar webcam" en el PC se queda en "Preguntando..."

- Por diseño, un PC no puede encender la cámara del teléfono en silencio. Mira el teléfono y toca la notificación **`<PC>` quiere usar tu cámara** (o abre Panal en el teléfono y toca **Webcam › Iniciar webcam**).

## 8. Revisar los registros de diagnóstico

Si necesitas informar de un error, abre **Ajustes › Conexión y Acerca de** en el PC y haz clic en **Abrir registros** (junto a la fila de la versión de Panal) para abrir la carpeta local de registros en el Explorador de archivos. Panal nunca guarda en sus registros el texto de las notificaciones, el contenido de los SMS ni el del portapapeles.

Los errores de Panal se comunican en [zaiddia/NectarLink](https://github.com/zaiddia/NectarLink/issues), no en el repositorio de la app original.
