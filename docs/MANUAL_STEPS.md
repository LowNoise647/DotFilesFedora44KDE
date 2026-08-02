# Pasos manuales (no automatizables)

El instalador automatiza todo lo que es técnicamente posible. Los elementos de
esta lista **requieren intervención manual** por seguridad, por ser datos
personales, o porque dependen de cuentas de terceros.

## 1. JetBrains Toolbox + IDEs

El autostart y los accesos directos de Toolbox/IDEA/Android Studio **no** se
reproducen (referencian rutas de descarga e instalan su propia
configuración).

1. Descarga [JetBrains Toolbox](https://www.jetbrains.com/toolbox-app/).
2. Ejecuta el binario `jetbrains-toolbox`; se añade solo al autostart.
3. Instala IntelliJ IDEA y Android Studio desde Toolbox.
4. `~/.bash_profile` y `~/.profile` ya incluyen (condicionalmente) la ruta
   `~/.local/share/JetBrains/Toolbox/scripts`.

## 2. opencode

`~/.opencode` (con su binario) es una instalación independiente:
`curl -fsSL https://opencode.ai/install | bash` (ver la documentación de
opencode). El `~/.bashrc` ya añade su carpeta `bin` al `PATH` si existe.

## 3. Credenciales y secretos (por diseño, nunca se copian)

| Elemento | Cómo restaurarlo |
|---|---|
| Claves SSH | `ssh-keygen -t ed25519` y añade la pública a tus servicios |
| Claves GPG | Importa desde tu backup seguro / YubiKey |
| KWallet | Introduce tu contraseña al primer uso; restaura `~/.local/share/kwalletd` desde un backup si lo tenías |
| Cuentas (Google, Firefox, etc.) | Inicia sesión de nuevo en cada app |
| Correo (Akonadi) | Configura la cuenta IMAP en KDE PIM; los datos de `~/.local/share/akonadi*` y `~/.config/akonadi*` no se copian |
| Tokens de Git/IDE | Vuelve a autenticarte en cada servicio |

## 4. Aplicaciones Flatpak de terceros

Discord y Spotify requieren iniciar sesión en sus cuentas tras instalarse.

## 5. Configuración de pantallas

`kwinoutputconfig.json` (escala 1.25 en el portátil, monitores eDP-1 +
HDMI-A-1) solo se aplica si detecta los mismos conectores. En otro hardware,
ajusta manualmente en *Preferencias del sistema → Pantallas*.

## 6. Usuario y contraseña del sistema

El instalador asume que ya existe un usuario normal. `useradd`, contraseñas y
sudo son responsabilidad del instalador del sistema operativo.
