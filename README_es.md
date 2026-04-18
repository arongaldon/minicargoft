# Minicargoft

[**English**](README.md) | [**Español**](README_es.md)

Arón Galdón Ginés, 2026

Minicargoft es un clon básico de Minecraft implementado con el propósito de comparar el rendimiento y la arquitectura de dos ecosistemas distintos: una versión web con **JavaScript** y una versión de escritorio nativa con **Rust**.

Ambas versiones cuentan con las mismas características de un Producto Mínimo Viable (MVP):

- Generación de terreno procesal usando ruido Simplex.
- Cámara en primera persona y movimiento con colisiones.
- Destrucción y colocación de bloques mediante Raycasting.
- Distintos tipos de bloques (Tierra, Hierba, Piedra).

---

## 🎮 Controles

| Acción | Tecla / Botón |
| :--- | :--- |
| Caminar | `W` `A` `S` `D` |
| Esprintar | Mantener `Shift` |
| Saltar | `Espacio` |
| Mirar | Ratón |
| Destruir Bloque | Click Izquierdo |
| Colocar Bloque | Click Derecho |
| Elegir Bloque | Teclas `1`, `2`, `3` |

---

## 🌐 Versión Web (JavaScript / Three.js)

Esta versión se ejecuta directamente en el navegador, ofreciendo una experiencia sin instalaciones previas, ideal para jugar rápido en cualquier dispositivo.

### Cómo ejecutar la versión JS

1. Asegúrate de tener `Node.js` instalado.
2. Entra a la carpeta de JavaScript:

   ```bash
   cd js
   ```

3. Instala las dependencias y corre el servidor de desarrollo:

   ```bash
   npm install
   npm run dev
   ```

4. Abre `http://localhost:5173` en tu navegador.

---

## 🦀 Versión Nativa (Rust / Bevy Engine)

Esta versión compila un ejecutable nativo aprovechando todo el potencial de tu tarjeta gráfica. Escrita en Rust usando el motor ECS de Bevy.

### Cómo ejecutar la versión Rust

1. Asegúrate de tener `Cargo` y `Rust` instalados.
2. Entra a la carpeta de Rust:

   ```bash
   cd rust
   ```

3. Ejecuta el juego en modo optimizado (release) para obtener el máximo rendimiento:

   ```bash
   cargo run --release
   ```

*(La primera vez que lo ejecutes puede tardar un poco en compilar las dependencias del motor Bevy).*
