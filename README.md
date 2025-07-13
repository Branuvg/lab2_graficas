# Juego de la Vida de Conway

Implementación del clásico autómata celular "El Juego de la Vida" de John Horton Conway, desarrollado en Rust con la biblioteca raylib-rs.

## Características

[Demo del proyecto](gol gif gb.gif)

### Patrones Incluidos
- **Still Lifes** (formas estáticas):
  - Bloque, Colmena, Pan, Bote, Bañera
- **Osciladores**:
  - Parpadeador, Sapo, Faro
- **Naves espaciales**:
  - Planeador (Glider)
  - LWSS (Light-weight Space Ship)
  - MWSS (Middle-weight Space Ship)
  - HWSS (Heavy-weight Space Ship)
- **Cañones de planeadores**:
  - Cañón de planeadores de Gosper

### Configuración
- **Ventana**: 800x600 píxeles
- **Tamaño del juego**: 100x75 celdas
- **Colores**: 
  - Células vivas: Blanco
  - Células muertas: Negro

### Rama "grande"
La rama `grande` incluye una versión con una pantalla más grande del juego, ideal para experimentar con patrones más grandes o más generaciones.

## Cómo Ejecutar

### Requisitos Previos
1. Instala [Rust](https://www.rust-lang.org/tools/install) si no lo tienes.
2. Asegúrate de tener instaladas las dependencias del sistema para raylib-rs.

### Pasos para Ejecutar
1. Navega al directorio del proyecto:
   ```bash
   cd lab2
   ```

2. Ejecuta el proyecto con Cargo:
   ```bash
   cargo run --release
   ```

   El flag `--release` es importante para un mejor rendimiento, especialmente con patrones complejos.

3. Para probar la versión en la rama `grande`:
   ```bash
   git checkout grande
   cargo run --release
   ```

## Estructura del Código
- `main.rs`: Lógica principal del juego, inicialización de patrones y bucle del juego.
- `framebuffer.rs`: Maneja el buffer de píxeles para el renderizado.
- `line.rs`: Implementación de algoritmos de dibujo de líneas.

## Personalización
Puedes modificar la función `setup_initial_pattern` en `main.rs` para crear tus propios patrones iniciales o experimentar con diferentes configuraciones.

## Reglas del Juego de la Vida
1. **Nacimiento**: Una célula muerta con exactamente 3 vecinas vivas "nace".
2. **Supervivencia**: Una célula viva con 2 o 3 vecinas vivas sobrevive.
3. **Muerte**:
   - Por soledad: Si tiene 1 o menos vecinos vivos.
   - Por sobrepoblación: Si tiene 4 o más vecinos vivos.
